use super::{
    models::{Condition, Emoji, Item, ItemPayload, ItemRow, ItemType, Location},
    schema::{
        KEYSPACE,
        columns::{items, users},
        tables,
    },
};
use crate::{
    AppState,
    error::ScyllaError,
    microservices::database::CREATE_KEYSPACE,
    utilities::convert_i8_to_u8,
    web::models::{RedisAccount, WebsitePath},
};
use chrono::{Duration as chronoDuration, Utc};
use scylla::{
    client::session::Session,
    response::{PagingState, query_result::FirstRowError::RowsEmpty},
    statement::{Statement, prepared::PreparedStatement},
};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct BoilerSwap {
    pub get_user: PreparedStatement,
    pub insert_user: PreparedStatement,
    pub check_lock: PreparedStatement,
    pub update_lock: PreparedStatement,
    pub unlock_account: PreparedStatement,
    pub insert_item: PreparedStatement,
    pub get_items: PreparedStatement,
    pub delete_item: PreparedStatement,
    pub get_cron_items: PreparedStatement,
    pub get_email: PreparedStatement,
    pub insert_email: PreparedStatement,
    pub delete_email: PreparedStatement,
}

impl BoilerSwap {
    pub async fn init(session: &Session) -> Result<Self, ScyllaError> {
        Ok(Self {
        get_user: session
            .prepare(format!(
                "SELECT {}, {} FROM {}.{} WHERE {} = ?",
                users::PASSWORD_HASH,
                users::LOCKED,
                KEYSPACE,
                tables::USERS,
                users::PRIMARY_KEY
            ))
            .await?,
        insert_user: session
            .prepare(format!(
                "INSERT INTO {}.{} ({}, {}, {}) VALUES (?, ?, ?) USING TTL {}",
                KEYSPACE,
                tables::USERS,
                users::EMAIL,
                users::PASSWORD_HASH,
                users::LOCKED,
                users::TTL
            ))
            .await?,
        check_lock: session
            .prepare(format!(
                "SELECT {} FROM {}.{} WHERE {} = ?",
                users::LOCKED,
                KEYSPACE,
                tables::USERS,
                users::PRIMARY_KEY
            ))
            .await?,
        update_lock: session
            .prepare(format!(
                "UPDATE {}.{} SET {} = ? WHERE {} = ?",
                KEYSPACE,
                tables::USERS,
                users::LOCKED,
                users::PRIMARY_KEY
            ))
            .await?,
        unlock_account: session
            .prepare(format!(
                "UPDATE {}.{} SET {} = false, {} = ? WHERE {} = ?",
                KEYSPACE,
                tables::USERS,
                users::LOCKED,
                users::PASSWORD_HASH,
                users::PRIMARY_KEY
            ))
            .await?,
        insert_item: session
            .prepare(format!(
                "INSERT INTO {}.{} ({}, {}, {}, {}, {}, {}, {}, {}) VALUES (?, ?, ?, ?, ?, ?, ?, ?) USING TTL {}",
                KEYSPACE,
                tables::ITEMS,
                items::ITEM_ID,
                items::ITEM_TYPE,
                items::TITLE,
                items::CONDITION,
                items::LOCATION,
                items::DESCRIPTION,
                items::EMOJI,
                items::EXPIRATION_DATE,
                items::TTL,
            ))
            .await?,
        get_items: session
            .prepare(
                Statement::new(format!(
                    "SELECT {}, {}, {}, {}, {}, {}, {}, {} FROM {}.{}", 
                    items::ITEM_ID,
                    items::ITEM_TYPE,
                    items::TITLE,
                    items::CONDITION,
                    items::LOCATION,
                    items::DESCRIPTION,
                    items::EMOJI,
                    items::EXPIRATION_DATE,
                    KEYSPACE,
                    tables::ITEMS
                )).with_page_size(100),
            )
            .await?,
        get_cron_items: session
            .prepare(
                Statement::new(format!(
                    "SELECT {}, {} FROM {}.{}", 
                    items::ITEM_ID,
                    items::EXPIRATION_DATE,
                    KEYSPACE,
                    tables::ITEMS
                )).with_page_size(100),
            )
            .await?,
        delete_item: session
            .prepare(format!(
                "DELETE FROM {}.{} WHERE {} = ?",
                KEYSPACE,
                tables::ITEMS,
                items::ITEM_ID,
            ))
            .await?,
        get_email: session
            .prepare(format!(
                "SELECT {} FROM {}.{} WHERE {} = ?",
                users::EMAIL,
                KEYSPACE,
                tables::ITEMS_EMAIL,
                items::PRIMARY_KEY,
            ))
            .await?,
        insert_email: session
            .prepare(format!(
                "INSERT INTO {}.{} ({}, {}) VALUES (?, ?) USING TTL {}",
                KEYSPACE,
                tables::ITEMS_EMAIL,
                items::ITEM_ID,
                users::EMAIL,
                items::SAFE_TTL,
            ))
            .await?,
        delete_email: session
            .prepare(format!(
                "DELETE FROM {}.{} WHERE {} = ?",
                KEYSPACE,
                tables::ITEMS_EMAIL,
                items::ITEM_ID,
            ))
            .await?,
        })
    }
}

pub async fn create_swap_tables(session: &Session) -> Result<(), ScyllaError> {
    session
        .query_unpaged(CREATE_KEYSPACE.replace("__KEYSPACE__", KEYSPACE), &[])
        .await?;

    session
        .query_unpaged(
            format!(
                "CREATE TABLE IF NOT EXISTS {}.{} (
            {} {},
            {} {},
            {} {},
            PRIMARY KEY({})
        )",
                KEYSPACE,
                tables::USERS,
                users::EMAIL,
                users::EMAIL_TYPE,
                users::PASSWORD_HASH,
                users::PASSWORD_HASH_TYPE,
                users::LOCKED,
                users::LOCKED_TYPE,
                users::PRIMARY_KEY,
            ),
            &[],
        )
        .await?;

    session
        .query_unpaged(
            format!(
                "CREATE TABLE IF NOT EXISTS {}.{} (
            {} {},
            {} {},
            {} {},
            {} {},
            {} {},
            {} {},
            {} {},
            {} {},
            PRIMARY KEY({})
        ) WITH cdc = {{'enabled': true}}",
                KEYSPACE,
                tables::ITEMS,
                items::ITEM_ID,
                items::ITEM_ID_TYPE,
                items::ITEM_TYPE,
                items::ITEM_TYPE_TYPE,
                items::TITLE,
                items::TITLE_TYPE,
                items::CONDITION,
                items::CONDITION_TYPE,
                items::LOCATION,
                items::LOCATION_TYPE,
                items::DESCRIPTION,
                items::DESCRIPTION_TYPE,
                items::EMOJI,
                items::EMOJI_TYPE,
                items::EXPIRATION_DATE,
                items::EXPIRATION_DATE_TYPE,
                items::PRIMARY_KEY,
            ),
            &[],
        )
        .await?;

    session
        .query_unpaged(
            format!(
                "CREATE TABLE IF NOT EXISTS {}.{} ({} {}, {} {}, PRIMARY KEY({}))",
                KEYSPACE,
                tables::ITEMS_EMAIL,
                items::ITEM_ID,
                items::ITEM_ID_TYPE,
                users::EMAIL,
                users::EMAIL_TYPE,
                items::PRIMARY_KEY,
            ),
            &[],
        )
        .await?;

    Ok(())
}

pub async fn insert_item(state: Arc<AppState>, item: ItemPayload) -> Result<Uuid, ScyllaError> {
    let fallback_page_state = PagingState::start();
    let id = Uuid::new_v4();

    state
        .database_session
        .execute_single_page(
            &state.database_queries.boiler_swap.insert_item,
            (
                &id,
                item.item_type as i8,
                item.title,
                item.condition as i8,
                item.location as i8,
                item.description,
                item.emoji as i8,
                Utc::now().date_naive() + chronoDuration::days(7),
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

pub async fn get_user(
    state: Arc<AppState>,
    email: &str,
    website_path: &WebsitePath,
) -> Result<Option<(String, bool)>, ScyllaError> {
    is_this_for_swap(website_path);

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

pub async fn insert_user(
    state: Arc<AppState>,
    account: &RedisAccount,
    website_path: &WebsitePath,
) -> Result<(), ScyllaError> {
    is_this_for_swap(website_path);

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

fn is_this_for_swap(website_path: &WebsitePath) {
    match website_path {
        WebsitePath::BoilerSwap => {}
        WebsitePath::Photos => panic!("Photos not implemented"),
        WebsitePath::Home => panic!("Home not implemented"),
        WebsitePath::Housing => panic!("Housing not implemented"),
    }
}

pub async fn check_lock(
    state: Arc<AppState>,
    email: &str,
    website_path: &WebsitePath,
) -> Result<Option<bool>, ScyllaError> {
    is_this_for_swap(website_path);

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

pub async fn update_lock(
    state: Arc<AppState>,
    email: &str,
    lock: bool,
    website_path: &WebsitePath,
) -> Result<(), ScyllaError> {
    is_this_for_swap(website_path);

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
    website_path: &WebsitePath,
) -> Result<(), ScyllaError> {
    is_this_for_swap(website_path);

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

pub async fn insert_email(state: Arc<AppState>, id: &Uuid, email: &str) -> Result<(), ScyllaError> {
    let fallback_page_state = PagingState::start();

    state
        .database_session
        .execute_single_page(
            &state.database_queries.boiler_swap.insert_email,
            (id, email),
            fallback_page_state,
        )
        .await?;

    Ok(())
}

pub async fn get_email(state: Arc<AppState>, item_id: &Uuid) -> Result<String, ScyllaError> {
    let fallback_page_state = PagingState::start();

    let (returned_rows, _) = state
        .database_session
        .execute_single_page(
            &state.database_queries.boiler_swap.get_email,
            (item_id,),
            fallback_page_state,
        )
        .await?;

    let (email,) = returned_rows.into_rows_result()?.first_row::<(String,)>()?;

    Ok(email)
}

pub async fn delete_email(state: Arc<AppState>, item_id: &Uuid) -> Result<(), ScyllaError> {
    let fallback_page_state = PagingState::start();

    state
        .database_session
        .execute_single_page(
            &state.database_queries.boiler_swap.delete_email,
            (item_id,),
            fallback_page_state,
        )
        .await?;

    Ok(())
}
