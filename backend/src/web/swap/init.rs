use super::{
    database::convert_db_items,
    models::ItemRow,
    schema::{columns::items, tables},
};
use crate::{
    AppError,
    microservices::{
        database::DatabaseQueries,
        meilisearch::{add_items, clear_index},
    },
};
use anyhow::Result as anyResult;
use meilisearch_sdk::{
    client::*,
    settings::{MinWordSizeForTypos, Settings, TypoToleranceSettings},
};
use scylla::{client::session::Session, response::PagingState};
use std::{ops::ControlFlow, sync::Arc};
use tokio::task::JoinHandle;

pub async fn init_swap(
    database_session: Arc<Session>,
    database_queries: &DatabaseQueries,
    meili_client: Arc<Client>,
) -> Result<JoinHandle<anyResult<()>>, AppError> {
    meili_client
        .index(tables::ITEMS)
        .set_settings(&init_item_settings())
        .await?;

    let database_queries_clone = database_queries.clone();

    Ok(tokio::spawn(async move {
        reindex(database_session, database_queries_clone, meili_client).await
    }))
}

async fn reindex(
    database_session: Arc<Session>,
    database_queries: DatabaseQueries,
    meili_client: Arc<Client>,
) -> anyResult<()> {
    let mut paging_state = PagingState::start();

    clear_index(meili_client.clone(), tables::ITEMS).await?;

    loop {
        let (query_result, paging_state_response) = database_session
            .execute_single_page(&database_queries.boiler_swap.get_items, &[], paging_state)
            .await?;

        let row_result = query_result.into_rows_result()?;

        let row_vec: Vec<ItemRow> = row_result
            .rows::<ItemRow>()?
            .collect::<Result<Vec<_>, _>>()?;

        add_items(
            meili_client.clone(),
            tables::ITEMS,
            &convert_db_items(&row_vec),
            items::ITEM_ID,
        )
        .await?;

        match paging_state_response.into_paging_control_flow() {
            ControlFlow::Break(()) => {
                break Ok(());
            }
            ControlFlow::Continue(new_paging_state) => paging_state = new_paging_state,
        }
    }
}

fn init_item_settings() -> Settings {
    Settings::new()
        .with_ranking_rules([
            "words",
            "typo",
            "proximity",
            "exactness",
            "attribute",
            "sort",
        ])
        .with_distinct_attribute(Some(items::ITEM_ID))
        .with_searchable_attributes([items::TITLE, items::DESCRIPTION])
        .with_filterable_attributes([items::ITEM_TYPE, items::CONDITION, items::LOCATION])
        .with_typo_tolerance(TypoToleranceSettings {
            enabled: Some(true),
            disable_on_attributes: None,
            disable_on_words: None,
            min_word_size_for_typos: Some(MinWordSizeForTypos {
                one_typo: Some(5),
                two_typos: Some(9),
            }),
        })
}
