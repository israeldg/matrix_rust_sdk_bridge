use crate::{
    core::error::failure::CustomFailure,
    features::matrix_client_registry::{
        data::datasources::registry_remote_data_source::RegistryRemoteDataSource,
        domain::{
            entities::registry_session::MatrixSessionEntity,
            repositories::registry_repository::RegistryRepository,
        },
    },
};

pub struct RegistryRepositoryImpl<R: RegistryRemoteDataSource> {
    remote: R,
}

impl<R: RegistryRemoteDataSource> RegistryRepositoryImpl<R> {
    pub fn new(remote: R) -> Self {
        Self { remote }
    }
}

impl<R: RegistryRemoteDataSource> RegistryRepository for RegistryRepositoryImpl<R> {
    async fn register_client(
        &self,
        client_session: MatrixSessionEntity,
    ) -> Result<MatrixSessionEntity, CustomFailure> {
        Ok(self.remote.register_client(client_session).await?)
    }
}
