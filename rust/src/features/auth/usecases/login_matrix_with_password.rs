use std::sync::Arc;

use crate::core::error::failure::CustomFailure;

use crate::features::auth::domain::repositories::auth_repository::AuthRepository;
use crate::features::matrix_client_registry::domain::entities::registry_session::UserSessionEntity;
use anyhow::Result;

pub struct LoginMatrixWithPassword<R: AuthRepository> {
    repo: Arc<R>,
}

impl<R: AuthRepository> LoginMatrixWithPassword<R>
where
    R: AuthRepository,
{
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }

    pub async fn execute(
        &self,
        username: String,
        password: String,
    ) -> Result<UserSessionEntity, CustomFailure> {
        self.repo.login_with_password(username, password).await
    }
}
