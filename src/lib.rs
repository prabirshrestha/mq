pub use async_trait::async_trait;
pub use chrono::{DateTime, Duration, Utc};

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

#[async_trait]
pub trait Consumer {}

async fn hello() {
    let mut p = NullProducer::new();

    let j1 = Job::new("foo", "hello")
        .on_queue("default".into())
        .schedule_immediate();

    p.enqueue(j1).await.unwrap();
}

#[async_trait]
pub trait MqManagement {
    async fn create_queue(&mut self, queue_name: &str) -> MqResult<()>;
    async fn delete_queue(&mut self, queue_name: &str) -> MqResult<()>;
}

#[async_trait]
pub trait MqConsumer {
    // async fn dequeue<E: TryFrom<MqMessageBytes, Error = Vec<u8>>>(
    //     &mut self,
    //     queue_name: &str,
    //     visiblity_timeout_in_ms: Option<u64>,
    // ) -> MqResult<Option<MqMessage<E>>>;

    async fn ack(&mut self, message_id: &str) -> MqResult<()>;

    async fn nack(&mut self, message_id: &str) -> MqResult<()>;

    async fn ping(&mut self) -> MqResult<()>;
}

#[async_trait]
pub trait MessageQueue {
    // async fn schedule_at<M: Into<MqMessageBytes> + Send>(
    //     &mut self,
    //     queue_name: &str,
    //     message: M,
    //     scheduled_at: DateTime<Utc>,
    // ) -> MqResult<String>;

    // async fn schedule<M: Into<MqMessageBytes> + Send>(
    //     &mut self,
    //     queue_name: &str,
    //     message: M,
    //     job_create_options: JobCreateOptions,
    // ) -> MqResult<String> {
    //     Ok(self.schedule_at(queue_name, message, Utc::now()).await?)
    // }

    // async fn schedule_in<M: Into<MqMessageBytes> + Send>(
    //     &mut self,
    //     queue_name: &str,
    //     message: M,
    //     schedule_in: Duration,
    // ) -> MqResult<String> {
    //     Ok(self
    //         .schedule_at(queue_name, message, Utc::now() + schedule_in)
    //         .await?)
    // }
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
