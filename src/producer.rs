use async_trait::async_trait;

use crate::{Job, MqResult};

#[async_trait]
pub trait Producer {
    async fn enqueue(&mut self, job: Job) -> MqResult<()>;
}

pub struct NullProducer {}

impl NullProducer {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Producer for NullProducer {
    async fn enqueue(&mut self, job: Job) -> MqResult<()> {
        Ok(())
    }
}
