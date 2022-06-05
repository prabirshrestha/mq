pub mod broker;

pub use async_trait::async_trait;

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

#[derive(Debug)]
pub struct MqMessageBytes(Vec<u8>);

#[derive(Debug)]
pub struct MqMessage<T: TryFrom<MqMessageBytes>> {
    pub id: String,
    pub data: T,
}

impl MqMessageBytes {
    pub fn into_bytes(self) -> Vec<u8> {
        self.0
    }
}

impl TryFrom<MqMessageBytes> for String {
    type Error = Vec<u8>;

    fn try_from(bytes: MqMessageBytes) -> Result<Self, Self::Error> {
        String::from_utf8(bytes.0).map_err(|e| e.into_bytes())
    }
}

impl TryFrom<MqMessageBytes> for Vec<u8> {
    type Error = Vec<u8>;

    fn try_from(bytes: MqMessageBytes) -> Result<Self, Vec<u8>> {
        Ok(bytes.0)
    }
}

impl From<String> for MqMessageBytes {
    fn from(t: String) -> MqMessageBytes {
        MqMessageBytes(t.into())
    }
}

impl From<&str> for MqMessageBytes {
    fn from(t: &str) -> MqMessageBytes {
        MqMessageBytes(t.into())
    }
}

impl From<Vec<u8>> for MqMessageBytes {
    fn from(t: Vec<u8>) -> MqMessageBytes {
        MqMessageBytes(t)
    }
}

impl From<&[u8]> for MqMessageBytes {
    fn from(t: &[u8]) -> MqMessageBytes {
        MqMessageBytes(t.into())
    }
}
