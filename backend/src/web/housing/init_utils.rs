use super::{
    models::{CostSymbol, Housing, HousingID, HousingPayload, Review, WeightedHousingRatings},
    schema::{
        columns::{housing, reviews},
        tables,
    },
};
use crate::{
    AppError,
    microservices::meilisearch::{add_items, clear_index},
};
use meilisearch_sdk::{
    client::*,
    settings::{MinWordSizeForTypos, Settings, TypoToleranceSettings},
};
use std::{
    collections::{BTreeSet, HashMap},
    sync::Arc,
};

pub async fn create_all_housing_indexes(
    meili_client: Arc<Client>,
    housing: &[HousingPayload],
) -> Result<(), AppError> {
    meili_client
        .index(tables::HOUSING)
        .set_settings(&init_housing_settings())
        .await?;

    for option in housing.iter() {
        // Search indexes are named by HousingID
        meili_client
            .index(option.id.as_ref())
            .set_settings(&init_review_settings())
            .await?;
    }

    Ok(())
}

pub async fn clear_all_housing_indexes(
    meili_client: Arc<Client>,
    housing: &[HousingPayload],
) -> Result<(), AppError> {
    for option in housing.iter() {
        // Remove index by HousingID
        clear_index(meili_client.clone(), option.id.as_ref()).await?;
    }

    Ok(())
}

pub fn attach_cost_symbols(housing: &mut [HousingPayload]) {
    let indexed_avgs: Vec<(usize, u16)> = housing
        .iter()
        .enumerate()
        .map(|(i, h)| (i, (h.cost_min as u16 + h.cost_max as u16) / 2))
        .collect();

    let unique_avgs: Vec<u16> = BTreeSet::from_iter(indexed_avgs.iter().map(|&(_, avg)| avg))
        .into_iter()
        .collect();

    let low_cutoff = unique_avgs[unique_avgs.len() / 3];
    let high_cutoff = unique_avgs[(2 * unique_avgs.len()) / 3];

    for (i, avg) in indexed_avgs {
        housing[i].cost_symbol = Some(match avg {
            a if a <= low_cutoff => CostSymbol::Cheap,
            a if a <= high_cutoff => CostSymbol::Affordable,
            _ => CostSymbol::Expensive,
        });
    }
}

pub async fn handle_housing_insertion(
    meili_client: Arc<Client>,
    housing: &mut [HousingPayload],
    housing_rating_map: &HashMap<HousingID, WeightedHousingRatings>,
) -> Result<(), AppError> {
    update_housing_ratings(housing, housing_rating_map);

    Ok(add_items(
        meili_client.clone(),
        tables::HOUSING,
        // Housing is converted into using their display name for searching
        &convert_housing(housing),
        housing::ID,
    )
    .await?)
}

pub fn update_housing_ratings(
    housing: &mut [HousingPayload],
    housing_rating_map: &HashMap<HousingID, WeightedHousingRatings>,
) {
    for option in housing.iter_mut() {
        let weighted = housing_rating_map
            .get(&option.id)
            .expect("Misconfigured housing!");
        option.overall_rating = (weighted.weighted_overall.round() as u16).min(500);
        option.review_count = weighted.count;

        option.ratings.living_conditions =
            (weighted.weighted_ratings.living_conditions.round() as u16).min(500);
        option.ratings.location = (weighted.weighted_ratings.location.round() as u16).min(500);
        option.ratings.amenities = (weighted.weighted_ratings.amenities.round() as u16).min(500);
        option.ratings.value = (weighted.weighted_ratings.value.round() as u16).min(500);
        option.ratings.community = (weighted.weighted_ratings.community.round() as u16).min(500);
    }
}

pub async fn handle_reviews_insertion(
    meili_client: Arc<Client>,
    reviews: &[Review],
    housing_rating_map: &mut HashMap<HousingID, WeightedHousingRatings>,
) -> Result<(), AppError> {
    for review in reviews {
        let housing_id_str = &review.housing_id;

        let entry = housing_rating_map
            .entry(
                housing_id_str
                    .parse::<HousingID>()
                    .expect("Review misconfigured!"),
            )
            .or_default();

        let weighted_ratings = &mut entry.weighted_ratings;
        let review_ratings = &review.ratings;

        entry.count += 1;

        entry.weighted_overall +=
            (review.overall_rating as f64 - entry.weighted_overall) / entry.count as f64;

        weighted_ratings.living_conditions += ((review_ratings.living_conditions as f64)
            - weighted_ratings.living_conditions)
            / entry.count as f64;
        weighted_ratings.location +=
            ((review_ratings.location as f64) - weighted_ratings.location) / entry.count as f64;
        weighted_ratings.amenities +=
            ((review_ratings.amenities as f64) - weighted_ratings.amenities) / entry.count as f64;
        weighted_ratings.value +=
            ((review_ratings.value as f64) - weighted_ratings.value) / entry.count as f64;
        weighted_ratings.community +=
            ((review_ratings.community as f64) - weighted_ratings.community) / entry.count as f64;

        add_items(
            meili_client.clone(),
            housing_id_str,
            &[review],
            reviews::REVIEW_ID,
        )
        .await?;
    }

    Ok(())
}

pub fn convert_housing(housing_vec: &[HousingPayload]) -> Vec<Housing> {
    housing_vec
        .iter()
        .map(|payload| Housing {
            // At this point, housing is converted to display name for searchability
            id: payload.id.display_name().to_string(),
            overall_rating: payload.overall_rating,
            ratings: payload.ratings.clone(),
            review_count: payload.review_count,
            housing_type: payload.housing_type.as_ref().to_string(),
            campus_type: payload.campus_type.as_ref().to_string(),
            walk_time_mins: payload.walk_time_mins,
            cost_min: payload.cost_min,
            cost_max: payload.cost_max,
            cost_symbol: payload
                .cost_symbol
                .as_ref()
                .expect("Housing misconfiguration!")
                .as_ref()
                .to_string(),
            address: payload.address.to_string(),
        })
        .collect()
}

pub fn init_housing_settings() -> Settings {
    Settings::new()
        .with_ranking_rules([
            "words",
            "typo",
            "proximity",
            "exactness",
            "attribute",
            "sort",
        ])
        .with_distinct_attribute(Some(housing::ID))
        // User search is matched by HousingLabels NOT HousingID
        // They are converted right before insertion
        .with_searchable_attributes([housing::ID])
        .with_filterable_attributes([
            housing::HOUSING_TYPE,
            housing::CAMPUS_TYPE,
            housing::COST_SYMBOL,
        ])
        .with_sortable_attributes([
            housing::OVERALL_RATING,
            housing::WALK_TIME_MINS,
            housing::COST_MIN,
            housing::COST_MAX,
            housing::REVIEW_COUNT,
        ])
        .with_typo_tolerance(TypoToleranceSettings {
            enabled: Some(true),
            disable_on_attributes: None,
            disable_on_words: None,
            min_word_size_for_typos: Some(MinWordSizeForTypos {
                one_typo: Some(5),
                two_typos: Some(9),
            }),
        })
}

pub fn init_review_settings() -> Settings {
    Settings::new()
        .with_ranking_rules([
            "words",
            "typo",
            "proximity",
            "exactness",
            "attribute",
            "sort",
        ])
        .with_distinct_attribute(Some(reviews::REVIEW_ID))
        .with_searchable_attributes([reviews::DESCRIPTION])
        .with_filterable_attributes([reviews::OVERALL_RATING])
        .with_sortable_attributes([reviews::DATE])
        .with_typo_tolerance(TypoToleranceSettings {
            enabled: Some(true),
            disable_on_attributes: None,
            disable_on_words: None,
            min_word_size_for_typos: Some(MinWordSizeForTypos {
                one_typo: Some(5),
                two_typos: Some(9),
            }),
        })
}
