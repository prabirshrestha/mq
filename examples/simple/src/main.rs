use std::sync::Arc;

use anyhow::Result;
use mq::{Consumer, Context, Job, JobResult, Producer, Worker};
use mq_surreal::{SurrealJobProcessor, SurrealProducer};
use serde::Deserialize;
use serde_json::json;
use surrealdb::opt::capabilities::Capabilities;
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
        surrealdb::engine::any::connect((
            format!(
                "surrealkv://{}/data.db",
                std::env::current_dir().unwrap().to_string_lossy()
            ),
            surrealdb::opt::Config::new()
                .set_strict(true)
                .capabilities(Capabilities::all()),
        ))
        .await?,
    );

    // create surrealdb namespace and db
    db.query("DEFINE NAMESPACE IF NOT EXISTS test; USE NAMESPACE test; DEFINE DATABASE IF NOT EXISTS test;")
        .await?
        .check()?;
    db.use_ns("test").use_db("test").await?;

    let table = "queue";

    // create table schema for mq
    db.query(format!(
        r#"
DEFINE TABLE IF NOT EXISTS {table} SCHEMAFULL;
DEFINE FIELD IF NOT EXISTS created_at     ON {table} TYPE datetime;
DEFINE FIELD IF NOT EXISTS updated_at     ON {table} TYPE datetime;
DEFINE FIELD IF NOT EXISTS scheduled_at   ON {table} TYPE datetime;
DEFINE FIELD IF NOT EXISTS locked_at      ON {table} TYPE option<datetime>;
DEFINE FIELD IF NOT EXISTS queue          ON {table} TYPE string;
DEFINE FIELD IF NOT EXISTS kind           ON {table} TYPE string;
DEFINE FIELD IF NOT EXISTS max_attempts   ON {table} TYPE number;
DEFINE FIELD IF NOT EXISTS attempts       ON {table} TYPE number;
DEFINE FIELD IF NOT EXISTS priority       ON {table} TYPE number;
DEFINE FIELD IF NOT EXISTS unique_key     ON {table} TYPE option<string>;
DEFINE FIELD IF NOT EXISTS lease_time     ON {table} TYPE number;
DEFINE FIELD IF NOT EXISTS payload        ON {table} FLEXIBLE TYPE object;
DEFINE FIELD IF NOT EXISTS error_reason   ON {table} FLEXIBLE TYPE option<object>;
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
            let send_email: SendEmail = ctx.deserialize()?;
            dbg!(send_email.to, send_email.body);
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
