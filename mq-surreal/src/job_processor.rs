use std::sync::Arc;

use async_trait::async_trait;
use mq::{Error, Job, JobProcessor};
use serde_json::Value;
use surrealdb::{engine::any::Any, Surreal};
use time::OffsetDateTime;

use crate::error::convert_surrealdb_error;

pub struct SurrealJobProcessor {
    db: Arc<Surreal<Any>>,
    table: String,
}

impl SurrealJobProcessor {
    pub fn new<T: Into<String>>(db: Arc<Surreal<Any>>, table: T) -> Self {
        Self {
            db,
            table: table.into(),
        }
    }
}

#[async_trait]
impl JobProcessor for SurrealJobProcessor {
    async fn poll_next_job(&self, queues: &[&str]) -> Result<Option<Job>, Error> {
        let mut result = self
            .db
            .query(
                r#"
            UPDATE (
                SELECT value id
                FROM (
                    SELECT * FROM type::table($table)
                    WHERE
                        attempts<max_attempts
                        AND scheduled_at<=$now
                        AND (
                            locked_at=null
                            OR (
                                time::unix(locked_at)<time::unix($now)-lease_time
                            )
                        )
                        AND queue IN $queues
                    ORDER by updated_at ASC
                    LIMIT 1
                )
            )
            SET
                locked_at=$now,
                updated_at=$now
            RETURN
                meta::id(id) as id,
                *"#,
            )
            .bind(("table", &self.table))
            .bind(("queues", queues))
            .bind(("now", OffsetDateTime::now_utc()))
            .await
            .map_err(convert_surrealdb_error)?
            .check()
            .map_err(convert_surrealdb_error)?;

        Ok(result.take(0).map_err(convert_surrealdb_error)?)
    }

    async fn complete_job_with_success(
        &self,
        queue: &str,
        kind: &str,
        id: &str,
    ) -> Result<(), Error> {
        self.db
            .query(r#"DELETE type::thing($table, $id) WHERE queue=$queue AND kind=$kind"#)
            .bind(("table", &self.table))
            .bind(("id", id))
            .bind(("queue", queue))
            .bind(("kind", kind))
            .await
            .map_err(convert_surrealdb_error)?
            .check()
            .map_err(convert_surrealdb_error)?;

        Ok(())
    }

    async fn fail_job(
        &self,
        queue: &str,
        kind: &str,
        id: &str,
        reason: Value,
    ) -> Result<(), Error> {
        self.db
            .query(
                r#"
            UPDATE type::thing($table, $id)
            SET
                locked_at=null,
                updated_at=$now,
                attempts=attempts+1,
                error_reason=$error_reason
            WHERE
                queue=$queue AND kind=$kind
            "#,
            )
            .bind(("table", &self.table))
            .bind(("id", id))
            .bind(("queue", queue))
            .bind(("kind", kind))
            .bind(("error_reason", reason))
            .bind(("now", OffsetDateTime::now_utc()))
            .await
            .map_err(convert_surrealdb_error)?
            .check()
            .map_err(convert_surrealdb_error)?;

        Ok(())
    }
}
