use super::{
    models::{HousingID, RatingsBrokenDown, Review, ReviewRow, SemesterSeason},
    schema::{KEYSPACE, columns::reviews, tables},
};
use crate::{
    AppError,
    microservices::database::CREATE_KEYSPACE,
    utilities::{convert_i8_to_u8, convert_i16_to_u16, convert_i32_to_u32},
};
use scylla::{
    client::session::Session,
    statement::{Statement, prepared::PreparedStatement},
};

#[derive(Clone)]
pub struct Housing {
    pub get_all_reviews: PreparedStatement,
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
                "CREATE TABLE IF NOT EXISTS {}.{} ({} {}, {} {}, {} {}, {} {}, {} {}, {} {}, {} {}, {} {}, {} {}, {} {}, {} {}, {} {}, {} {}, PRIMARY KEY({}))",
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
                thumbs_up: convert_i32_to_u32(thumbs_up),
                thumbs_down: convert_i32_to_u32(thumbs_down),
            },
        )
        .collect()
}
