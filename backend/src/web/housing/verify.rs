use super::models::{RatingsBrokenDown, ReviewPayload};
use crate::{config::try_load, error::HttpErrorResponse};
use once_cell::sync::Lazy;
use rustrict::CensorStr;

static MAX_CHARS: Lazy<usize> = Lazy::new(|| try_load("PUBLIC_HOUSING_MAX_CHARS", "350").unwrap());
static MIN_CHARS: Lazy<usize> = Lazy::new(|| try_load("PUBLIC_HOUSING_MIN_CHARS", "50").unwrap());

pub fn check_review(payload: &ReviewPayload) -> Result<(), HttpErrorResponse> {
    validate_review(payload).map_err(|e| HttpErrorResponse::BadRequest(e.to_string()))?;

    Ok(())
}

fn validate_review(payload: &ReviewPayload) -> Result<(), &'static str> {
    validate_description(&payload.description)?;

    validate_ratings(&payload.overall_rating, &payload.ratings)?;

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
    if payload.len() > *MAX_CHARS || payload.len() < *MIN_CHARS {
        return Err("Char count wrong");
    }

    if payload.is_inappropriate() {
        return Err("Inappropriate");
    }

    Ok(())
}
