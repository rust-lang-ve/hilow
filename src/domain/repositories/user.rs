use anyhow::{Error, Result};
use sqlx::postgres::Postgres;
use sqlx::Transaction;
use std::ops::Deref;
use std::sync::Arc;

use crate::database::DbPool;

use super::dto::user::UserDto;

#[derive(Clone)]
pub struct UserRepository {
    db_pool: Arc<DbPool>,
}

impl UserRepository {
    pub fn new(db_pool: Arc<DbPool>) -> Self {
        Self { db_pool }
    }

    pub async fn find_by_user_ids(&self, email: &str, username: &str) -> Result<UserDto> {
        match sqlx::query_file!("sql/users/find_by_user_ids.sql", email, username)
            .fetch_one(self.db_pool.deref())
            .await
        {
            Ok(row) => Ok(UserDto {
                id: row.id,
                email: row.email,
                name: row.name,
                created_at: row.created_at,
                updated_at: row.updated_at,
            }),
            Err(err) => Err(Error::from(err)),
        }
    }

    pub async fn add(
        &self,
        tx: &mut Transaction<'static, Postgres>,
        email: &str,
        username: &str,
    ) -> Result<UserDto> {
        match sqlx::query_file!("sql/users/insert.sql", email, username)
            .fetch_one(tx)
            .await
        {
            Ok(row) => Ok(UserDto {
                id: row.id,
                email: row.email,
                name: row.name,
                created_at: row.created_at,
                updated_at: row.updated_at,
            }),
            Err(err) => Err(Error::from(err)),
        }
    }
}
