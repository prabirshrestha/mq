use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{serde_as, DurationSeconds};
use time::OffsetDateTime;

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct Job {
    id: String,
    queue: String,
    kind: String,
    #[serde(with = "time::serde::iso8601::option")]
    created_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::iso8601::option")]
    scheduled_at: Option<OffsetDateTime>,
    payload: Value,
    error_reason: Option<Value>,
    attempts: u16,
    max_attempts: u16,
    #[serde_as(as = "DurationSeconds<u64>")]
    lease_time: Duration,
}

impl Job {
    pub fn new<K, P>(kind: K, payload: P) -> Self
    where
        K: Into<String>,
        P: Into<Value>,
    {
        Self {
            id: xid::new().to_string(),
            queue: "default".into(),
            kind: kind.into(),
            payload: payload.into(),
            error_reason: None,
            created_at: None,
            scheduled_at: None,
            attempts: 0,
            max_attempts: 3,
            lease_time: Duration::from_secs(30),
        }
    }

    pub fn with_queue<S: Into<String>>(mut self, queue: S) -> Self {
        self.queue = queue.into();
        self
    }

    pub fn with_new_id<S: Into<String>>(mut self) -> Self {
        self.id = xid::new().to_string();
        self
    }

    pub fn with_id<S: Into<String>>(mut self, id: S) -> Self {
        self.id = id.into();
        self
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn queue(&self) -> &str {
        &self.queue
    }

    pub fn kind(&self) -> &str {
        &self.kind
    }

    pub fn created_at(&self) -> &Option<OffsetDateTime> {
        &self.created_at
    }

    pub fn payload(&self) -> &Value {
        &self.payload
    }

    pub fn with_attempts(mut self, attempts: u16) -> Self {
        self.attempts = attempts;
        self
    }

    pub fn attempts(&self) -> u16 {
        self.attempts
    }

    pub fn with_max_attempts(mut self, max_attempts: u16) -> Self {
        self.max_attempts = max_attempts;
        self
    }

    pub fn max_attempts(&self) -> u16 {
        self.max_attempts
    }

    pub fn with_lease_time(mut self, lease_time: Duration) -> Self {
        self.lease_time = lease_time;
        self
    }

    pub fn lease_time(&self) -> &Duration {
        &self.lease_time
    }

    pub fn with_schedule_at(mut self, date_time: OffsetDateTime) -> Self {
        self.scheduled_at = Some(date_time);
        self
    }

    pub fn with_schedule_now(mut self) -> Self {
        self.scheduled_at = None;
        self
    }

    pub fn with_schedule_in(mut self, duration: Duration) -> Self {
        self.scheduled_at = Some(OffsetDateTime::now_utc() + duration);
        self
    }

    pub fn scheduled_at(&self) -> &Option<OffsetDateTime> {
        &self.scheduled_at
    }

    pub fn with_error_reason(mut self, error_reason: Option<Value>) -> Self {
        self.error_reason = error_reason;
        self
    }

    pub fn error_reason(&self) -> &Option<Value> {
        &self.error_reason
    }
}
