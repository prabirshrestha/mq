use fnv::FnvHashMap;

use crate::{Job, MqResult, DEFAULT_QUEUE_NAME};

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

    fn run<I>(&mut self, queues: I) -> MqResult<()>
    where
        I: Iterator<Item = ConsumerQueueOptions>,
    {
        loop {}
    }
}
