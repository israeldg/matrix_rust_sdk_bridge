use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::core::common::matrix_client_management::matrix_client_context::MatrixClientContext;
use crate::core::error::failure::CustomFailure;

#[derive(Default)]
pub struct MatrixClientRegistry {
    clients: RwLock<HashMap<String, Arc<MatrixClientContext>>>,
    active_account_id: RwLock<Option<String>>,
}

impl MatrixClientRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a new Matrix client context
    pub fn register_client(&self, account_id: String, ctx: Arc<MatrixClientContext>) {
        let mut clients = self.clients.write().unwrap();
        clients.insert(account_id.clone(), ctx);

        // Optionally set the first registered client as active
        let mut active = self.active_account_id.write().unwrap();
        if active.is_none() {
            *active = Some(account_id);
        }
    }

    /// Remove a client completely
    pub fn remove_client(&self, account_id: &str) {
        {
            let mut clients = self.clients.write().unwrap();
            clients.remove(account_id);
        }

        let mut active = self.active_account_id.write().unwrap();
        if active.as_ref().map(String::as_str) == Some(account_id) {
            *active = None; // caller may set another later
        }
    }

    /// Set active client by account ID
    pub fn set_active_account(&self, account_id: String) -> Result<(), CustomFailure> {
        let clients = self.clients.read().unwrap();
        if !clients.contains_key(&account_id) {
            return Err(CustomFailure::AccountNotFound());
        }

        *self.active_account_id.write().unwrap() = Some(account_id);
        Ok(())
    }

    /// Get the active client context
    pub fn get_active_client(&self) -> Result<Arc<MatrixClientContext>, CustomFailure> {
        let binding = self
            .active_account_id
            .read()
            .map_err(|_| CustomFailure::NoActiveAccount())?;
        let account_id = binding.as_ref().ok_or(CustomFailure::NoActiveAccount())?;

        self.clients
            .read()
            .map_err(|_| CustomFailure::AccountNotFound())?
            .get(account_id)
            .cloned()
            .ok_or(CustomFailure::AccountNotFound())
    }

    /// Get client by account ID
    pub fn get(&self, account_id: &str) -> Result<Arc<MatrixClientContext>, CustomFailure> {
        let clients = self
            .clients
            .read()
            .map_err(|_| CustomFailure::AccountNotFound())?;

        clients
            .get(account_id)
            .cloned()
            .ok_or(CustomFailure::AccountNotFound())
    }
}
