use super::models::{
    HousingID, RatingsBrokenDown, Review, ReviewPayload, ReviewRow, ThumbsDelta, ThumbsDeltaMap,
};
use crate::{
    AppState,
    error::ScyllaError,
    utilities::{convert_i8_to_u8, convert_i16_to_u16, convert_i64_to_u64},
};
use chrono::Utc;
use scylla::{response::PagingState, statement::batch::Batch};
use std::sync::Arc;
use uuid::Uuid;

fn convert_db_ratings(
    rating_living_i16: &i16,
    ratings_location_i16: &i16,
    ratings_amenities_i16: &i16,
    ratings_value_i16: &i16,
    ratings_community_i16: &i16,
) -> RatingsBrokenDown {
    RatingsBrokenDown {
        living_conditions: convert_i16_to_u16(rating_living_i16),
        location: convert_i16_to_u16(ratings_location_i16),
        amenities: convert_i16_to_u16(ratings_amenities_i16),
        value: convert_i16_to_u16(ratings_value_i16),
        community: convert_i16_to_u16(ratings_community_i16),
    }
}

pub fn convert_db_reviews(row_vec: &Vec<ReviewRow>) -> Vec<Review> {
    row_vec
        .iter()
        .map(
            |(
                id,
                housing_id_i8,
                overall_rating_i16,
                rating_living_i16,
                ratings_location_i16,
                ratings_amenities_i16,
                ratings_value_i16,
                ratings_community_i16,
                date,
                description,
                thumbs_up,
                thumbs_down,
            )| Review {
                id: *id,
                housing_id: HousingID::try_from(convert_i8_to_u8(housing_id_i8))
                    .expect("Invalid id!")
                    .as_ref()
                    .to_string(),
                overall_rating: convert_i16_to_u16(overall_rating_i16),
                ratings: convert_db_ratings(
                    rating_living_i16,
                    ratings_location_i16,
                    ratings_amenities_i16,
                    ratings_value_i16,
                    ratings_community_i16,
                ),
                date: date.format("%Y-%m-%d").to_string(),
                description: description.to_string(),
                thumbs_up: convert_i64_to_u64(thumbs_up),
                thumbs_down: convert_i64_to_u64(thumbs_down),
            },
        )
        .collect()
}

pub async fn insert_review(state: Arc<AppState>, review: ReviewPayload) -> Result<(), ScyllaError> {
    let fallback_page_state = PagingState::start();
    let id = Uuid::new_v4();
    let housing_id = review.housing_id;

    state
        .database_session
        .execute_single_page(
            &state.database_queries.housing.insert_review,
            (
                id,
                housing_id.clone() as i8,
                review.overall_rating as i16,
                review.ratings.living_conditions as i16,
                review.ratings.location as i16,
                review.ratings.amenities as i16,
                review.ratings.value as i16,
                review.ratings.community as i16,
                Utc::now().date_naive(),
                review.description,
                0i64,
                0i64,
            ),
            fallback_page_state.clone(),
        )
        .await?;

    state
        .database_session
        .execute_single_page(
            &state.database_queries.housing.insert_housing_id,
            (id, housing_id as i8),
            fallback_page_state,
        )
        .await?;

    Ok(())
}

pub async fn update_thumbs(
    state: Arc<AppState>,
    thumbs_map: ThumbsDeltaMap,
) -> Result<(), ScyllaError> {
    let mut batch: Batch = Default::default();
    let mut batch_values = Vec::new();

    for (uuid, delta) in thumbs_map {
        batch.append_statement(state.database_queries.housing.update_thumbs.clone());

        let (up, down) = match delta {
            ThumbsDelta::Up => (1i64, 0i64),
            ThumbsDelta::Down => (0i64, 1i64),
        };

        batch_values.push((up, down, uuid));
    }

    state.database_session.batch(&batch, &batch_values).await?;

    Ok(())
}

pub async fn get_housing_id(state: Arc<AppState>, review_id: &Uuid) -> Result<String, ScyllaError> {
    let fallback_page_state = PagingState::start();

    let (returned_rows, _) = state
        .database_session
        .execute_single_page(
            &state.database_queries.housing.get_housing_id,
            (review_id,),
            fallback_page_state,
        )
        .await?;

    let (housing_id_i8,) = returned_rows.into_rows_result()?.first_row::<(i8,)>()?;

    Ok(HousingID::try_from(convert_i8_to_u8(&housing_id_i8))
        .expect("Invalid id!")
        .as_ref()
        .to_string())
}

pub async fn delete_housing_id(state: Arc<AppState>, review_id: &Uuid) -> Result<(), ScyllaError> {
    let fallback_page_state = PagingState::start();

    state
        .database_session
        .execute_single_page(
            &state.database_queries.housing.delete_housing_id,
            (review_id,),
            fallback_page_state,
        )
        .await?;

    Ok(())
}
