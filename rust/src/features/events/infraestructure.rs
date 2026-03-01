use std::sync::Arc;

use matrix_sdk::async_trait;

use rdkafka::{
    consumer::{Consumer, StreamConsumer},
    producer::{FutureProducer, FutureRecord},
    ClientConfig, Message,
};

use tokio::sync::broadcast;

use crate::features::events::domain::{
    DomainEvent, EventHandler, EventRepository, LlmClient, MatrixAdapter,
};

pub struct RedpandaEventRepository {
    producer: FutureProducer,
    topic: String,
}
impl RedpandaEventRepository {
    pub fn new(brokers: &str, topic: &str, username: &str, password: &str) -> Self {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("security.protocol", "SASL_PLAINTEXT")
            .set("sasl.mechanisms", "SCRAM-SHA-256")
            .set("sasl.username", username)
            .set("sasl.password", password)
            .create()
            .expect("Producer creation failed");

        Self {
            producer,
            topic: topic.to_string(),
        }
    }
}

#[async_trait]
impl EventRepository for RedpandaEventRepository {
    async fn persist_and_broadcast(&self, event: DomainEvent) {
        let payload = serde_json::to_string(&event).unwrap();

        let record = FutureRecord::to(&self.topic)
            .payload(&payload)
            .key("simple-key");
        //.key(&format!("{:?}", event)); // Use a unique key for partitioning

        let _ = self
            .producer
            .send(record, tokio::time::Duration::from_secs(0))
            .await;
        println!("📮 [Redpanda] Event streamed to topic: {}", self.topic);
    }
}

pub async fn start_redpanda_worker(
    brokers: &str,
    username: &str,
    password: &str,
    group_id: &str,
    topic: &str,
    handlers: Vec<Arc<dyn EventHandler>>,
) {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("group.id", group_id)
        .set("security.protocol", "SASL_PLAINTEXT")
        .set("sasl.mechanisms", "SCRAM-SHA-256")
        .set("sasl.username", username)
        .set("sasl.password", password)
        .set("auto.offset.reset", "earliest")
        .set("enable.auto.commit", "true")
        .set("session.timeout.ms", "6000")
        .set("enable.partition.eof", "false")
        .create()
        .expect("Consumer creation failed");

    consumer
        .subscribe(&[topic])
        .expect("Can't subscribe to topic");

    println!("🎧 [Worker] Listening to Redpanda stream...");

    loop {
        match consumer.recv().await {
            Err(e) => eprintln!("Kafka error: {}", e),
            Ok(m) => {
                let payload = match m.payload_view::<str>() {
                    None => continue,
                    Some(Ok(s)) => s,
                    Some(Err(_)) => continue,
                };

                if let Ok(event) = serde_json::from_str::<DomainEvent>(payload) {
                    for handler in &handlers {
                        handler.handle(event.clone()).await;
                    }
                }
            }
        }
    }
}

///////separating tthe inmemmory

pub struct InMemoryEventRepository {
    sender: broadcast::Sender<DomainEvent>,
}
impl InMemoryEventRepository {
    pub fn new(sender: broadcast::Sender<DomainEvent>) -> Self {
        Self { sender }
    }
}
#[async_trait]
impl EventRepository for InMemoryEventRepository {
    async fn persist_and_broadcast(&self, event: DomainEvent) {
        println!("📝 [Repo] Persisting to Bus: {:?}", event);
        let _ = self.sender.send(event);
    }
}

pub struct MockLlm;
#[async_trait]
impl LlmClient for MockLlm {
    async fn generate(&self, prompt: &str) -> String {
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        format!("🤖 Simulated AI Response to: '{}'", prompt)
    }
}

pub struct MockMatrix;
#[async_trait]
impl MatrixAdapter for MockMatrix {
    async fn send_message(&self, room_id: &str, body: &str) {
        println!("🚀 [Matrix API] Sending reply to {}: {}", room_id, body);
    }
}
