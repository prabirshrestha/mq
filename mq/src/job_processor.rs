use async_trait::async_trait;
use serde_json::Value;

use crate::{Error, Job};

#[async_trait]
pub trait JobProcessor: Send + Sync {
    async fn poll_next_job(&self, queues: &[&str]) -> Result<Option<Job>, Error>;
    async fn complete_job_with_success(
        &self,
        queue: &str,
        kind: &str,
        id: &str,
    ) -> Result<(), Error>;
    async fn fail_job(&self, queue: &str, kind: &str, id: &str, reason: Value)
        -> Result<(), Error>;
}
