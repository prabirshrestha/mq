use std::{str::FromStr, sync::Arc};

use async_trait::async_trait;
use mq::{Error, Job, JobProcessor};
use serde_json::Value;
use surrealdb::{engine::any::Any, sql::Datetime, Surreal};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

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
                            locked_at=NONE
                            OR (
                                time::unix(locked_at)<time::unix($now)-lease_time
                            )
                        )
                        AND queue IN $queues
                    ORDER by priority DESC, updated_at ASC
                    LIMIT 1
                )
            )
            SET
                attempts=attempts+1,
                locked_at=$now,
                updated_at=$now
            RETURN
                record::id(id) as id,
                *"#,
            )
            .bind(("table", self.table.clone()))
            .bind((
                "queues",
                queues
                    .into_iter()
                    .map(|q| q.to_string())
                    .collect::<Vec<String>>(),
            ))
            .bind((
                "now",
                Datetime::from_str(&OffsetDateTime::now_utc().format(&Rfc3339).unwrap()).unwrap(),
            ))
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
            .bind(("table", self.table.to_owned()))
            .bind(("id", id.to_owned()))
            .bind(("queue", queue.to_owned()))
            .bind(("kind", kind.to_owned()))
            .await
            .map_err(convert_surrealdb_error)?
            .check()
            .map_err(convert_surrealdb_error)?;

        Ok(())
    }

    async fn complete_job_with_cancelled(
        &self,
        queue: &str,
        kind: &str,
        id: &str,
        _message: Option<String>,
    ) -> Result<(), Error> {
        self.db
            .query(r#"DELETE type::thing($table, $id) WHERE queue=$queue AND kind=$kind"#)
            .bind(("table", self.table.clone()))
            .bind(("id", id.to_owned()))
            .bind(("queue", queue.to_owned()))
            .bind(("kind", kind.to_owned()))
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
                locked_at=NONE,
                updated_at=$now,
                error_reason=$error_reason
            WHERE
                queue=$queue AND kind=$kind
            "#,
            )
            .bind(("table", self.table.clone()))
            .bind(("id", id.to_owned()))
            .bind(("queue", queue.to_owned()))
            .bind(("kind", kind.to_owned()))
            .bind(("error_reason", reason))
            .bind((
                "now",
                Datetime::from_str(&OffsetDateTime::now_utc().format(&Rfc3339).unwrap()).unwrap(),
            ))
            .await
            .map_err(convert_surrealdb_error)?
            .check()
            .map_err(convert_surrealdb_error)?;

        Ok(())
    }
}
