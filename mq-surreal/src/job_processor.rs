use std::sync::Arc;

use async_trait::async_trait;
use mq::{Error, Job, JobProcessor};
use serde_json::Value;
use surrealdb::{engine::any::Any, Surreal};

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
        loop {
            let mut result = self
                .db
                .query(
                    r#"
                LET $now=time::now();
                LET $unix_now=time::unix($now);
                SELECT meta::id(id) as id, * 
                FROM type::table($table)
                WHERE
                    attempts<max_attempts
                    AND scheduled_at<=$now
                    AND (
                        locked_at=null
                        OR (
                            time::unix(locked_at)<$unix_now-lease_time
                        )
                    )
                    AND queue IN $queues
                ORDER BY created_at ASC
                LIMIT 1"#,
                )
                .bind(("table", &self.table))
                .bind(("queues", queues))
                .await
                .map_err(convert_surrealdb_error)?
                .check()
                .map_err(convert_surrealdb_error)?;

            let job: Option<Job> = result.take(2).map_err(convert_surrealdb_error)?;

            match job {
                Some(job) => {
                    let mut result = self
                        .db
                        .query(
                            r#"
                    LET $now=time::now();
                    LET $unix_now=time::unix($now);
                    UPDATE type::thing($table, $id)
                    SET
                        locked_at=time::now(),
                        attempts=attempts+1
                    WHERE
                        attempts<max_attempts
                        AND scheduled_at<=$now
                        AND (
                            locked_at=null
                            OR (
                                time::unix(locked_at)<$unix_now-lease_time
                            )
                        )
                    RETURN meta::id(id) as id, *;
                    "#,
                        )
                        .bind(("table", &self.table))
                        .bind(("id", job.id()))
                        .await
                        .map_err(convert_surrealdb_error)?
                        .check()
                        .map_err(convert_surrealdb_error)?;

                    let job = result.take(2).map_err(convert_surrealdb_error)?;
                    match job {
                        Some(job) => return Ok(Some(job)),
                        None => continue,
                    }
                }
                None => return Ok(None),
            }
        }
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
            .await
            .map_err(convert_surrealdb_error)?
            .check()
            .map_err(convert_surrealdb_error)?;

        Ok(())
    }
}
