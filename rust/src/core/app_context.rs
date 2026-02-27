// core/app_context.rs

use std::sync::Arc;

use crate::{
    core::common::matrix_client_management::{
        matrix_client_context_factory::MatrixClientContextFactory,
        matrix_client_registry::MatrixClientRegistry,
    },
    features::matrix_client_registry::{
        data::{
            datasources::registry_remote_data_source::RegistryRemoteDataSourceImpl,
            repositories::registry_repository_impl::RegistryRepositoryImpl,
        },
        usecases::register_matrix_client::RegisterMatrixClient,
    },
};

pub struct AppContext {
    pub registry: Arc<MatrixClientRegistry>,
    pub register_matrix_client:
        Arc<RegisterMatrixClient<RegistryRepositoryImpl<RegistryRemoteDataSourceImpl>>>,
}

impl AppContext {
    pub fn init() -> Self {
        let registry = Arc::new(MatrixClientRegistry::new());
        let client_factory = Arc::new(MatrixClientContextFactory);

        let registry_remote =
            RegistryRemoteDataSourceImpl::new(registry.clone(), client_factory.clone());
        let registry_repo = Arc::new(RegistryRepositoryImpl::new(registry_remote));
        let register_matrix_client = Arc::new(RegisterMatrixClient::new(registry_repo.clone()));

        Self {
            registry,
            register_matrix_client,
        }
    }
}
