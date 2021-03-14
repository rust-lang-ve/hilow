use anyhow::{Context, Error, Result};
use sqlx::pool::Pool;
use sqlx::postgres::{PgPoolOptions, Postgres};
use std::env::var;

pub type DbPool = Pool<Postgres>;

/// Creates a new `DbPool` instance using the `DATABASE_URL`
/// environment variable provided.
///
/// `make_db_pool` also makes use of `DB_MAX_CONNECTIONS`
/// environment variable for connection pool configuration,
/// if `DB_MAX_CONNECTIONS` is not available, the max number
/// of connections will be set to `5` instead.
pub fn make_db_pool() -> Result<DbPool> {
    let database_url =
        var("DATABASE_URL").context("Missing \"DATABASE_URL\" environment variable")?;
    let max_db_conns = var("DB_MAX_CONNECTIONS")
        .unwrap_or_else(|_| String::from("5"))
        .parse::<u32>()
        .context("Invalid value provided for \"DB_MAX_CONNECTIONS\" expected numeric value")?;

    PgPoolOptions::new()
        .max_connections(max_db_conns)
        .connect_lazy(database_url.as_str())
        .map_err(Error::from)
}
