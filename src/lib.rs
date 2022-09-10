use std::sync::Arc;

mod job;
pub use job::*;

pub use async_trait::async_trait;
pub use chrono::{DateTime, Duration, Utc};
use fnv::FnvHashMap;

const DEFAULT_QUEUE_NAME: &'static str = "default";

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

pub struct ConsumerQueueOptions {
    pub queue: String,
    pub priority: i8,
}

impl ConsumerQueueOptions {
    pub fn new<K>(queue: K, priority: i8) -> Self
    where
        K: Into<String>,
    {
        Self {
            queue: queue.into(),
            priority,
        }
    }

    pub fn with_queue(mut self, queue: String) -> Self {
        if queue.is_empty() {
            self.queue = DEFAULT_QUEUE_NAME.into();
        } else {
            self.queue = queue;
        }
        self
    }

    pub fn with_priority(mut self, priority: i8) -> Self {
        self.priority = priority;
        self
    }
}

impl Default for ConsumerQueueOptions {
    fn default() -> Self {
        ConsumerQueueOptions::new(DEFAULT_QUEUE_NAME, 1)
    }
}

type JobRunner = dyn Fn(Job) -> MqResult<()> + Send + Sync;
type BoxedJobRunner = Box<JobRunner>;

pub trait Consumer {
    fn register<K, H>(&mut self, kind: K, handler: H) -> &mut Self
    where
        K: Into<String>,
        H: Fn(Job) -> MqResult<()> + Send + Sync + 'static;

    fn run<I>(&mut self, queues: I) -> MqResult<()>
    where
        I: Iterator<Item = ConsumerQueueOptions>;
}

pub struct NullConsumer {
    callbacks: FnvHashMap<String, BoxedJobRunner>,
    workers: usize,
}

impl Default for NullConsumer {
    fn default() -> Self {
        Self {
            callbacks: Default::default(),
            workers: 1,
        }
    }
}

impl NullConsumer {
    fn new() -> Self {
        Self::default()
    }
}

impl Consumer for NullConsumer {
    fn register<K, H>(&mut self, kind: K, handler: H) -> &mut Self
    where
        K: Into<String>,
        H: Fn(Job) -> MqResult<()> + Send + Sync + 'static,
    {
        self.callbacks.insert(kind.into(), Box::new(handler));
        self
    }

    fn run<I>(&mut self, queues: I) -> MqResult<()>
    where
        I: Iterator<Item = ConsumerQueueOptions>,
    {
        loop {}
    }
}

async fn hello() {
    let mut p = NullProducer::new();

    let j1 = Job::new("foo", "hello")
        .on_queue("default".into())
        .schedule_immediate();

    p.enqueue(j1).await.unwrap();

    let mut c = NullConsumer::new();
    c.register("foo", |j| {
        println!("foo: {}", j.id);
        Ok(())
    })
    .register("bar", |j| {
        println!("bar: {}", j.id);
        Ok(())
    });

    c.run([ConsumerQueueOptions::new("default", 1)].into_iter())
        .unwrap();
}

#[derive(thiserror::Error, Debug)]
pub enum MqError {
    #[error("Cannot decode message.")]
    CannotDecodeMessage(Vec<u8>),
}

pub type MqResult<T> = Result<T, MqError>;

#[derive(Debug)]
pub struct MqMessageBytes(Vec<u8>);

impl MqMessageBytes {
    pub fn into_bytes(self) -> Vec<u8> {
        self.0
    }
}

impl TryFrom<MqMessageBytes> for String {
    type Error = Vec<u8>;

    fn try_from(bytes: MqMessageBytes) -> Result<Self, Self::Error> {
        String::from_utf8(bytes.0).map_err(|e| e.into_bytes())
    }
}

impl TryFrom<MqMessageBytes> for Vec<u8> {
    type Error = Vec<u8>;

    fn try_from(bytes: MqMessageBytes) -> Result<Self, Vec<u8>> {
        Ok(bytes.0)
    }
}

impl From<String> for MqMessageBytes {
    fn from(t: String) -> MqMessageBytes {
        MqMessageBytes(t.into())
    }
}

impl From<&str> for MqMessageBytes {
    fn from(t: &str) -> MqMessageBytes {
        MqMessageBytes(t.into())
    }
}

impl From<Vec<u8>> for MqMessageBytes {
    fn from(t: Vec<u8>) -> MqMessageBytes {
        MqMessageBytes(t)
    }
}

impl From<&[u8]> for MqMessageBytes {
    fn from(t: &[u8]) -> MqMessageBytes {
        MqMessageBytes(t.into())
    }
}
