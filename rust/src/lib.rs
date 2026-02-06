#![recursion_limit = "256"]

pub mod api;
mod frb_generated;

mod features {
    pub mod auth;
    pub mod matrix_client_registry;
    pub mod rooms;
    pub mod sync;
    pub mod timeline;
}
mod core;
