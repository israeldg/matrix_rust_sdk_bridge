// src/infrastructure/network/connection_checker.rs
pub struct ConnectionChecker;

impl ConnectionChecker {
    pub fn new() -> Self {
        Self
    }

    pub fn is_connected(&self) -> bool {
        // Pretend to ping or check network
        true
    }
}
