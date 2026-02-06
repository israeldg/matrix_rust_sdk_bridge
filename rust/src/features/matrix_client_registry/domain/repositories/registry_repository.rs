// src/domain/repositories/profile_repository.rs
use crate::{
    core::error::failure::CustomFailure,
    features::matrix_client_registry::domain::entities::registry_session::MatrixSessionEntity,
};

pub trait RegistryRepository {
    async fn register_client(
        &self,
        client_session: MatrixSessionEntity,
    ) -> Result<MatrixSessionEntity, CustomFailure>;
}
