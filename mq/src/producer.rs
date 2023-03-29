use async_trait::async_trait;

use crate::{Error, Job};

#[async_trait]
pub trait Producer: Send + Sync {
    async fn publish(&self, job: Job) -> Result<(), Error>;
}
