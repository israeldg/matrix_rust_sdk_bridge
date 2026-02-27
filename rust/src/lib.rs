#![recursion_limit = "256"]

pub mod api;
mod frb_generated;

pub mod features {
    pub mod auth;
    pub mod events;
    pub mod matrix_client_registry;
    pub mod rooms;
    pub mod sync;
    pub mod timeline;
}
pub mod core;
