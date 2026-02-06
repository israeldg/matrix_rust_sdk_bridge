pub mod data {
    pub mod datasources {
        pub mod timeline_remote_data_source;
    }

    mod models {
        pub mod event_model;
        pub mod timeline_handle;
    }

    pub mod repositories {
        pub mod timeline_repository_impl;
    }
}

pub mod domain {
    pub mod entities {
        pub mod event;
        pub mod event_entity_delta;
    }

    pub mod repositories {
        pub mod timeline_repository;
    }
}

pub mod usecases {
    pub mod fetch_room_events_by_room_id;
}
