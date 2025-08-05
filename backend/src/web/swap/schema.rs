use super::schema::columns::{items, users};
use crate::{error::ScyllaError, microservices::database::CREATE_KEYSPACE};
use scylla::{
    client::session::Session,
    statement::{Statement, prepared::PreparedStatement},
};

pub const KEYSPACE: &str = "boiler_swap";

pub mod tables {
    pub const USERS: &str = "users";
    pub const ITEMS: &str = "items";
    pub const ITEMS_CDC: &str = "items_cdc";
    pub const ITEMS_EMAIL: &str = "items_email";
}

pub mod columns {
    pub mod users {
        pub const EMAIL: &str = "email";
        pub const EMAIL_TYPE: &str = "text";

        pub const PASSWORD_HASH: &str = "password_hash";
        pub const PASSWORD_HASH_TYPE: &str = "text";

        pub const LOCKED: &str = "locked";
        pub const LOCKED_TYPE: &str = "boolean";

        pub const PRIMARY_KEY: &str = EMAIL;
        pub const TTL: &str = "126144000";
    }

    pub mod items {
        pub const ITEM_ID: &str = "item_id";
        pub const ITEM_ID_TYPE: &str = "uuid";

        pub const ITEM_TYPE: &str = "item_type";
        pub const ITEM_TYPE_TYPE: &str = "tinyint";

        pub const TITLE: &str = "title";
        pub const TITLE_TYPE: &str = "text";

        pub const CONDITION: &str = "condition";
        pub const CONDITION_TYPE: &str = "tinyint";

        pub const LOCATION: &str = "location";
        pub const LOCATION_TYPE: &str = "tinyint";

        pub const DESCRIPTION: &str = "description";
        pub const DESCRIPTION_TYPE: &str = "text";

        pub const EMOJI: &str = "emoji";
        pub const EMOJI_TYPE: &str = "tinyint";

        pub const EXPIRATION_DATE: &str = "expiration_date";
        pub const EXPIRATION_DATE_TYPE: &str = "date";

        pub const PRIMARY_KEY: &str = ITEM_ID;
        pub const TTL: &str = "1814400";
        pub const SAFE_TTL: &str = "2419200";
    }
}

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
