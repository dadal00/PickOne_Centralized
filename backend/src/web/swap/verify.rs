use super::models::ItemPayload;
use crate::{error::HttpErrorResponse, web::verify::validate_length};
use rustrict::CensorStr;

pub fn check_item(payload: &ItemPayload) -> Result<(), HttpErrorResponse> {
    validate_item(&payload.title, &payload.description)
        .map_err(|e| HttpErrorResponse::BadRequest(e.to_string()))?;

    Ok(())
}

pub fn validate_item(title: &str, description: &str) -> Result<(), &'static str> {
    validate_item_attribute(title)?;

    validate_item_attribute(description)?;

    Ok(())
}

pub fn validate_item_attribute(payload: &str) -> Result<(), &'static str> {
    if !validate_length(payload) {
        return Err("Too many chars");
    }

    if payload.is_inappropriate() {
        return Err("Inappropriate");
    }

    Ok(())
}
