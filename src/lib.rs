pub use async_trait::async_trait;

#[async_trait]
pub trait MessageQueue {
    async fn create_queue(queue_name: &str);
    async fn delete_queue(queue_name: &str);
}
