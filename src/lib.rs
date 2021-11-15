pub mod broker;

pub use async_trait::async_trait;

#[async_trait]
pub trait MessageQueue {
    async fn create_queue(&mut self, queue_name: &str);
    async fn delete_queue(&mut self, queue_name: &str);

    async fn dequeue(&mut self, queue_name: &str);
    async fn enqueue(&mut self, queue_name: &str);

    async fn ack(&mut self, message_id: &str);
    async fn nack(&mut self, message_id: &str);

    async fn ping(&mut self);
}
