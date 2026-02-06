use serde::{Deserialize, Serialize};

use crate::features::{
    rooms::domain::entities::room::RoomEntity, timeline::domain::entities::event::EventEntity,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventModel {
    pub entity: EventEntity,
}

impl EventModel {
    pub fn new(entity: EventEntity) -> Self {
        Self { entity }
    }

    pub fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(&self.entity).unwrap()
    }

    pub fn from_json(json: &serde_json::Value) -> serde_json::Result<Self> {
        let entity: EventEntity = serde_json::from_value(json.clone())?;
        Ok(Self { entity })
    }

    pub fn to_entity(self) -> EventEntity {
        self.entity
    }
}
