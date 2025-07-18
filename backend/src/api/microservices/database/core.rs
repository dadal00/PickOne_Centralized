use crate::{
    AppError, AppState,
    api::{
        models::{Condition, Emoji, Item, ItemPayload, ItemRow, ItemType, Location, RedisAccount},
        utilities::convert_i8_to_u8,
    },
};
use chrono::{Duration as chronoDuration, Utc};
use scylla::response::{PagingState, query_result::FirstRowError::RowsEmpty};
use std::sync::Arc;
use uuid::Uuid;

pub async fn get_user(
    state: Arc<AppState>,
    email: &str,
) -> Result<Option<(String, bool)>, AppError> {
    let fallback_page_state = PagingState::start();

    let (returned_rows, _) = state
        .database_session
        .execute_single_page(
            &state.database_queries.get_user,
            (email,),
            fallback_page_state,
        )
        .await?;

    match returned_rows
        .into_rows_result()?
        .first_row::<(String, bool)>()
    {
        Ok((password_hash, locked)) => Ok(Some((password_hash, locked))),
        Err(RowsEmpty) => Ok(None),
        Err(e) => Err(e.into()),
    }
}

pub async fn insert_user(state: Arc<AppState>, account: RedisAccount) -> Result<(), AppError> {
    let fallback_page_state = PagingState::start();

    state
        .database_session
        .execute_single_page(
            &state.database_queries.insert_user,
            (account.email, account.password_hash, false),
            fallback_page_state,
        )
        .await?;

    Ok(())
}

pub async fn check_lock(state: Arc<AppState>, email: &str) -> Result<Option<bool>, AppError> {
    let fallback_page_state = PagingState::start();

    let (returned_rows, _) = state
        .database_session
        .execute_single_page(
            &state.database_queries.check_lock,
            (email,),
            fallback_page_state,
        )
        .await?;

    match returned_rows.into_rows_result()?.first_row::<(bool,)>() {
        Ok((locked,)) => Ok(Some(locked)),
        Err(RowsEmpty) => Ok(None),
        Err(e) => Err(e.into()),
    }
}

pub async fn update_lock(state: Arc<AppState>, email: &str, lock: bool) -> Result<(), AppError> {
    let fallback_page_state = PagingState::start();

    state
        .database_session
        .execute_single_page(
            &state.database_queries.update_lock,
            (lock, email),
            fallback_page_state,
        )
        .await?;

    Ok(())
}

pub async fn unlock_account(
    state: Arc<AppState>,
    email: &str,
    password_hash: &str,
) -> Result<(), AppError> {
    let fallback_page_state = PagingState::start();

    state
        .database_session
        .execute_single_page(
            &state.database_queries.unlock_account,
            (password_hash, email),
            fallback_page_state,
        )
        .await?;

    Ok(())
}

pub async fn insert_item(state: Arc<AppState>, item: ItemPayload) -> Result<Uuid, AppError> {
    let fallback_page_state = PagingState::start();
    let id = Uuid::new_v4();

    state
        .database_session
        .execute_single_page(
            &state.database_queries.insert_item,
            (
                &id,
                item.item_type as i8,
                item.title,
                item.condition as i8,
                item.location as i8,
                item.description,
                item.emoji as i8,
                Utc::now().date_naive() + chronoDuration::days(7),
                604800 * 3,
            ),
            fallback_page_state,
        )
        .await?;

    Ok(id)
}

pub fn convert_db_items(row_vec: &Vec<ItemRow>) -> Vec<Item> {
    row_vec
        .iter()
        .map(
            |(
                id,
                item_type_i8,
                title,
                condition_i8,
                location_i8,
                description,
                emoji_i8,
                expiration_date,
            )| Item {
                item_id: *id,
                item_type: ItemType::try_from(convert_i8_to_u8(item_type_i8))
                    .unwrap_or(ItemType::Other)
                    .as_ref()
                    .to_string(),
                title: title.to_string(),
                condition: Condition::try_from(convert_i8_to_u8(condition_i8))
                    .unwrap_or(Condition::Fair)
                    .as_ref()
                    .to_string(),
                location: Location::try_from(convert_i8_to_u8(location_i8))
                    .unwrap_or(Location::CaryQuadEast)
                    .as_ref()
                    .to_string(),
                description: description.to_string(),
                emoji: Emoji::try_from(convert_i8_to_u8(emoji_i8))
                    .unwrap_or(Emoji::Books)
                    .as_ref()
                    .to_string(),
                expiration_date: expiration_date.format("%Y-%m-%d").to_string(),
            },
        )
        .collect()
}
