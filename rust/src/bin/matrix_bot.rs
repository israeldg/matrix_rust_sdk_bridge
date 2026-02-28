use std::{env, sync::Arc};

use futures_util::{pin_mut, StreamExt};
use matrix_rust_sdk_bridge::{
    core::app_context::AppContext,
    features::{
        events::{
            application::{AiHandler, MatrixReplyHandler, ProcessAiUseCase, SendReplyUseCase},
            domain::{DomainEvent, EventHandler, EventRepository},
            infraestructure::{
                start_redpanda_worker, MockLlm, MockMatrix, RedpandaEventRepository,
            },
        },
        matrix_client_registry::domain::entities::registry_session::{
            ClientSessionEntity, Credentials, MatrixSessionEntity, UserSessionEntity,
        },
    },
};

//use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    println!("--- GETTING ENV VARIABLES ---");
    dotenvy::dotenv().ok(); // Load .env file

    let brokers = env::var("BROKERS").unwrap();
    let topic = env::var("TOPIC").unwrap();
    let rp_username = env::var("RP_USERNAME").unwrap();
    let rp_password = env::var("RP_PASSWORD").unwrap();
    let group_id = env::var("GROUP_ID").unwrap();

    let homeserver = env::var("HOMESERVER").unwrap();
    let session_path = env::var("SESSION_PATH").unwrap();
    let passphrase = env::var("PASSPHRASE").unwrap();
    let matrix_user_id = env::var("MATRIX_USER_ID").unwrap();
    let matrix_username = env::var("MATRIX_USERNAME").unwrap();
    let matrix_password = env::var("MATRIX_PASSWORD").unwrap();

    println!("--- Starting Simple Event-Driven Matrix AI ---");

    // 1. Setup Redpanda Infrastructure
    let event_repo = Arc::new(RedpandaEventRepository::new(
        brokers.as_str(),
        topic.as_str(),
        rp_username.as_str(),
        rp_password.as_str(),
    ));
    let llm = Arc::new(MockLlm);
    let matrix = Arc::new(MockMatrix);

    // 2. Init Use Cases
    let ai_use_case = Arc::new(ProcessAiUseCase {
        llm,
        event_repo: event_repo.clone(),
    });
    let reply_use_case = Arc::new(SendReplyUseCase {
        adapter: matrix,
        event_repo: event_repo.clone(),
    });

    // 3. Init Handlers
    let ai_handler = Arc::new(AiHandler {
        use_case: ai_use_case,
    });
    let reply_handler = Arc::new(MatrixReplyHandler {
        use_case: reply_use_case,
    });

    // 3. Start the Redpanda Worker in a background task
    let handlers: Vec<Arc<dyn EventHandler>> = vec![ai_handler, reply_handler];

    tokio::spawn(async move {
        start_redpanda_worker(
            brokers.as_str(),
            rp_username.as_str(),
            rp_password.as_str(),
            group_id.as_str(),
            topic.as_str(),
            handlers,
        )
        .await;
    });

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    //setting up matrix client
    let client_session = ClientSessionEntity::new(homeserver, session_path, passphrase);

    let matrix_session = MatrixSessionEntity::new(
        client_session,
        None, // no user session yet
        None,
        Some(Credentials::UserPassword {
            username: matrix_username,
            password: matrix_password,
        }),
    );

    println!("{:#?}", matrix_session);

    let app_context = Arc::new(AppContext::init());

    let full_session = app_context
        .register_matrix_client
        .execute(matrix_session)
        .await
        .unwrap();

    let client_context = app_context.registry.get(&matrix_user_id).unwrap();

    // 1. Setup the stream (Synchronously relative to this task)
    // If this fails, Flutter gets the error immediately.
    let sync_strem = client_context.sync_events.execute().await.unwrap();

    let task_id = uuid::Uuid::new_v4();
    println!("RUST: Bridge Task {} started", task_id);
    pin_mut!(sync_strem);
    // 2. The Loop (This blocks this specific FRB task, which is fine!)
    while let Some(telem) = sync_strem.next().await {
        println!("Sync result: {}", telem);
        //4. Trigger initial event via Redpanda
        event_repo
            .persist_and_broadcast(DomainEvent::MatrixMessageReceived {
                room_id: "!KlTyRSlWiVvxzSyTmq:matrix-n0k0g8c444gcos00wwo84sg0.neivi.app"
                    .to_string(),
                body: telem,
            })
            .await;
    }

    println!("RUST: Task {} - Completed", task_id);

    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
}
