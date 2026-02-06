use serde::{Deserialize, Serialize};

use crate::features::rooms::domain::entities::room::RoomEntity;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomModel {    
    pub entity: RoomEntity,
}

impl RoomModel {
    pub fn new(entity: RoomEntity) -> Self {
        Self { entity }
    }

    pub fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(&self.entity).unwrap()
    }

    pub fn from_json(json: &serde_json::Value) -> serde_json::Result<Self> {
        let entity: RoomEntity = serde_json::from_value(json.clone())?;
        Ok(Self { entity })
    }

    pub fn to_entity(self) -> RoomEntity {
        self.entity
    }
}
