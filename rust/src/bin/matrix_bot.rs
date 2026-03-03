use std::{env, sync::Arc};

use futures_util::{pin_mut, StreamExt};
use matrix_rust_sdk_bridge::{
    core::app_context::AppContext,
    features::{
        events::{
            ai_handler::AiMessageHandler,
            conversation_handler::ConversationActionHandler,
            domain::domain::{Address, DomainEvent, EventHandler, EventRepository},
            infraestructure::{
                infraestructure::{start_redpanda_worker, MockLlm, RedpandaEventRepository},
                matrix_conversation_gateway::MatrixConversationGateway,
                sqlx_conversation_resolver::SqlxConversationResolver,
            },
        },
        matrix_client_registry::domain::entities::registry_session::{
            ClientSessionEntity, Credentials, MatrixSessionEntity,
        },
    },
};
use sqlx::{PgPool, Row};
use uuid::Uuid;

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
    let database_url = env::var("DATABASE_URL").unwrap();

    // -------- DATABASE --------
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

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

    // 1. Setup Redpanda Infrastructure
    let event_repo = Arc::new(RedpandaEventRepository::new(
        brokers.as_str(),
        topic.as_str(),
        rp_username.as_str(),
        rp_password.as_str(),
    ));

    let llm = Arc::new(MockLlm);
    // -------- RESOLVER + GATEWAY --------
    let resolver = Arc::new(SqlxConversationResolver {
        pool: pool.clone(),
        matrix_client: client_context.clone(),
    });

    let gateway = Arc::new(MatrixConversationGateway {
        resolver,
        matrix_client: client_context.clone(),
    });
    // 3. Init Handlers
    let ai_handler = Arc::new(AiMessageHandler {
        llm,
        event_repository: event_repo.clone(),
    });

    let conversation_handler = Arc::new(ConversationActionHandler { gateway });
    // 3. Start the Redpanda Worker in a background task
    let handlers: Vec<Arc<dyn EventHandler>> = vec![ai_handler, conversation_handler];

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

    // 1. Setup the stream (Synchronously relative to this task)
    // If this fails, Flutter gets the error immediately.
    let sync_strem = client_context.sync_events.execute().await.unwrap();

    let task_id = uuid::Uuid::new_v4();
    println!("RUST: Bridge Task {} started", task_id);
    pin_mut!(sync_strem);
    // 2. The Loop (This blocks this specific FRB task, which is fine!)
    while let Some(telem) = sync_strem.next().await {
        println!("Sync result: {:#?}", telem);

        if telem.sender_mxid != matrix_user_id {
            // 🔥 ROOM → CONVERSATION LOOKUP
            let record = sqlx::query("SELECT id FROM conversations WHERE room_id = $1")
                .bind(&telem.room_id)
                .fetch_optional(&pool)
                .await
                .unwrap();

            let conversation_id = if let Some(row) = record {
                row.get::<Uuid, _>("id")
            } else {
                // create new conversation
                let new_id = Uuid::new_v4();

                sqlx::query("INSERT INTO conversations (id, room_id) VALUES ($1, $2)")
                    .bind(new_id)
                    .bind(&telem.room_id)
                    .execute(&pool)
                    .await
                    .unwrap();

                new_id
            };

            event_repo
                .persist_and_broadcast(DomainEvent::MessageReceived {
                    address: Address::ConversationId(conversation_id),
                    body: telem.content,
                })
                .await;
        }
    }

    println!("RUST: Task {} - Completed", task_id);
}
