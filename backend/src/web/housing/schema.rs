pub const KEYSPACE: &str = "housing";

pub mod tables {
    pub const HOUSING: &str = "housing";
    pub const REVIEWS: &str = "reviews";
    pub const REVIEWS_CDC: &str = "reviews_cdc";
}

pub mod columns {
    pub mod housing {
        pub const ID: &str = "id";
        pub const _OVERALL_RATING: &str = "overall_rating";
        pub const _RATINGS: &str = "ratings";
        pub const _REVIEW_COUNT: &str = "review_count";
        pub const HOUSING_TYPE: &str = "housing_type";
        pub const CAMPUS_TYPE: &str = "campus_type";
        pub const _WALK_TIME_MINS: &str = "walk_time_mins";
        pub const _COST_MIN: &str = "cost_min";
        pub const _COST_MAX: &str = "cost_max";
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

        pub const SEMESTER_SEASON: &str = "semester_season";
        pub const SEMESTER_SEASON_TYPE: &str = "tinyint";

        pub const SEMESTER_YEAR: &str = "semester_year";
        pub const SEMESTER_YEAR_TYPE: &str = "tinyint";

        pub const DESCRIPTION: &str = "description";
        pub const DESCRIPTION_TYPE: &str = "text";

        pub const THUMBS_UP: &str = "thumbs_up";
        pub const THUMBS_UP_TYPE: &str = "counter";

        pub const THUMBS_DOWN: &str = "thumbs_down";
        pub const THUMBS_DOWN_TYPE: &str = "counter";

        pub const PRIMARY_KEY: &str = REVIEW_ID;
        pub const CLUSTER_KEY: &str = HOUSING_ID;
    }
}
