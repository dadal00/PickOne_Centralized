use super::{
    models::{HousingID, RatingsBrokenDown, Review, SemesterSeason, ThumbsPayload},
    schema::{KEYSPACE, columns::reviews, tables},
};
use crate::{
    AppError, AppState, ScyllaCDCParams, WebsitePath,
    microservices::cdc::utilities::{
        get_cdc_id, get_cdc_text, get_cdc_u8, get_cdc_u16, get_cdc_u64,
    },
    start_cdc,
};
use anyhow::Error as anyhowError;
use futures_util::future::RemoteHandle;
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
        semester_season: SemesterSeason::try_from(get_cdc_u8(data, reviews::SEMESTER_SEASON))
            .expect("Invalid season!")
            .as_ref()
            .to_string(),
        semester_year: get_cdc_u8(data, reviews::SEMESTER_YEAR),
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
        housing_id: HousingID::try_from(get_cdc_u8(data, reviews::HOUSING_ID))
            .expect("Invalid id!")
            .as_ref()
            .to_string(),
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
            id_name: reviews::REVIEW_ID.to_string(),
        },
        WebsitePath::Housing,
        None,
        tables::REVIEWS_CDC,
    )
    .await
}
