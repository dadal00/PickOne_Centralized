use crate::{
    config::try_load,
    error::ScyllaError,
    web::{
        housing::schema::{Housing, create_housing_tables},
        swap::schema::{BoilerSwap, create_swap_tables},
    },
};
use scylla::client::{session::Session, session_builder::SessionBuilder};
use std::sync::Arc;

pub const CREATE_KEYSPACE: &str = "CREATE KEYSPACE IF NOT EXISTS __KEYSPACE__ WITH REPLICATION = {{'class': 'SimpleStrategy', 'replication_factor': 1}}";

#[derive(Clone)]
pub struct DatabaseQueries {
    pub boiler_swap: BoilerSwap,
    pub housing: Housing,
}

impl DatabaseQueries {
    pub async fn init(session: &Session) -> Result<Self, ScyllaError> {
        Ok(Self {
            boiler_swap: BoilerSwap::init(session).await?,
            housing: Housing::init(session).await?,
        })
    }
}

pub async fn init_database() -> Result<(Arc<Session>, DatabaseQueries), ScyllaError> {
    let database_uri = try_load::<String>("RUST_DB_URI", "scylladb:9042").unwrap();

    let database_session: Session = SessionBuilder::new()
        .known_node(database_uri)
        .build()
        .await?;

    create_swap_tables(&database_session).await?;
    create_housing_tables(&database_session).await?;

    let database_queries = DatabaseQueries::init(&database_session).await?;

    Ok((Arc::new(database_session), database_queries))
}
