use anyhow::{Error, Result};
use sqlx::postgres::Postgres;
use sqlx::Transaction;
use std::sync::Arc;
use uuid::Uuid;

use crate::database::DbPool;

#[derive(Clone)]
pub struct SecretRepository {
    db_pool: Arc<DbPool>,
}

impl SecretRepository {
    pub fn new(db_pool: Arc<DbPool>) -> Self {
        Self { db_pool }
    }

    pub async fn add(
        &self,
        tx: &mut Transaction<'static, Postgres>,
        hash: &str,
        user_id: &Uuid,
    ) -> Result<()> {
        match sqlx::query_file!("sql/secrets/insert.sql", hash, user_id)
            .fetch_one(tx)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::from(err)),
        }
    }
}
