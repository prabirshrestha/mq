use std::sync::Arc;

use async_trait::async_trait;
use mq::{Error, Job, Producer};
use surrealdb::{engine::any::Any, Surreal};
use time::OffsetDateTime;

use crate::error::convert_surrealdb_error;

pub struct SurrealProducer {
    db: Arc<Surreal<Any>>,
    table: String,
}

impl SurrealProducer {
    pub fn new<T: Into<String>>(db: Arc<Surreal<Any>>, table: T) -> Self {
        Self {
            db,
            table: table.into(),
        }
    }
}

#[async_trait]
impl Producer for SurrealProducer {
    async fn publish(&self, job: Job) -> Result<(), Error> {
        self.db
            .query(
                r#"CREATE type::thing($table, $id)
                SET created_at=$created_at,
                    scheduled_at=$scheduled_at,
                    queue=$queue,
                    kind=$kind,
                    payload=$payload,
                    attempts=$attempts,
                    max_attempts=$max_attempts,
                    lease_time=$lease_time,
                    error_reason=null"#,
            )
            .bind(("table", &self.table))
            .bind(("id", job.id()))
            .bind(("queue", job.queue()))
            .bind(("kind", job.kind()))
            .bind(("payload", job.payload()))
            .bind(("created_at", OffsetDateTime::now_utc()))
            .bind((
                "scheduled_at",
                job.scheduled_at()
                    .unwrap_or_else(|| (OffsetDateTime::now_utc())),
            ))
            .bind(("attempts", job.attempts()))
            .bind(("max_attempts", job.max_attempts()))
            .bind(("lease_time", job.lease_time().as_secs()))
            .await
            .map_err(convert_surrealdb_error)?
            .check()
            .map_err(convert_surrealdb_error)?;

        Ok(())
    }
}
