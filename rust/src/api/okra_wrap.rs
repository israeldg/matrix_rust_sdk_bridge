use anyhow::{Ok, Result};

use once_cell::sync::OnceCell;
use std::sync::Arc;

// ========== Matrix SDK References ==========

// ========== Matrix SDK References ==========

// ========== Okra References ==========
use crate::core::common::matrix_client_management::matrix_client_registry::MatrixClientRegistry;
use crate::features::matrix_client_registry::data::datasources::registry_remote_data_source::RegistryRemoteDataSourceImpl;
use crate::features::matrix_client_registry::data::repositories::registry_repository_impl::RegistryRepositoryImpl;
use crate::features::matrix_client_registry::domain::entities::registry_session::MatrixSessionEntity;
use crate::features::timeline::domain::entities::event::EventEntity;
use crate::features::timeline::domain::entities::event_entity_delta::EventDeltaEntity;
use crate::{
    core::common::matrix_client_management::matrix_client_context_factory::MatrixClientContextFactory,
    features::matrix_client_registry::usecases::register_matrix_client::RegisterMatrixClient,
};

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

struct AppContext {
    // Reference to the main SDK client.
    registry: Arc<MatrixClientRegistry>,
    register_matrix_client:
        Arc<RegisterMatrixClient<RegistryRepositoryImpl<RegistryRemoteDataSourceImpl>>>,
}

#[flutter_rust_bridge::frb(opaque)]
impl AppContext {
    fn init() -> Self {
        let registry = Arc::new(MatrixClientRegistry::new());
        let client_factory = Arc::new(MatrixClientContextFactory);

        let registry_remote =
            RegistryRemoteDataSourceImpl::new(registry.clone(), client_factory.clone());
        let registry_repo = Arc::new(RegistryRepositoryImpl::new(registry_remote));
        let register_matrix_client = Arc::new(RegisterMatrixClient::new(registry_repo.clone()));

        Self {
            registry,
            register_matrix_client,
        }
    }
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

    client_context.sync_events.execute(sink).await
}

pub async fn sync_rooms_by_space(
    space_id: String,
    account_id: String,
    sink: StreamSink<Vec<RoomEntity>>,
) -> Result<()> {
    let app_context = global_app();

    let client_context = app_context.registry.get(account_id.as_str())?;

    client_context.get_rooms.execute(space_id, sink).await
}

pub async fn fetch_room_events_by_room_id(
    room_id: String,
    account_id: String,
    sink: StreamSink<Vec<EventDeltaEntity>>,
) -> Result<()> {
    let app_context = global_app();

    let client_context = app_context.registry.get(account_id.as_str())?;

    client_context
        .fetch_room_events_by_room_id
        .execute(room_id, sink)
        .await
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
