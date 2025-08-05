use crate::{
    AppState, WebsitePath,
    web::{
        housing::cdc::{handle_review_deletion, handle_review_insertion, handle_review_updates},
        swap::cdc::{handle_item_deletion, handle_item_insertion},
    },
};
use anyhow::Result as anyResult;
use meilisearch_sdk::client::*;
use scylla_cdc::consumer::CDCRow;
use std::sync::Arc;

pub async fn choose_addition(
    data: &CDCRow<'_>,
    meili_client: Arc<Client>,
    website_path: &WebsitePath,
) -> anyResult<()> {
    match website_path {
        WebsitePath::BoilerSwap => handle_item_insertion(data, meili_client).await,
        WebsitePath::Photos => panic!("Photos cdc insertion not implemented"),
        WebsitePath::Home => panic!("Home cdc insertion not implemented"),
        WebsitePath::Housing => handle_review_insertion(data, meili_client).await,
    }
}

pub async fn choose_deletion(
    data: &CDCRow<'_>,
    state: Arc<AppState>,
    website_path: &WebsitePath,
) -> anyResult<()> {
    match website_path {
        WebsitePath::BoilerSwap => handle_item_deletion(data, state).await,
        WebsitePath::Photos => panic!("Photos cdc deletion not implemented"),
        WebsitePath::Home => panic!("Home cdc deletion not implemented"),
        WebsitePath::Housing => handle_review_deletion(data, state).await,
    }
}

pub async fn choose_update(
    data: &CDCRow<'_>,
    state: Arc<AppState>,
    website_path: &WebsitePath,
) -> anyResult<()> {
    match website_path {
        WebsitePath::BoilerSwap => panic!("Swap cdc update not implemented"),
        WebsitePath::Photos => panic!("Photos cdc update not implemented"),
        WebsitePath::Home => panic!("Home cdc update not implemented"),
        WebsitePath::Housing => handle_review_updates(data, state).await,
    }
}
