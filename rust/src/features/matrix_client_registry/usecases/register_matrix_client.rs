use std::sync::Arc;

use crate::core::error::failure::CustomFailure;

use crate::features::matrix_client_registry::domain::entities::registry_session::MatrixSessionEntity;
use crate::features::matrix_client_registry::domain::repositories::registry_repository::RegistryRepository;
use anyhow::Result;

pub struct RegisterMatrixClient<R: RegistryRepository> {
    repo: Arc<R>,
}

impl<R: RegistryRepository> RegisterMatrixClient<R>
where
    R: RegistryRepository,
{
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }
    pub async fn execute(
        &self,
        client_session: MatrixSessionEntity,
    ) -> Result<MatrixSessionEntity, CustomFailure> {
        self.repo.register_client(client_session).await
    }
}
