use std::path::Path;
use std::sync::Arc;

use crate::core::error::failure::CustomFailure;
use crate::features::auth::domain::entities::auth_session::MatrixSessionEntity;
use crate::features::auth::domain::repositories::auth_repository::AuthRepository;
use anyhow::Result;

pub struct LoginMatrix<R: AuthRepository> {
    repo: Arc<R>,
}

impl<R: AuthRepository> LoginMatrix<R>
where
    R: AuthRepository,
{
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }

    pub async fn execute(
        &self,
        home_server: &String,
        data_dir: &Path,
        auth_token: &str,
    ) -> Result<MatrixSessionEntity, CustomFailure> {
        self.repo.login(home_server, data_dir, auth_token).await
    }
}
