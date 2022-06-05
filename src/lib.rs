pub mod broker;

pub use async_trait::async_trait;

#[derive(Debug)]
pub struct MqMessageBytes(Vec<u8>);

#[derive(Debug)]
pub struct MqMessage<T: TryFrom<MqMessageBytes>> {
    pub id: String,
    pub data: T,
}

#[async_trait]
pub trait MessageQueue {
    async fn create_queue(&mut self, queue_name: &str) -> MqResult<()>;
    async fn delete_queue(&mut self, queue_name: &str) -> MqResult<()>;

    async fn dequeue<E: TryFrom<MqMessageBytes, Error = Vec<u8>>>(
        &mut self,
        queue_name: &str,
        visiblity_timeout_in_ms: Option<u64>,
    ) -> MqResult<Option<MqMessage<E>>>;

    async fn enqueue<M: Into<MqMessageBytes> + Send>(
        &mut self,
        queue_name: &str,
        message: M,
    ) -> MqResult<String>;

    async fn ack(&mut self, message_id: &str) -> MqResult<()>;

    async fn nack(&mut self, message_id: &str) -> MqResult<()>;

    async fn ping(&mut self) -> MqResult<()>;
}

#[derive(thiserror::Error, Debug)]
pub enum MqError {
    #[error("Cannot decode message.")]
    CannotDecodeMessage(Vec<u8>),
}

pub type MqResult<T> = Result<T, MqError>;
