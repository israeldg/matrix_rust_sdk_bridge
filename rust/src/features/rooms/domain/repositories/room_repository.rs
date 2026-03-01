// src/domain/repositories/room_repository.rs
use crate::{
    core::error::failure::CustomFailure, features::rooms::domain::entities::room::RoomEntity,
};
use futures_util::stream::BoxStream;

pub trait RoomRepository {
    async fn get_spaces(&self) -> Result<Vec<RoomEntity>, CustomFailure>;
    async fn send_message_to_room(
        &self,
        room_id: String,
        message_content: String,
    ) -> Result<(), CustomFailure>;

    async fn get_rooms_by_space(
        &self,
        space_id: String,
    ) -> Result<BoxStream<'static, Vec<RoomEntity>>, CustomFailure>;
}
