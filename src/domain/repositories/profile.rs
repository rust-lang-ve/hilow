use anyhow::{Error, Result};
use sqlx::postgres::Postgres;
use sqlx::Transaction;
use std::sync::Arc;
use uuid::Uuid;

use crate::database::DbPool;

use super::dto::profile::ProfileDto;

#[derive(Clone)]
pub struct ProfileRepository {
    db_pool: Arc<DbPool>,
}

impl ProfileRepository {
    pub fn new(db_pool: Arc<DbPool>) -> Self {
        Self { db_pool }
    }

    pub async fn add(
        &self,
        tx: &mut Transaction<'static, Postgres>,
        name: &str,
        surname: Option<String>,
        user_id: &Uuid,
    ) -> Result<ProfileDto> {
        match sqlx::query_file!("sql/profiles/insert.sql", name, surname, user_id)
            .fetch_one(tx)
            .await
        {
            Ok(row) => Ok(ProfileDto {
                id: row.id,
                name: row.name,
                surname: row.surname,
                user_id: row.user_id,
                created_at: row.created_at,
                updated_at: row.updated_at,
            }),
            Err(err) => Err(Error::from(err)),
        }
    }
}
