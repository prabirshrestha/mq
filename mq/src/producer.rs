use async_trait::async_trait;

use crate::{Error, Job};

#[async_trait]
pub trait Producer: Send + Sync {
    async fn publish(&self, job: Job) -> Result<(), Error>;
    async fn exists(&self, queue: &str, kind: &str, id: &str) -> Result<bool, Error>;
    async fn cancel_by_id(&self, queue: &str, kind: &str, id: &str) -> Result<(), Error>;
    async fn cancel_by_unique_key(&self, queue: &str, kind: &str, key: &str) -> Result<(), Error>;
}
