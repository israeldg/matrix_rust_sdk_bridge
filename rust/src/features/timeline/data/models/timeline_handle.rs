use std::sync::Arc;

use flutter_rust_bridge::JoinHandle;

pub struct TimelineHandle {
    pub timeline: Arc<matrix_sdk_ui::Timeline>,
    pub task: JoinHandle<()>,
}
