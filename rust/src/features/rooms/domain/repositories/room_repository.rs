use crate::frb_generated::StreamSink;
// src/domain/repositories/profile_repository.rs
use crate::{
    core::error::failure::CustomFailure, features::rooms::domain::entities::room::RoomEntity,
};

pub trait RoomRepository {
    async fn get_spaces(&self) -> Result<Vec<RoomEntity>, CustomFailure>;
    async fn get_rooms_by_space(&self, space_id: String, sink: StreamSink<Vec<RoomEntity>>) -> Result<(), CustomFailure>;
}
