use anyhow::{Ok, Result};

use futures_util::pin_mut;
use futures_util::StreamExt;
use once_cell::sync::OnceCell;
use std::sync::Arc;

// ========== Matrix SDK References ==========

// ========== Matrix SDK References ==========

use crate::core::app_context::AppContext;
// ========== Okra References ==========

use crate::features::matrix_client_registry::domain::entities::registry_session::MatrixSessionEntity;
use crate::features::timeline::domain::entities::event_entity_delta::EventDeltaEntity;

use crate::features::rooms::domain::entities::room::RoomEntity;

use std::{thread::sleep, time::Duration};

use crate::frb_generated::StreamSink;
// ========== Okra References ==========

// Flutter Rust Bridge Init
#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}

// ========== Global app context singleton ==========
static APP: OnceCell<Arc<AppContext>> = OnceCell::new();

//Manage Global Context
fn global_app() -> &'static Arc<AppContext> {
    APP.get_or_init(|| Arc::new(AppContext::init()))
}

// ==========Rust Bridge Public Methods ==========

pub async fn register_matrix_client(session: MatrixSessionEntity) -> Result<MatrixSessionEntity> {
    println!(
        "RUST: Attempting connection to: {}",
        session.client_session.homeserver
    );

    let app_context = global_app();

    let full_session = app_context.register_matrix_client.execute(session).await?;

    Ok(full_session)
}

pub async fn sync_events(account_id: String, sink: StreamSink<String>) -> Result<()> {
    let app_context = global_app();

    let client_context = app_context.registry.get(account_id.as_str())?;

    // 1. Setup the stream (Synchronously relative to this task)
    // If this fails, Flutter gets the error immediately.
    let sync_strem = client_context.sync_events.execute().await?;

    let task_id = uuid::Uuid::new_v4();
    println!("RUST: Bridge Task {} started", task_id);
    pin_mut!(sync_strem);
    // 2. The Loop (This blocks this specific FRB task, which is fine!)
    while let Some(telem) = sync_strem.next().await {
        if sink.add(telem.content).is_err() {
            println!("RUST: Task {} - Sink closed", task_id);
            break;
        }
    }

    println!("RUST: Task {} - Completed", task_id);
    Ok(())
}

pub async fn sync_rooms_by_space(
    space_id: String,
    account_id: String,
    sink: StreamSink<Vec<RoomEntity>>,
) -> Result<()> {
    let app_context = global_app();
    let client_context = app_context.registry.get(account_id.as_str())?;

    // 1. Setup the stream (Synchronously relative to this task)
    // If this fails, Flutter gets the error immediately.
    let room_stream = client_context.get_rooms.execute(space_id).await?;

    let task_id = uuid::Uuid::new_v4();
    println!("RUST: Bridge Task {} started", task_id);
    pin_mut!(room_stream);
    // 2. The Loop (This blocks this specific FRB task, which is fine!)
    while let Some(rooms) = room_stream.next().await {
        if sink.add(rooms).is_err() {
            println!("RUST: Task {} - Sink closed", task_id);
            break;
        }
    }

    println!("RUST: Task {} - Completed", task_id);
    Ok(())
}

pub async fn fetch_room_events_by_room_id(
    room_id: String,
    account_id: String,
    sink: StreamSink<Vec<EventDeltaEntity>>,
) -> Result<()> {
    let app_context = global_app();

    let client_context = app_context.registry.get(account_id.as_str())?;

    let stream = client_context
        .fetch_room_events_by_room_id
        .execute(room_id)
        .await?;

    pin_mut!(stream);

    while let Some(deltas) = stream.next().await {
        if sink.add(deltas).is_err() {
            break; // User left the room, stop the stream
        }
    }
    Ok(())
}

const ONE_SECOND: Duration = Duration::from_secs(1);

// can't omit the return type yet, this is a bug
pub fn tick(sink: StreamSink<i32>) -> Result<()> {
    let mut ticks = 0;
    loop {
        sink.add(ticks);
        sleep(ONE_SECOND);
        if ticks == i32::MAX {
            break;
        }
        ticks += 1;
    }
    Ok(())
}
