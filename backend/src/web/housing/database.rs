use super::{
    models::{
        HousingID, RatingsBrokenDown, Review, ReviewPayload, ReviewRow, SemesterSeason,
        ThumbsDelta, ThumbsDeltaMap,
    },
    schema::{KEYSPACE, columns::reviews, tables},
};
use crate::{
    AppError, AppState,
    microservices::database::CREATE_KEYSPACE,
    utilities::{convert_i8_to_u8, convert_i16_to_u16, convert_i64_to_u64},
};
use scylla::{
    client::session::Session,
    response::PagingState,
    statement::{Statement, batch::Batch, prepared::PreparedStatement},
};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct Housing {
    pub get_all_reviews: PreparedStatement,
    pub insert_review: PreparedStatement,
    pub update_thumbs: PreparedStatement,
}

impl Housing {
    pub async fn init(session: &Session) -> Result<Self, AppError> {
        Ok(Self {
            get_all_reviews: session
                .prepare(
                    Statement::new(format!(
                        "SELECT {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {} FROM {}.{}",
                        reviews::REVIEW_ID,
                        reviews::HOUSING_ID,
                        reviews::OVERALL_RATING,
                        reviews::RATINGS_LIVING,
                        reviews::RATINGS_LOCATION,
                        reviews::RATINGS_AMENITIES,
                        reviews::RATINGS_VALUE,
                        reviews::RATINGS_COMMUNITY,
                        reviews::SEMESTER_SEASON,
                        reviews::SEMESTER_YEAR,
                        reviews::DESCRIPTION,
                        reviews::THUMBS_UP,
                        reviews::THUMBS_DOWN,
                        KEYSPACE,
                        tables::REVIEWS
                    ))
                    .with_page_size(100),
                )
                .await?,
            insert_review: session
                .prepare(format!(
                    "INSERT INTO {}.{} ({}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
                    KEYSPACE,
                    tables::REVIEWS,
                    reviews::REVIEW_ID,
                    reviews::HOUSING_ID,
                    reviews::OVERALL_RATING,
                    reviews::RATINGS_LIVING,
                    reviews::RATINGS_LOCATION,
                    reviews::RATINGS_AMENITIES,
                    reviews::RATINGS_VALUE,
                    reviews::RATINGS_COMMUNITY,
                    reviews::SEMESTER_SEASON,
                    reviews::SEMESTER_YEAR,
                    reviews::DESCRIPTION,
                    reviews::THUMBS_UP,
                    reviews::THUMBS_DOWN,
                ))
                .await?,
            update_thumbs: session
                .prepare(format!(
                    "UPDATE {}.{} SET {} = {} + ?, SET {} = {} + ? WHERE {} = ? AND {} = ?",
                    KEYSPACE,
                    tables::REVIEWS,
                    reviews::THUMBS_UP,
                    reviews::THUMBS_UP,
                    reviews::THUMBS_DOWN,
                    reviews::THUMBS_DOWN,
                    reviews::REVIEW_ID,
                    reviews::HOUSING_ID,
                ))
                .await?,
        })
    }
}

pub async fn create_housing_tables(session: &Session) -> Result<(), AppError> {
    session
        .query_unpaged(CREATE_KEYSPACE.replace("__KEYSPACE__", KEYSPACE), &[])
        .await?;

    session
        .query_unpaged(
            format!(
                "CREATE TABLE IF NOT EXISTS {}.{} ({} {}, {} {}, {} {}, {} {}, {} {}, {} {}, {} {}, {} {}, {} {}, {} {}, {} {}, {} {}, {} {}, PRIMARY KEY({}, {})) WITH cdc = {{'enabled': true}}",
                KEYSPACE,
                tables::REVIEWS,
                reviews::REVIEW_ID,
                reviews::REVIEW_ID_TYPE,
                reviews::HOUSING_ID,
                reviews::HOUSING_ID_TYPE,
                reviews::OVERALL_RATING,
                reviews::OVERALL_RATING_TYPE,
                reviews::RATINGS_LIVING,
                reviews::RATINGS_LIVING_TYPE,
                reviews::RATINGS_LOCATION,
                reviews::RATINGS_LOCATION_TYPE,
                reviews::RATINGS_AMENITIES,
                reviews::RATINGS_AMENITIES_TYPE,
                reviews::RATINGS_VALUE,
                reviews::RATINGS_VALUE_TYPE,
                reviews::RATINGS_COMMUNITY,
                reviews::RATINGS_COMMUNITY_TYPE,
                reviews::SEMESTER_SEASON,
                reviews::SEMESTER_SEASON_TYPE,
                reviews::SEMESTER_YEAR,
                reviews::SEMESTER_YEAR_TYPE,
                reviews::DESCRIPTION,
                reviews::DESCRIPTION_TYPE,
                reviews::THUMBS_UP,
                reviews::THUMBS_UP_TYPE,
                reviews::THUMBS_DOWN,
                reviews::THUMBS_DOWN_TYPE,
                reviews::PRIMARY_KEY,
                reviews::CLUSTER_KEY,
            ),
            &[],
        )
        .await?;

    Ok(())
}

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
                semester_season_i8,
                semester_year,
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
                semester_season: SemesterSeason::try_from(convert_i8_to_u8(semester_season_i8))
                    .expect("Invalid semester season!")
                    .as_ref()
                    .to_string(),
                semester_year: convert_i8_to_u8(semester_year),
                description: description.to_string(),
                thumbs_up: convert_i64_to_u64(thumbs_up),
                thumbs_down: convert_i64_to_u64(thumbs_down),
            },
        )
        .collect()
}

pub async fn insert_review(state: Arc<AppState>, review: ReviewPayload) -> Result<(), AppError> {
    let fallback_page_state = PagingState::start();

    state
        .database_session
        .execute_single_page(
            &state.database_queries.housing.insert_review,
            (
                Uuid::new_v4(),
                review.housing_id as i8,
                review.overall_rating as i16,
                review.ratings.living_conditions as i16,
                review.ratings.location as i16,
                review.ratings.amenities as i16,
                review.ratings.value as i16,
                review.ratings.community as i16,
                review.semester_season as i8,
                review.semester_year as i8,
                review.description,
                0i64,
                0i64,
            ),
            fallback_page_state,
        )
        .await?;

    Ok(())
}

pub async fn update_thumbs(
    state: Arc<AppState>,
    thumbs_map: ThumbsDeltaMap,
) -> Result<(), AppError> {
    let mut batch: Batch = Default::default();
    let mut batch_values = Vec::new();

    for (uuid, entry) in thumbs_map {
        batch.append_statement(state.database_queries.housing.update_thumbs.clone());

        let (up, down) = match entry.delta {
            ThumbsDelta::Up => (1i64, 0i64),
            ThumbsDelta::Down => (0i64, 1i64),
        };

        batch_values.push((up, down, uuid, entry.housing_id as i8));
    }

    state.database_session.batch(&batch, &batch_values).await?;

    Ok(())
}
