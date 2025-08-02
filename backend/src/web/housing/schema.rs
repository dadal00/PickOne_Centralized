use super::schema::columns::reviews;
use crate::{error::ScyllaError, microservices::database::CREATE_KEYSPACE};
use scylla::{
    client::session::Session,
    statement::{Statement, prepared::PreparedStatement},
};

pub const KEYSPACE: &str = "housing";

pub mod tables {
    pub const HOUSING: &str = "housing";
    pub const REVIEWS: &str = "reviews";
    pub const REVIEWS_CDC: &str = "reviews_cdc";
    pub const REVIEWS_HOUSING_ID: &str = "reviews_housing_id";
}

pub mod columns {
    pub mod housing {
        pub const ID: &str = "id";
        pub const OVERALL_RATING: &str = "overall_rating";
        pub const _RATINGS: &str = "ratings";
        pub const REVIEW_COUNT: &str = "review_count";
        pub const HOUSING_TYPE: &str = "housing_type";
        pub const CAMPUS_TYPE: &str = "campus_type";
        pub const WALK_TIME_MINS: &str = "walk_time_mins";
        pub const COST_MIN: &str = "cost_min";
        pub const COST_MAX: &str = "cost_max";
        pub const COST_SYMBOL: &str = "cost_symbol";
        pub const _ADDRESS: &str = "address";
    }

    pub mod reviews {
        pub const REVIEW_ID: &str = "id";
        pub const REVIEW_ID_TYPE: &str = "uuid";

        pub const HOUSING_ID: &str = "housing_id";
        pub const HOUSING_ID_TYPE: &str = "tinyint";

        pub const OVERALL_RATING: &str = "overall_rating";
        pub const OVERALL_RATING_TYPE: &str = "smallint";

        // Ratings broken down

        pub const RATINGS_LIVING: &str = "ratings_living";
        pub const RATINGS_LIVING_TYPE: &str = "smallint";

        pub const RATINGS_LOCATION: &str = "ratings_location";
        pub const RATINGS_LOCATION_TYPE: &str = "smallint";

        pub const RATINGS_AMENITIES: &str = "ratings_amenities";
        pub const RATINGS_AMENITIES_TYPE: &str = "smallint";

        pub const RATINGS_VALUE: &str = "ratings_value";
        pub const RATINGS_VALUE_TYPE: &str = "smallint";

        pub const RATINGS_COMMUNITY: &str = "ratings_community";
        pub const RATINGS_COMMUNITY_TYPE: &str = "smallint";

        // End of Ratings

        pub const DATE: &str = "date";
        pub const DATE_TYPE: &str = "date";

        pub const DESCRIPTION: &str = "description";
        pub const DESCRIPTION_TYPE: &str = "text";

        pub const THUMBS_UP: &str = "thumbs_up";
        pub const THUMBS_UP_TYPE: &str = "counter";

        pub const THUMBS_DOWN: &str = "thumbs_down";
        pub const THUMBS_DOWN_TYPE: &str = "counter";

        pub const PRIMARY_KEY: &str = REVIEW_ID;
    }
}

#[derive(Clone)]
pub struct Housing {
    pub get_all_reviews: PreparedStatement,
    pub insert_review: PreparedStatement,
    pub update_thumbs: PreparedStatement,
    pub get_housing_id: PreparedStatement,
    pub insert_housing_id: PreparedStatement,
    pub delete_housing_id: PreparedStatement,
}

impl Housing {
    pub async fn init(session: &Session) -> Result<Self, ScyllaError> {
        Ok(Self {
            get_all_reviews: session
                .prepare(
                    Statement::new(format!(
                        "SELECT {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {} FROM {}.{}",
                        reviews::REVIEW_ID,
                        reviews::HOUSING_ID,
                        reviews::OVERALL_RATING,
                        reviews::RATINGS_LIVING,
                        reviews::RATINGS_LOCATION,
                        reviews::RATINGS_AMENITIES,
                        reviews::RATINGS_VALUE,
                        reviews::RATINGS_COMMUNITY,
                        reviews::DATE,
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
                    "INSERT INTO {}.{} ({}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
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
                    reviews::DATE,
                    reviews::DESCRIPTION,
                    reviews::THUMBS_UP,
                    reviews::THUMBS_DOWN,
                ))
                .await?,
            insert_housing_id: session
                .prepare(format!(
                    "INSERT INTO {}.{} ({}, {}) VALUES (?, ?)",
                    KEYSPACE,
                    tables::REVIEWS_HOUSING_ID,
                    reviews::REVIEW_ID,
                    reviews::HOUSING_ID,
                ))
                .await?,
            update_thumbs: session
                .prepare(format!(
                    "UPDATE {}.{} SET {} = {} + ?, SET {} = {} + ? WHERE {} = ?",
                    KEYSPACE,
                    tables::REVIEWS,
                    reviews::THUMBS_UP,
                    reviews::THUMBS_UP,
                    reviews::THUMBS_DOWN,
                    reviews::THUMBS_DOWN,
                    reviews::REVIEW_ID,
                ))
                .await?,
            get_housing_id: session
                .prepare(format!(
                    "SELECT {} FROM {}.{} WHERE {} = ?",
                    reviews::HOUSING_ID,
                    KEYSPACE,
                    tables::REVIEWS_HOUSING_ID,
                    reviews::REVIEW_ID,
                ))
                .await?,
            delete_housing_id: session
                .prepare(format!(
                    "DELETE FROM {}.{} WHERE {} = ?",
                    KEYSPACE,
                    tables::REVIEWS_HOUSING_ID,
                    reviews::REVIEW_ID,
                ))
                .await?,
        })
    }
}

pub async fn create_housing_tables(session: &Session) -> Result<(), ScyllaError> {
    session
        .query_unpaged(CREATE_KEYSPACE.replace("__KEYSPACE__", KEYSPACE), &[])
        .await?;

    session
        .query_unpaged(
            format!(
                "CREATE TABLE IF NOT EXISTS {}.{} ({} {}, {} {}, {} {}, {} {}, {} {}, {} {}, {} {}, {} {}, {} {}, {} {}, {} {}, {} {}, PRIMARY KEY({})) WITH cdc = {{'enabled': true}}",
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
                reviews::DATE,
                reviews::DATE_TYPE,
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

    session
        .query_unpaged(
            format!(
                "CREATE TABLE IF NOT EXISTS {}.{} ({} {}, {} {}, PRIMARY KEY({}))",
                KEYSPACE,
                tables::REVIEWS_HOUSING_ID,
                reviews::REVIEW_ID,
                reviews::REVIEW_ID_TYPE,
                reviews::HOUSING_ID,
                reviews::HOUSING_ID_TYPE,
                reviews::PRIMARY_KEY,
            ),
            &[],
        )
        .await?;

    Ok(())
}
