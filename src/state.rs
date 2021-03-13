use sqlx::postgres::PgPool;
use std::{convert::TryInto, sync::Arc};

use crate::config::Config;

/// State Builder
#[derive(Default)]
pub struct StateBuilder {
    database_url: String,
}

impl StateBuilder {
    /// Set the database_url in the builder
    pub fn with_database(mut self, database_url: String) -> Self {
        self.database_url = database_url;
        self
    }

    /// Build the app state with the current configurations
    pub async fn build(self) -> anyhow::Result<State> {
        State::new(&self.database_url).await
    }
}
/// Struct used to store the State of the application.
pub struct State {
    pub db: PgPool,
}

impl State {
    /// Create a new state with the given database_url
    pub async fn new(database_url: &str) -> anyhow::Result<Self> {
        let db = PgPool::connect(database_url).await?;
        Ok(Self { db })
    }
}

pub type AppState = Arc<State>;

/// Init the State of the app from environment variables and wrap it in an Arc
pub async fn init_app_state() -> anyhow::Result<AppState> {
    let state_builder: StateBuilder = Config::from_env()?.try_into()?;

    let state = state_builder.build().await?;

    Ok(Arc::new(state))
}
