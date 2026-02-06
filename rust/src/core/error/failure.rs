use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum CustomFailure {
    NotFound(String),
    DatabaseError(String),
    InvalidInput(String),
    NetworkError(String),
    Unknown(String),
    AccountNotFound(),
    NoActiveAccount(),
}

impl fmt::Display for CustomFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomFailure::NotFound(msg) => write!(f, "Resources not found: {}", msg),
            CustomFailure::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            CustomFailure::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            CustomFailure::NetworkError(msg) => write!(f, "Network error: {}", msg),
            CustomFailure::Unknown(msg) => write!(f, "Unknown error: {}", msg),
            CustomFailure::AccountNotFound() => write!(f, "AccountNotFound failure"),
            CustomFailure::NoActiveAccount() => write!(f, "NoActiveAccount failure"),
        }
    }
}

impl Error for CustomFailure {}
