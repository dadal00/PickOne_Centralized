use crate::{
    AppState, WebsitePath,
    microservices::meilisearch::{add_items, update_items},
    web::{
        housing::cdc::{convert_cdc_review, convert_cdc_thumbs},
        swap::cdc::{convert_cdc_item, handle_item_deletion},
    },
};
use anyhow::Result as anyResult;
use meilisearch_sdk::client::*;
use scylla_cdc::consumer::CDCRow;
use std::sync::Arc;

pub async fn choose_addition(
    data: &CDCRow<'_>,
    meili_client: Arc<Client>,
    meili_index: &str,
    scylla_id_name: &str,
    website_path: &WebsitePath,
) -> anyResult<()> {
    match website_path {
        WebsitePath::BoilerSwap => {
            add_items(
                meili_client,
                meili_index,
                &[convert_cdc_item(data)],
                scylla_id_name,
            )
            .await
        }
        WebsitePath::Photos => panic!("Photos cdc insertion not implemented"),
        WebsitePath::Home => panic!("Home cdc insertion not implemented"),
        WebsitePath::Housing => {
            let review = convert_cdc_review(data);

            add_items(
                meili_client,
                &review.housing_id.clone(),
                &[review],
                scylla_id_name,
            )
            .await
        }
    }
}

pub async fn choose_deletion(
    data: &CDCRow<'_>,
    state: Arc<AppState>,
    meili_index: &str,
    redis_deletion_name: &Option<&str>,
    scylla_id_name: &str,
    website_path: &WebsitePath,
) -> anyResult<()> {
    match website_path {
        WebsitePath::BoilerSwap => Ok(handle_item_deletion(
            data,
            state,
            meili_index,
            redis_deletion_name.expect("Misconfigured swap cdc!"),
            scylla_id_name,
            website_path.as_ref(),
        )
        .await?),
        WebsitePath::Photos => panic!("Photos cdc deletion not implemented"),
        WebsitePath::Home => panic!("Home cdc deletion not implemented"),
        WebsitePath::Housing => panic!("Housing cdc deletion not implemented"),
    }
}

pub async fn choose_update(
    data: &CDCRow<'_>,
    meili_client: Arc<Client>,
    scylla_id_name: &str,
    website_path: &WebsitePath,
) -> anyResult<()> {
    match website_path {
        WebsitePath::BoilerSwap => panic!("Swap cdc update not implemented"),
        WebsitePath::Photos => panic!("Photos cdc update not implemented"),
        WebsitePath::Home => panic!("Home cdc update not implemented"),
        WebsitePath::Housing => {
            let thumb_payload = convert_cdc_thumbs(data);

            update_items(
                meili_client,
                &thumb_payload.housing_id.clone(),
                &[thumb_payload],
                scylla_id_name,
            )
            .await
        }
    }
}
