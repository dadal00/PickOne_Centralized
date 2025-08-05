use super::{
    database::convert_db_reviews,
    init_utils::{
        attach_cost_symbols, clear_all_housing_indexes, create_all_housing_indexes,
        handle_housing_insertion, handle_reviews_insertion,
    },
    models::{HousingID, HousingPayload, ReviewRow, WeightedHousingRatings},
};
use crate::{config::try_load, microservices::database::DatabaseQueries};
use anyhow::Result as anyResult;
use meilisearch_sdk::client::Client;
use scylla::{client::session::Session, response::PagingState};
use std::{collections::HashMap, ops::ControlFlow, sync::Arc};
use tokio::{fs, task::JoinHandle};

pub async fn init_housing(
    database_session: Arc<Session>,
    database_queries: &DatabaseQueries,
    meili_client: Arc<Client>,
) -> anyResult<JoinHandle<anyResult<()>>> {
    let housing_file_path = format!(
        "{}{}",
        try_load::<String>("RUST_CONTAINER_FOLDER_PATH", "/assets").unwrap(),
        try_load::<String>("RUST_HOUSING_FILE", "/housing.json").unwrap()
    );

    let mut housing: Vec<HousingPayload> =
        serde_json::from_str(&fs::read_to_string(housing_file_path).await?)?;

    attach_cost_symbols(&mut housing);

    create_all_housing_indexes(meili_client.clone(), &housing).await?;

    let database_queries_clone = database_queries.clone();

    Ok(tokio::spawn(async move {
        reindex(
            database_session,
            database_queries_clone,
            meili_client,
            &mut housing,
        )
        .await
    }))
}

async fn reindex(
    database_session: Arc<Session>,
    database_queries: DatabaseQueries,
    meili_client: Arc<Client>,
    housing: &mut [HousingPayload],
) -> anyResult<()> {
    let mut paging_state = PagingState::start();

    clear_all_housing_indexes(meili_client.clone(), housing).await?;

    let mut housing_rating_map = HashMap::<HousingID, WeightedHousingRatings>::default();

    loop {
        let (query_result, paging_state_response) = database_session
            .execute_single_page(&database_queries.housing.get_all_reviews, &[], paging_state)
            .await?;

        let row_result = query_result.into_rows_result()?;

        let row_vec: Vec<ReviewRow> = row_result
            .rows::<ReviewRow>()?
            .collect::<Result<Vec<_>, _>>()?;

        let reviews = convert_db_reviews(&row_vec);

        handle_reviews_insertion(meili_client.clone(), &reviews, &mut housing_rating_map).await?;

        match paging_state_response.into_paging_control_flow() {
            ControlFlow::Break(()) => {
                break;
            }
            ControlFlow::Continue(new_paging_state) => paging_state = new_paging_state,
        }
    }

    handle_housing_insertion(meili_client, housing, &housing_rating_map).await?;

    Ok(())
}
