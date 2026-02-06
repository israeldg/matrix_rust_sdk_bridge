use serde::{Deserialize, Serialize};

use crate::features::timeline::domain::entities::event::EventEntity;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventDeltaEntity {
    PushFront { value: EventEntity },
    PushBack { value: EventEntity },
    Insert { index: u32, value: EventEntity },
    Remove { index: u32 },
    Update { index: u32, value: EventEntity },
    Reset { items: Vec<EventEntity> },
}
