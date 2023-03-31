use std::{future::ready, pin::pin, time::Duration};

use crate::{Consumer, Context, Error, JobProcessor};
use futures::{stream, FutureExt, StreamExt, TryStreamExt};
use serde_json::json;
use tokio_util::sync::CancellationToken;
use tracing::debug;

pub struct Worker {
    consumer: Consumer,
    cancellation_token: CancellationToken,
    concurrency: Option<usize>,
    poll_interval: Option<u64>,
}

impl Worker {
    pub fn new(consumer: Consumer) -> Self {
        Self {
            cancellation_token: CancellationToken::new(),
            consumer,
            concurrency: None,
            poll_interval: Some(3000),
        }
    }

    pub fn concurrency(&self) -> Option<usize> {
        self.concurrency
    }

    pub fn with_concurrency(mut self, concurrency: Option<usize>) -> Self {
        self.concurrency = concurrency;
        self
    }

    pub fn poll_interval(&self) -> &Option<u64> {
        &self.poll_interval
    }

    pub fn with_poll_interval(mut self, poll_interval: Option<u64>) -> Self {
        self.poll_interval = poll_interval;
        self
    }

    pub fn cancellation_token(&self) -> &CancellationToken {
        &self.cancellation_token
    }

    pub fn with_cancellation_token(mut self, cancellation_token: CancellationToken) -> Self {
        self.cancellation_token = cancellation_token;
        self
    }

    pub async fn run(self, job_processor: impl JobProcessor) -> Result<(), Error> {
        let interval =
            tokio::time::interval(Duration::from_millis(self.poll_interval.unwrap_or(3000)));

        let queues: Vec<&str> = self.consumer.handlers().keys().map(|k| &**k).collect();

        let ct = pin!(self.cancellation_token.cancelled().fuse());

        let job_stream = stream::unfold((interval, ct), |mut f| async {
            tokio::select! {
                 _ = (&mut f.0).tick() => Some((StreamSource::Polling, f)),
                _ = &mut f.1 => None,
            }
        });

        job_stream
            .then(|source| self.process_next_job(source, &job_processor, &queues))
            .try_for_each_concurrent(self.concurrency, |_| ready(Ok(())))
            .await?;

        Ok(())
    }

    async fn process_next_job<T: JobProcessor>(
        &self,
        _source: StreamSource,
        job_processor: &T,
        queues: &[&str],
    ) -> Result<(), Error> {
        loop {
            match job_processor.poll_next_job(queues).await? {
                Some(job) => {
                    // TODO: Probably want to filter via queues+kind instead of just queue. But for now
                    // using queues so it is compatible with other backends.
                    let handler = self
                        .consumer
                        .handlers()
                        .get(job.queue())
                        .unwrap()
                        .get(job.kind())
                        .unwrap();

                    let id = job.id().to_string();
                    let ctx = Context::new(job, self.cancellation_token.clone());

                    match handler.handle(ctx).await {
                        Ok(result) => match result {
                            crate::JobResult::CompleteWithSuccess => {
                                job_processor
                                    .complete_job_with_success(handler.queue(), handler.kind(), &id)
                                    .await?;
                            }
                        },
                        Err(e) => {
                            job_processor
                                .fail_job(
                                    handler.queue(),
                                    handler.kind(),
                                    &id,
                                    json!(e.to_string()),
                                )
                                .await?;
                        }
                    }
                }
                None => {
                    debug!("No new jobs found");
                    break;
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
enum StreamSource {
    Polling,
    Listener,
}
