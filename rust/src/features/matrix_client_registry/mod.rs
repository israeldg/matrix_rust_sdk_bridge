pub mod data {
    pub mod datasources {
        pub mod registry_remote_data_source;
    }
    pub mod models {
        pub mod registry_session_model;
    }
    pub mod repositories {
        pub mod registry_repository_impl;
    }
}
pub mod domain {
    pub mod entities {
        pub mod registry_session;
    }
    pub mod repositories {
        pub mod registry_repository;
    }
}

pub mod usecases {
    pub mod register_matrix_client;
}
