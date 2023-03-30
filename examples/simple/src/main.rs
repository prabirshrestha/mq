use std::sync::Arc;

use anyhow::Result;
use mq::{Consumer, Context, Job, JobResult, Producer, Worker};
use mq_surreal::{SurrealJobProcessor, SurrealProducer};
use serde::Deserialize;
use serde_json::json;
use tokio_util::sync::CancellationToken;

#[derive(Deserialize, Debug)]
pub struct SendEmail {
    to: String,
    body: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    // connect to surrealdb
    let db = Arc::new(
        surrealdb::engine::any::connect(("file://./data.db", surrealdb::opt::Strict)).await?,
    );

    // create surrealdb namespace and db
    db.query("DEFINE NAMESPACE test; USE NAMESPACE test; DEFINE DATABASE test;")
        .await?
        .check()?;
    db.use_ns("test").use_db("test").await?;

    let table = "queue";

    // create table schema for mq
    db.query(format!(
        r#"REMOVE TABLE {table};
    DEFINE TABLE {table} SCHEMAFULL;
    DEFINE FIELD created_at     ON {table} TYPE datetime    ASSERT $value != NONE;
    DEFINE FIELD scheduled_at   ON {table} TYPE datetime    ASSERT $value != NONE;
    DEFINE FIELD locked_at      ON {table} TYPE datetime;
    DEFINE FIELD queue          ON {table} TYPE string      ASSERT $value != NONE;
    DEFINE FIELD kind           ON {table} TYPE string      ASSERT $value != NONE;
    DEFINE FIELD max_attempts   ON {table} TYPE number      ASSERT $value != NONE;
    DEFINE FIELD attempts       ON {table} TYPE number      ASSERT $value != NONE;
    DEFINE FIELD lease_time     ON {table} TYPE number      ASSERT $value != NONE;
    DEFINE FIELD payload        ON {table} FLEXIBLE TYPE object;
    DEFINE FIELD error_reason   ON {table} FLEXIBLE TYPE object;
    "#
    ))
    .await?
    .check()?;

    // uncomment the following line to delete all the records in the table to start clean
    // db.query(format!("DELETE {table}")).await?.check()?;

    // create a producer to start publishing new jobs
    let producer = SurrealProducer::new(db.clone(), table);

    // publish a job
    producer
        .publish(Job::new(
            "send-email",
            json!({ "to": "hi@example.com", "body": "hello from mq!" }),
        ))
        .await?;

    // register job handlers and start the worker
    let cancellation_token = CancellationToken::new();
    let worker = Worker::new(
        Consumer::new().register(("send-email", |ctx: Context| async move {
            dbg!(&ctx);
            let send_email: SendEmail = ctx.deserialize()?;
            dbg!(&send_email);
            // Err(mq::Error::UnknownError("some error".into()))
            Ok(JobResult::CompleteWithSuccess)
        })),
    )
    .with_concurrency(Some(num_cpus::get()))
    .with_poll_interval(Some(3000))
    .with_cancellation_token(cancellation_token.clone())
    .run(SurrealJobProcessor::new(db.clone(), table));

    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        cancellation_token.cancel();
    });

    worker.await?;

    Ok(())
}
