pub mod data {
    pub mod datasources {
        pub mod sync_remote_data_source;
    }

    pub mod repositories {
        pub mod sync_repository_impl;
    }
}

pub mod domain {

    pub mod repositories {
        pub mod sync_repository;
    }
}

pub mod usecases {
    pub mod sync_events;
}
