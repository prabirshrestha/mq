use std::time::Duration;

use serde::de::DeserializeOwned;
use serde_json::Value;
use tokio_util::sync::CancellationToken;

use crate::{Error, Job};

#[derive(Debug)]
pub struct Context {
    job: Job,
    cancellation_token: CancellationToken,
}

impl Context {
    pub fn new(job: Job, cancellation_token: CancellationToken) -> Self {
        Self {
            job,
            cancellation_token,
        }
    }

    pub fn id(&self) -> &str {
        self.job.id()
    }

    pub fn queue(&self) -> &str {
        self.job.queue()
    }

    pub fn kind(&self) -> &str {
        self.job.kind()
    }

    pub fn payload(&self) -> &Value {
        self.job.payload()
    }

    pub fn deserialize<T>(self) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        Ok(serde_json::from_value::<T>(self.job.payload)?)
    }

    pub fn error_reason(&self) -> &Option<Value> {
        self.job.error_reason()
    }

    pub fn lease_time(&self) -> &Duration {
        self.job.lease_time()
    }

    pub fn cancellation_token(&self) -> &CancellationToken {
        &self.cancellation_token
    }
}
