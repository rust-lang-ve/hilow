use std::convert::TryInto;
use std::env;

use crate::state::*;

/// Configuration of the hilow social feed api.
#[derive(Clone, Debug)]
pub struct Config {
    /// URL of the postgres database. The supported format is:
    /// postgres:://<username>:<password>@<host>:<port>/<db_name>
    database_url: String,
}

impl Config {
    /// Create a config from the environment variables.
    /// Environment variables used:
    ///     * POSTGRES_USER
    ///     * POSTGRES_PASSWORD
    ///     * POSTGRES_HOST
    ///     * POSTGRES_DB
    pub fn from_env() -> anyhow::Result<Self> {
        let database_url = format!(
            "postgres://{}:{}@{}/{}",
            env::var("POSTGRES_USER")?,
            env::var("POSTGRES_PASSWORD")?,
            env::var("POSTGRES_HOST")?,
            env::var("POSTGRES_DB")?
        );

        Ok(Self { database_url })
    }
}

impl TryInto<StateBuilder> for Config {
    type Error = anyhow::Error;
    fn try_into(self) -> anyhow::Result<StateBuilder> {
        Ok(StateBuilder::default().with_database(self.database_url))
    }
}
