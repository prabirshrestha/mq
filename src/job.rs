use crate::{MqMessageBytes, DEFAULT_QUEUE_NAME};
use chrono::{DateTime, Duration, Utc};

#[derive(Debug)]
pub struct Job {
    id: String,
    kind: String,
    data: MqMessageBytes,
    queue: String,
    at: Option<DateTime<Utc>>,
}

impl Job {
    pub fn new<K, D>(kind: K, data: D) -> Self
    where
        K: Into<String>,
        D: Into<MqMessageBytes>,
    {
        Self {
            id: xid::new().to_string(),
            kind: kind.into(),
            data: data.into(),
            queue: DEFAULT_QUEUE_NAME.into(),
            at: None,
        }
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

    pub fn data(&self) -> &MqMessageBytes {
        &self.data
    }

    pub fn with_id(mut self, id: String) -> Self {
        self.id = id;
        self
    }

    pub fn on_queue(mut self, queue: String) -> Self {
        if queue.is_empty() {
            self.queue = DEFAULT_QUEUE_NAME.into();
        } else {
            self.queue = queue;
        }
        self
    }

    pub fn schedule_at(mut self, at: DateTime<Utc>) -> Self {
        self.at = Some(at);
        self
    }

    pub fn schedule_in(mut self, duration: Duration) -> Self {
        self.at = Some(Utc::now() + duration);
        self
    }

    pub fn schedule_immediate(mut self) -> Self {
        self.at = None;
        self
    }

    pub fn with_data(mut self, data: MqMessageBytes) -> Self {
        self.data = data;
        self
    }
}
