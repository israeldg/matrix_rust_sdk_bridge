Absolutely! That's a great architectural pattern. Here's how to structure it with separate error types for each layer:

## DataSource Layer Error

```rust
// datasource/error.rs
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum DataSourceError {
    ConnectionFailed(String),
    QueryFailed(String),
    Timeout,
    SerializationError(String),
    PoolExhausted,
}

impl fmt::Display for DataSourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataSourceError::ConnectionFailed(msg) => write!(f, "Connection failed: {}", msg),
            DataSourceError::QueryFailed(msg) => write!(f, "Query failed: {}", msg),
            DataSourceError::Timeout => write!(f, "Database operation timed out"),
            DataSourceError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            DataSourceError::PoolExhausted => write!(f, "Connection pool exhausted"),
        }
    }
}

impl Error for DataSourceError {}
```

## DataSource Implementation

```rust
// datasource/user_datasource.rs
use super::error::DataSourceError;

pub struct UserDataSource {
    // database connection, etc.
}

impl UserDataSource {
    pub fn fetch_by_id(&self, id: u32) -> Result<UserEntity, DataSourceError> {
        // Simulate database operation
        if id == 0 {
            return Err(DataSourceError::QueryFailed("Invalid ID".to_string()));
        }
        
        // Database query logic
        Ok(UserEntity { id, name: "John".to_string() })
    }

    pub fn insert(&self, entity: UserEntity) -> Result<u32, DataSourceError> {
        // Simulate connection error
        Err(DataSourceError::ConnectionFailed("No connection available".to_string()))
    }

    pub fn delete(&self, id: u32) -> Result<(), DataSourceError> {
        if id == 0 {
            return Err(DataSourceError::QueryFailed("Cannot delete ID 0".to_string()));
        }
        Ok(())
    }
}
```

## Repository Layer Error (CustomFailure)

```rust
// repository/error.rs
use std::error::Error;
use std::fmt;
use crate::datasource::error::DataSourceError;

#[derive(Debug)]
pub enum CustomFailure {
    NotFound,
    DatabaseError(String),
    InvalidInput(String),
    NetworkError(String),
    Unknown(String),
}

impl fmt::Display for CustomFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomFailure::NotFound => write!(f, "Resource not found"),
            CustomFailure::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            CustomFailure::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            CustomFailure::NetworkError(msg) => write!(f, "Network error: {}", msg),
            CustomFailure::Unknown(msg) => write!(f, "Unknown error: {}", msg),
        }
    }
}

impl Error for CustomFailure {}

// Convert DataSourceError to CustomFailure
impl From<DataSourceError> for CustomFailure {
    fn from(error: DataSourceError) -> Self {
        match error {
            DataSourceError::ConnectionFailed(msg) => 
                CustomFailure::DatabaseError(format!("Connection failed: {}", msg)),
            DataSourceError::QueryFailed(msg) => 
                CustomFailure::DatabaseError(format!("Query failed: {}", msg)),
            DataSourceError::Timeout => 
                CustomFailure::DatabaseError("Operation timed out".to_string()),
            DataSourceError::SerializationError(msg) => 
                CustomFailure::Unknown(format!("Serialization error: {}", msg)),
            DataSourceError::PoolExhausted => 
                CustomFailure::DatabaseError("Connection pool exhausted".to_string()),
        }
    }
}
```

## Repository Implementation

```rust
// repository/user_repository.rs
use crate::datasource::UserDataSource;
use crate::repository::error::CustomFailure;

pub struct UserRepository {
    datasource: UserDataSource,
}

impl UserRepository {
    pub fn new(datasource: UserDataSource) -> Self {
        Self { datasource }
    }

    pub fn find_user(&self, id: u32) -> Result<User, CustomFailure> {
        // DataSourceError is automatically converted to CustomFailure via From trait
        let entity = self.datasource.fetch_by_id(id)?;
        
        // Map entity to domain model
        Ok(User {
            id: entity.id,
            name: entity.name,
        })
    }

    pub fn save_user(&self, user: User) -> Result<u32, CustomFailure> {
        // Validate domain rules
        if user.name.is_empty() {
            return Err(CustomFailure::InvalidInput("Name cannot be empty".to_string()));
        }

        let entity = UserEntity {
            id: user.id,
            name: user.name,
        };

        // DataSourceError automatically converts to CustomFailure
        let id = self.datasource.insert(entity)?;
        Ok(id)
    }

    pub fn delete_user(&self, id: u32) -> Result<(), CustomFailure> {
        // Handle specific datasource errors differently if needed
        match self.datasource.delete(id) {
            Ok(_) => Ok(()),
            Err(DataSourceError::QueryFailed(_)) => Err(CustomFailure::NotFound),
            Err(e) => Err(e.into()), // Convert other errors via From trait
        }
    }
}
```

## UseCase Layer (converts to anyhow)

```rust
// usecase/user_usecase.rs
use anyhow::{Result, Context};
use crate::repository::{UserRepository, CustomFailure};

pub struct UserUseCase {
    repo: UserRepository,
}

impl UserUseCase {
    pub fn new(repo: UserRepository) -> Self {
        Self { repo }
    }

    pub fn get_user(&self, id: u32) -> Result<User> {
        self.repo.find_user(id)
            .context(format!("Failed to get user with id {}", id))
    }

    pub fn create_user(&self, name: String) -> Result<u32> {
        let user = User { id: 0, name };
        
        self.repo.save_user(user)
            .context("Failed to create user")
    }

    pub fn remove_user(&self, id: u32) -> Result<()> {
        match self.repo.delete_user(id) {
            Ok(_) => Ok(()),
            Err(CustomFailure::NotFound) => {
                anyhow::bail!("User with id {} not found", id)
            }
            Err(e) => Err(e).context("Failed to remove user"),
        }
    }
}
```

## Architecture Overview

```
┌─────────────────────────────────────────┐
│  UseCase Layer (anyhow::Result)         │
│  - Business logic                        │
│  - Orchestration                         │
└──────────────┬──────────────────────────┘
               │ converts to anyhow
┌──────────────▼──────────────────────────┐
│  Repository Layer (CustomFailure)        │
│  - Domain logic                          │
│  - Entity mapping                        │
└──────────────┬──────────────────────────┘
               │ From<DataSourceError>
┌──────────────▼──────────────────────────┐
│  DataSource Layer (DataSourceError)      │
│  - Database operations                   │
│  - Low-level I/O                         │
└─────────────────────────────────────────┘
```

## Benefits

1. **Separation of concerns**: Each layer has errors appropriate to its abstraction level
2. **Type safety**: Compiler ensures proper error handling at each boundary
3. **Automatic conversion**: `From` trait makes `?` operator work seamlessly
4. **Specific error handling**: You can match on specific error types at any layer
5. **Clean architecture**: Dependencies point inward (DataSource → Repository → UseCase)

The `From<DataSourceError> for CustomFailure` implementation is key—it allows the `?` operator to automatically convert datasource errors to repository errors!