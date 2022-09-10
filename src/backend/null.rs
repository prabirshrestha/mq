use async_trait::async_trait;
use fnv::FnvHashMap;

use crate::{producer::Producer, BoxedJobRunner, Consumer, ConsumerQueueOptions, Job, MqResult};

pub struct NullProducer {}

impl NullProducer {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Producer for NullProducer {
    async fn enqueue(&mut self, _job: Job) -> MqResult<()> {
        Ok(())
    }
}

pub struct NullConsumer {
    callbacks: FnvHashMap<String, BoxedJobRunner>,
}

impl Default for NullConsumer {
    fn default() -> Self {
        Self {
            callbacks: Default::default(),
        }
    }
}

impl NullConsumer {
    pub fn new() -> Self {
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

    fn run<I>(&mut self, _queues: I) -> MqResult<()>
    where
        I: Iterator<Item = ConsumerQueueOptions>,
    {
        loop {}
    }
}
