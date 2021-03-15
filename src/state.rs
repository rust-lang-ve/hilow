use std::sync::Arc;

use anyhow::Result;
use sqlx::postgres::PgPool;

use crate::database::make_db_pool;
use crate::domain::services::Services;

/// Application `State` used to share resources
/// on different `App` locations such as endpoints
#[derive(Clone)]
pub struct State {
    pub services: Services,
    pub db_pool: Arc<PgPool>,
}

impl State {
    pub fn new() -> Result<Self> {
        let db_pool = make_db_pool()?;
        let db_pool = Arc::new(db_pool);

        let services = Services::new(Arc::clone(&db_pool))?;

        info!("Application state initialized with success!");
        Ok(Self { services, db_pool })
    }
}
