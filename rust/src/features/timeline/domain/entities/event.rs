use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventEntity {
    pub id: String,
    pub sender_id: String,
    pub sender_display_name: Option<String>,
    pub sender_avatar_url: Option<String>,
    pub content: String,
    pub formatted_content: Option<String>,
    pub timestamp: Option<DateTime<Utc>>,
    pub is_redacted: bool,
    pub event_type: String,
    pub message_type: String,
    pub status: String,
    pub is_encrypted: bool,
    pub file_info: Option<HashMap<String, String>>,
}

impl Default for EventEntity {
    fn default() -> Self {
        Self {
            id: String::new(),
            sender_id: String::new(),
            sender_display_name: None,
            sender_avatar_url: None,
            content: String::from("hello world"),
            formatted_content: None,
            timestamp: None,
            is_redacted: false,
            event_type: String::new(),
            message_type: String::new(),
            status: String::new(),
            is_encrypted: false,
            file_info: None,
        }
    }
}
