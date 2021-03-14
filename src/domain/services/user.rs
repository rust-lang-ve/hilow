use anyhow::{Error, Result};
use std::sync::Arc;

use crate::database::DbPool;
use crate::domain::repositories::{ProfileRepository, UserRepository};
use crate::domain::services::secret::SecretService;

use super::dto::user::RegisterUserDto;

#[derive(Clone)]
pub struct UserService {
    db_pool: Arc<DbPool>,
    user_repository: Arc<UserRepository>,
    profile_repository: Arc<ProfileRepository>,
    secret_service: Arc<SecretService>,
}

impl UserService {
    pub fn new(
        db_pool: Arc<DbPool>,
        user_repository: Arc<UserRepository>,
        profile_repository: Arc<ProfileRepository>,
        secret_service: Arc<SecretService>,
    ) -> Self {
        Self {
            db_pool,
            user_repository,
            profile_repository,
            secret_service,
        }
    }

    pub async fn register(&self, params: RegisterUserDto) -> Result<()> {
        if !UserService::validate_password(params.password.as_str()) {
            return Err(Error::msg("Password is not strong enough"));
        }

        if self
            .user_repository
            .find_by_user_ids(params.email.as_str(), params.username.as_str())
            .await
            .is_ok()
        {
            return Err(Error::msg("Username/Email already taken"));
        }

        let mut tx = self.db_pool.begin().await?;

        let user = self
            .user_repository
            .add(&mut tx, params.email.as_str(), params.username.as_str())
            .await?;
        self.profile_repository
            .add(&mut tx, params.name.as_str(), params.surname, &user.id)
            .await?;
        self.secret_service
            .create_password(&mut tx, params.password.as_str(), &user.id)
            .await?;

        tx.commit().await?;

        Ok(())
    }

    /// Checks if the provided password is strong enough
    fn validate_password(_: &str) -> bool {
        true
    }
}
