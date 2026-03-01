#[derive(Debug)]
pub struct SimpleEvent {
    pub room_id: String,
    pub sender_mxid: String,
    pub content: String,
}

impl SimpleEvent {
    pub fn new(room_id: String, sender_mxid: String, content: String) -> Self {
        Self {
            room_id,
            sender_mxid,
            content,
        }
    }
}
