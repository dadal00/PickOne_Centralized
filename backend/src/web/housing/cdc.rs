use super::{
    database::{delete_housing_id, get_housing_id},
    models::{HousingID, RatingsBrokenDown, Review, ThumbsPayload},
    schema::{KEYSPACE, columns::reviews, tables},
};
use crate::{
    AppError, AppState, ScyllaCDCParams, WebsitePath,
    microservices::{
        cdc::utilities::{
            get_cdc_date, get_cdc_id, get_cdc_text, get_cdc_u8, get_cdc_u16, get_cdc_u64,
        },
        meilisearch::{add_items, delete_item, update_items},
    },
    start_cdc,
};
use anyhow::{Error as anyhowError, Result as anyResult};
use futures_util::future::RemoteHandle;
use meilisearch_sdk::client::Client;
use scylla_cdc::{consumer::CDCRow, log_reader::CDCLogReader};
use std::sync::Arc;

pub fn convert_cdc_review(data: &CDCRow<'_>) -> Review {
    Review {
        id: get_cdc_id(data, reviews::REVIEW_ID),
        housing_id: HousingID::try_from(get_cdc_u8(data, reviews::HOUSING_ID))
            .expect("Invalid id!")
            .as_ref()
            .to_string(),
        overall_rating: get_cdc_u16(data, reviews::OVERALL_RATING),
        ratings: convert_cdc_ratings(data),
        date: get_cdc_date(data, reviews::DATE),
        description: get_cdc_text(data, reviews::DESCRIPTION),
        thumbs_up: get_cdc_u64(data, reviews::THUMBS_UP),
        thumbs_down: get_cdc_u64(data, reviews::THUMBS_DOWN),
    }
}

fn convert_cdc_ratings(data: &CDCRow<'_>) -> RatingsBrokenDown {
    RatingsBrokenDown {
        living_conditions: get_cdc_u16(data, reviews::RATINGS_LIVING),
        location: get_cdc_u16(data, reviews::RATINGS_LOCATION),
        amenities: get_cdc_u16(data, reviews::RATINGS_AMENITIES),
        value: get_cdc_u16(data, reviews::RATINGS_VALUE),
        community: get_cdc_u16(data, reviews::RATINGS_COMMUNITY),
    }
}

pub fn convert_cdc_thumbs(data: &CDCRow<'_>) -> ThumbsPayload {
    ThumbsPayload {
        id: get_cdc_id(data, reviews::REVIEW_ID),
        thumbs_up: get_cdc_u64(data, reviews::THUMBS_UP),
        thumbs_down: get_cdc_u64(data, reviews::THUMBS_DOWN),
    }
}

pub async fn start_housing_cdc(
    state: Arc<AppState>,
) -> Result<(CDCLogReader, RemoteHandle<Result<(), anyhowError>>), AppError> {
    start_cdc(
        state.clone(),
        ScyllaCDCParams {
            keyspace: KEYSPACE.to_string(),
            table: tables::REVIEWS.to_string(),
        },
        WebsitePath::Housing,
        tables::REVIEWS_CDC,
    )
    .await
}

pub async fn handle_review_updates(data: &CDCRow<'_>, state: Arc<AppState>) -> anyResult<()> {
    let thumbs = convert_cdc_thumbs(data);

    update_items(
        state.meili_client.clone(),
        &get_housing_id(state.clone(), &thumbs.id).await?,
        &[thumbs.clone()],
        reviews::REVIEW_ID,
    )
    .await
}

pub async fn handle_review_deletion(data: &CDCRow<'_>, state: Arc<AppState>) -> anyResult<()> {
    let id = get_cdc_id(data, reviews::REVIEW_ID);

    delete_item(
        state.meili_client.clone(),
        &get_housing_id(state.clone(), &id).await?,
        id,
    )
    .await?;

    delete_housing_id(state, &id).await?;

    Ok(())
}

pub async fn handle_review_insertion(
    data: &CDCRow<'_>,
    meili_client: Arc<Client>,
) -> anyResult<()> {
    let review = convert_cdc_review(data);

    add_items(
        meili_client,
        &review.housing_id.clone(),
        &[review],
        reviews::REVIEW_ID,
    )
    .await
}
