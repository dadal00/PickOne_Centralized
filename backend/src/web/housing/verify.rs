use super::models::{RatingsBrokenDown, ReviewPayload, SemesterSeason};
use crate::{AppError, config::try_load};
use chrono::{Datelike, Utc};
use once_cell::sync::Lazy;
use rustrict::CensorStr;

static MAX_CHARS: Lazy<usize> = Lazy::new(|| try_load("PUBLIC_HOUSING_MAX_CHARS", "350").unwrap());

pub fn check_review(payload: &ReviewPayload) -> Result<(), AppError> {
    validate_review(payload).map_err(|e| AppError::BadRequest(e.to_string()))?;

    Ok(())
}

fn validate_review(payload: &ReviewPayload) -> Result<(), &'static str> {
    validate_description(&payload.description)?;

    validate_ratings(&payload.overall_rating, &payload.ratings)?;

    validate_date(&payload.semester_year, &payload.semester_season)?;

    Ok(())
}

fn valid_seasons_for_month(month: &u32) -> Vec<SemesterSeason> {
    match month {
        1..=5 => vec![SemesterSeason::Spring],
        6..=8 => vec![SemesterSeason::Spring, SemesterSeason::Summer],
        9..=12 => vec![
            SemesterSeason::Spring,
            SemesterSeason::Summer,
            SemesterSeason::Fall,
        ],
        _ => vec![],
    }
}

fn validate_date(semester_year: &u8, semester_season: &SemesterSeason) -> Result<(), &'static str> {
    let now = Utc::now();
    let current_year = (now.year() % 100) as u8;
    let current_month = now.month();

    if *semester_year > current_year {
        return Err("Year is in the future");
    }

    if *semester_year < 20 {
        return Err("Year is too old");
    }

    if *semester_year == current_year
        && !valid_seasons_for_month(&current_month).contains(semester_season)
    {
        return Err("Invalid semester season");
    }

    Ok(())
}

fn validate_ratings(overall_rating: &u16, ratings: &RatingsBrokenDown) -> Result<(), &'static str> {
    fn is_valid(value: u16) -> bool {
        matches!(value, 100 | 200 | 300 | 400 | 500)
    }

    if !is_valid(*overall_rating) {
        return Err("Invalid overall rating");
    }

    if !is_valid(ratings.living_conditions)
        || !is_valid(ratings.location)
        || !is_valid(ratings.amenities)
        || !is_valid(ratings.value)
        || !is_valid(ratings.community)
    {
        return Err("Invalid sub-rating");
    }

    Ok(())
}

fn validate_description(payload: &str) -> Result<(), &'static str> {
    if !payload.len() < *MAX_CHARS {
        return Err("Too many chars");
    }

    if payload.is_inappropriate() {
        return Err("Inappropriate");
    }

    Ok(())
}
