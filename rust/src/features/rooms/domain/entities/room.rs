use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};



#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoomEntity {
    pub room_id: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub last_event_text: Option<String>,
    pub last_event_received_time: Option<DateTime<Utc>>,
    pub is_direct_chat: bool,
    pub is_encrypted: bool,
    pub last_event: Option<String>,
    pub participant_count: i32,
}

impl RoomEntity {
    pub fn new(
        room_id: String,
        display_name: String,
        is_direct_chat: bool,
        is_encrypted: bool,
        avatar_url: Option<String>,
        last_event_text: Option<String>,
        last_event_received_time: Option<DateTime<Utc>>,
        last_event: Option<String>,
        participant_count: Option<i32>,
    ) -> Self {
        Self {
            room_id,
            display_name,
            avatar_url,
            last_event_text,
            last_event_received_time,
            is_direct_chat,
            is_encrypted,
            last_event,
            participant_count: participant_count.unwrap_or(0),
        }
    }
}