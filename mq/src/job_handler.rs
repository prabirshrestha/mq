use std::future::Future;

use async_trait::async_trait;

use crate::{Context, Error, JobResult};

#[async_trait]
pub trait JobHandler: Send + Sync {
    fn queue(&self) -> &str {
        "default"
    }

    fn kind(&self) -> &str;

    async fn handle(&self, ctx: Context) -> Result<JobResult, Error>;
}

#[async_trait]
impl<F, Fut> JobHandler for (&str, &str, F)
where
    F: Fn(Context) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<JobResult, Error>> + Send,
{
    fn queue(&self) -> &str {
        self.0
    }

    fn kind(&self) -> &str {
        self.1
    }

    async fn handle(&self, ctx: Context) -> Result<JobResult, Error> {
        self.2(ctx).await
    }
}

#[async_trait]
impl<F, Fut> JobHandler for (&str, F)
where
    F: Fn(Context) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<JobResult, Error>> + Send,
{
    fn kind(&self) -> &str {
        self.0
    }

    async fn handle(&self, ctx: Context) -> Result<JobResult, Error> {
        self.1(ctx).await
    }
}
