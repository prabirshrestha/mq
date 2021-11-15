pub use async_trait::async_trait;

#[async_trait]
pub trait MessageQueue {
    async fn create_queue(&mut self, queue_name: &str);
    async fn delete_queue(&mut self, queue_name: &str);

    async fn consume(&mut self, queue_name: &str);
}
