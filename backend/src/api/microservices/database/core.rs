use crate::{AppError, AppState, api::models::RedisAccount};
use scylla::response::{PagingState, query_result::FirstRowError::RowsEmpty};
use std::sync::Arc;

pub async fn get_user(
    state: Arc<AppState>,
    email: &str,
) -> Result<Option<(String, bool)>, AppError> {
    let fallback_page_state = PagingState::start();

    let (returned_rows, _) = state
        .database_session
        .execute_single_page(
            &state.database_queries.boiler_swap.get_user,
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

pub async fn insert_user(state: Arc<AppState>, account: &RedisAccount) -> Result<(), AppError> {
    let fallback_page_state = PagingState::start();

    state
        .database_session
        .execute_single_page(
            &state.database_queries.boiler_swap.insert_user,
            (account.email.clone(), account.password_hash.clone(), false),
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
            &state.database_queries.boiler_swap.check_lock,
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
            &state.database_queries.boiler_swap.update_lock,
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
            &state.database_queries.boiler_swap.unlock_account,
            (password_hash, email),
            fallback_page_state,
        )
        .await?;

    Ok(())
}
