use anyhow::Result;
use std::sync::Arc;

use crate::database::DbPool;

use super::repositories::{ProfileRepository, SecretRepository, UserRepository};

pub mod dto;
mod secret;
mod user;

#[derive(Clone)]
pub struct Services {
    pub user_service: user::UserService,
}

impl Services {
    pub fn new(db_pool: Arc<DbPool>) -> Result<Self> {
        let user_repository = Arc::new(UserRepository::new(Arc::clone(&db_pool)));
        let profile_repository = Arc::new(ProfileRepository::new(Arc::clone(&db_pool)));
        let secret_repository = Arc::new(SecretRepository::new(Arc::clone(&db_pool)));

        let secret_service = Arc::new(secret::SecretService::new(secret_repository));
        let user_service = user::UserService::new(
            Arc::clone(&db_pool),
            user_repository,
            profile_repository,
            Arc::clone(&secret_service),
        );

        Ok(Self { user_service })
    }
}
