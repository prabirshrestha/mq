use crate::{MessageQueue, MqError, MqMessage, MqMessageBytes, MqResult};
pub use async_trait::async_trait;

pub struct BlackholeMessageBroker {}

impl BlackholeMessageBroker {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl MessageQueue for BlackholeMessageBroker {
    async fn create_queue(&mut self, _queue_name: &str) -> MqResult<()> {
        Ok(())
    }

    async fn delete_queue(&mut self, _queue_name: &str) -> MqResult<()> {
        Ok(())
    }

    async fn dequeue<E: TryFrom<MqMessageBytes, Error = Vec<u8>>>(
        &mut self,
        _queue_name: &str,
        _visiblity_timeout_in_ms: Option<u64>,
    ) -> MqResult<Option<MqMessage<E>>> {
        let data = E::try_from(MqMessageBytes(vec![])).map_err(MqError::CannotDecodeMessage)?;
        Ok(Some(MqMessage {
            id: String::from("blackholeid"),
            data,
        }))
    }

    async fn enqueue<M: Into<MqMessageBytes> + Send>(
        &mut self,
        _queue_name: &str,
        _message: M,
    ) -> MqResult<String> {
        Ok(String::from("blackholeid"))
    }

    async fn ack(&mut self, _message_id: &str) -> MqResult<()> {
        Ok(())
    }

    async fn nack(&mut self, _message_id: &str) -> MqResult<()> {
        Ok(())
    }

    async fn ping(&mut self) -> MqResult<()> {
        Ok(())
    }
}
