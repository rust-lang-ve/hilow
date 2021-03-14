use anyhow::{Error, Result};
use argon2::{hash_encoded, Config};
use rand::{thread_rng, Rng};
use sqlx::postgres::Postgres;
use sqlx::Transaction;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::repositories::SecretRepository;

#[derive(Clone)]
pub struct SecretService {
    secret_repository: Arc<SecretRepository>,
}

impl SecretService {
    pub fn new(secret_repository: Arc<SecretRepository>) -> Self {
        Self { secret_repository }
    }

    pub async fn create_password(
        &self,
        tx: &mut Transaction<'static, Postgres>,
        pwd: &str,
        user_id: &Uuid,
    ) -> Result<()> {
        let password_hash = self.hash(pwd.as_bytes())?;

        self.secret_repository
            .add(tx, password_hash.as_str(), user_id)
            .await?;

        Ok(())
    }

    fn hash(&self, pwd: &[u8]) -> Result<String> {
        let config = Config::default();
        let salt = thread_rng().gen::<[u8; 32]>();
        let hash = hash_encoded(pwd, &salt, &config);

        hash.map_err(Error::from)
    }
}
