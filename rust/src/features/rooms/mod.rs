pub mod data {
    pub mod datasources {
        pub mod room_remote_data_source;
    }

    mod models {
        pub mod room_model;
    }

    pub mod repositories {
        pub mod room_repository_impl;
    }
}

pub mod domain {
    pub mod entities {
        pub mod room;
    }

    pub mod repositories {
        pub mod room_repository;
    }
}

pub mod usecases {
    pub mod get_rooms;
    pub mod get_spaces;
}
