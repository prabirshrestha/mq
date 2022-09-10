mod job;
pub use job::*;

mod producer;
pub use producer::*;

mod consumer;
pub use consumer::*;

pub use async_trait::async_trait;
pub use chrono::{DateTime, Duration, Utc};
use producer::{NullProducer, Producer};

const DEFAULT_QUEUE_NAME: &'static str = "default";

async fn hello() {
    let mut p = NullProducer::new();

    let j1 = Job::new("foo", "hello")
        .on_queue("default".into())
        .schedule_immediate();

    p.enqueue(j1).await.unwrap();

    let mut c = NullConsumer::new();
    c.register("foo", |j| {
        println!("foo: {}", j.id());
        Ok(())
    })
    .register("bar", |j| {
        println!("bar: {}", j.id());
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
