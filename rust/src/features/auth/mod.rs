pub mod data {
    pub mod datasources {
        pub mod auth_remote_data_source;
    }

    pub mod repositories {
        pub mod auth_repository_impl;
    }
}
pub mod domain {

    pub mod repositories {
        pub mod auth_repository;
    }
}

pub mod usecases {
    pub mod login_matrix_with_password;
    pub mod restore_matrix_session;
}
