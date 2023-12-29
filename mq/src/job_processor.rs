use async_trait::async_trait;
use serde_json::Value;

use crate::{Error, Job};

#[async_trait]
pub trait JobProcessor: Send + Sync {
    /// Poll next job. If there are no jobs Ok(None) is returned.
    /// ### Priority
    ///
    /// Higher priority will be polled first.
    async fn poll_next_job(&self, queues: &[&str]) -> Result<Option<Job>, Error>;

    /// Complete the job with success.
    async fn complete_job_with_success(
        &self,
        queue: &str,
        kind: &str,
        id: &str,
    ) -> Result<(), Error>;

    /// Complete the job with cancel.
    async fn complete_job_with_cancelled(
        &self,
        queue: &str,
        kind: &str,
        id: &str,
        message: Option<String>,
    ) -> Result<(), Error>;

    /// Fail the job.
    async fn fail_job(&self, queue: &str, kind: &str, id: &str, reason: Value)
        -> Result<(), Error>;
}
