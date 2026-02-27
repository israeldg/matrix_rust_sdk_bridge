use std::sync::Arc;

use matrix_rust_sdk_bridge::features::events::{
    application::{AiHandler, MatrixReplyHandler, ProcessAiUseCase, SendReplyUseCase},
    domain::{DomainEvent, EventHandler, EventRepository},
    infraestructure::{start_redpanda_worker, MockLlm, MockMatrix, RedpandaEventRepository},
};

//use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    println!("--- Starting Simple Event-Driven Matrix AI ---");

    let brokers = "127.0.0.1:19092,127.0.0.1:29092,127.0.0.1:39092";
    let topic = "matrix_events";
    let username = "superuser";
    let password = "xxxxxx";
    let group_id = "bot-group";

    // 1. Setup Redpanda Infrastructure
    let event_repo = Arc::new(RedpandaEventRepository::new(
        brokers, topic, username, password,
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
        start_redpanda_worker(brokers, username, password, group_id, topic, handlers).await;
    });

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // 4. Trigger initial event via Redpanda
    event_repo
        .persist_and_broadcast(DomainEvent::MatrixMessageReceived {
            room_id: "!room:redpanda.com".to_string(),
            body: "Stream this!".to_string(),
        })
        .await;

    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    // // 1. Initialize Infrastructure (Data Source & Clients)
    // let (tx, _) = broadcast::channel::<DomainEvent>(100);
    // let event_repo = Arc::new(InMemoryEventRepository::new(tx.clone()));
    // let llm = Arc::new(MockLlm);
    // let matrix = Arc::new(MockMatrix);

    // // 2. Initialize Use Cases
    // let ai_use_case = Arc::new(ProcessAiUseCase {
    //     llm: llm.clone(),
    //     event_repo: event_repo.clone(),
    // });
    // let reply_use_case = Arc::new(SendReplyUseCase {
    //     adapter: matrix.clone(),
    //     event_repo: event_repo.clone(),
    // });

    // // 3. Initialize Handlers
    // let ai_handler = Arc::new(AiHandler {
    //     use_case: ai_use_case,
    // });
    // let reply_handler = Arc::new(MatrixReplyHandler {
    //     use_case: reply_use_case,
    // });

    // // 4. Register Handlers (The Plumbing)
    // register_listener(tx.clone(), ai_handler);
    // register_listener(tx.clone(), reply_handler);

    // // 5. Trigger First Event (Simulates message arriving from Matrix)
    // println!("\n📥 [Simulator] New message arrived from Matrix Room");
    // event_repo
    //     .persist_and_broadcast(DomainEvent::MatrixMessageReceived {
    //         room_id: "!rust:matrix.org".to_string(),
    //         body: "Hello world!".to_string(),
    //     })
    //     .await;

    // // Give it a moment to complete the async cycle
    // tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    // println!("\n--- POC Completed ---");
}

// fn register_listener(sender: broadcast::Sender<DomainEvent>, handler: Arc<dyn EventHandler>) {
//     let mut rx: broadcast::Receiver<DomainEvent> = sender.subscribe();
//     tokio::spawn(async move {
//         while let Ok(event) = rx.recv().await {
//             handler.handle(event).await;
//         }
//     });
// }
