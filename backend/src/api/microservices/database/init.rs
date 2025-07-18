use super::schema::{
    KEYSPACE,
    columns::{items, users},
    tables,
};
use crate::{AppError, config::try_load};
use scylla::{
    client::{session::Session, session_builder::SessionBuilder},
    statement::{prepared::PreparedStatement, unprepared::Statement},
};
use std::sync::Arc;

#[derive(Clone)]
pub struct DatabaseQueries {
    pub get_user: PreparedStatement,
    pub insert_user: PreparedStatement,
    pub check_lock: PreparedStatement,
    pub update_lock: PreparedStatement,
    pub unlock_account: PreparedStatement,
    pub insert_item: PreparedStatement,
    pub get_items: PreparedStatement,
    pub delete_item: PreparedStatement,
    pub get_cron_items: PreparedStatement,
}

impl DatabaseQueries {
    pub async fn init(session: &Session) -> Result<Self, AppError> {
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
                "INSERT INTO {}.{} ({}, {}, {}, {}, {}, {}, {}, {}) VALUES (?, ?, ?, ?, ?, ?, ?, ?) USING TTL ?",
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
        })
    }
}

pub async fn init_database() -> Result<(Arc<Session>, DatabaseQueries), AppError> {
    let database_uri = try_load::<String>("RUST_DB_URI", "scylladb:9042").unwrap();

    let database_session: Session = SessionBuilder::new()
        .known_node(database_uri)
        .build()
        .await?;

    create_tables(&database_session).await?;

    let database_queries = DatabaseQueries::init(&database_session).await?;

    Ok((Arc::new(database_session), database_queries))
}

async fn create_tables(session: &Session) -> Result<(), AppError> {
    session.query_unpaged(
        format!("CREATE KEYSPACE IF NOT EXISTS {} WITH REPLICATION = {{'class': 'SimpleStrategy', 'replication_factor': 1}}", KEYSPACE),
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

    Ok(())
}
