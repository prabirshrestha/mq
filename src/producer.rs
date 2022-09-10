use async_trait::async_trait;

use crate::{Job, MqResult};

#[async_trait]
pub trait Producer {
    async fn enqueue(&mut self, job: Job) -> MqResult<()>;
}
