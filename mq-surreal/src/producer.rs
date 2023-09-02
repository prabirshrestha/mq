use std::{str::FromStr, sync::Arc};

use async_trait::async_trait;
use mq::{Error, Job, Producer};
use surrealdb::{engine::any::Any, sql::Datetime, Surreal};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

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
        let now = Datetime::from_str(&OffsetDateTime::now_utc().format(&Rfc3339).unwrap()).unwrap();

        self.db
            .query(
                r#"
                BEGIN TRANSACTION;

                LET $allow = IF $unique_key == NONE THEN
                    true
                ELSE
                    count((
                        SELECT * FROM queue
                        WHERE
                            queue=$queue
                            AND kind=$kind
                            AND unique_key=$unique_key
                            AND attempts<max_attempts
                    )) == 0
                END;

                IF $allow THEN
                    CREATE type::thing($table, $id)
                    SET created_at=$now,
                        updated_at=$now,
                        scheduled_at=$scheduled_at,
                        locked_at=NONE,
                        queue=$queue,
                        kind=$kind,
                        payload=$payload,
                        attempts=$attempts,
                        max_attempts=$max_attempts,
                        priority=$priority,
                        unique_key=$unique_key,
                        lease_time=$lease_time,
                        error_reason=NONE;
                END;

                COMMIT TRANSACTION;
                "#,
            )
            .bind(("table", &self.table))
            .bind(("id", job.id()))
            .bind(("queue", job.queue()))
            .bind(("kind", job.kind()))
            .bind(("payload", job.payload()))
            .bind(("now", now))
            .bind(("unique_key", job.unique_key()))
            .bind((
                "scheduled_at",
                Datetime::from_str(
                    &job.scheduled_at()
                        .unwrap_or_else(|| (OffsetDateTime::now_utc()))
                        .format(&Rfc3339)
                        .unwrap(),
                )
                .unwrap(),
            ))
            .bind(("attempts", job.attempts()))
            .bind(("max_attempts", job.max_attempts()))
            .bind(("priority", job.priority()))
            .bind(("lease_time", job.lease_time().as_secs()))
            .await
            .map_err(convert_surrealdb_error)?
            .check()
            .map_err(convert_surrealdb_error)?;

        Ok(())
    }
}
