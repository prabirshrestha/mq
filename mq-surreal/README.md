# mq-surreal

[SurrealDB](https://surrealdb.com/) backend for the [mq](https://crates.io/crates/mq) message queue library.

## Installation

```toml
[dependencies]
mq = "0.21.0"
mq-surreal = "0.21.0"
surrealdb = { version = "3.0.2", features = ["kv-surrealkv"] }
```

## Usage

```rust
use std::sync::Arc;
use mq::{Consumer, Context, Job, JobResult, Producer, Worker};
use mq_surreal::{SurrealJobProcessor, SurrealProducer};
use serde_json::json;

// connect to surrealdb
let db = Arc::new(
    surrealdb::engine::any::connect("surrealkv://data.db").await?,
);
db.use_ns("test").use_db("test").await?;

let table = "queue";

// create a producer and publish a job
let producer = SurrealProducer::new(db.clone(), table);
producer
    .publish(Job::new("send-email", json!({ "to": "hi@example.com" })))
    .await?;

// create a worker to process jobs
let worker = Worker::new(
    Consumer::new().register(("send-email", |ctx: Context| async move {
        println!("Processing job: {:?}", ctx.job());
        Ok(JobResult::CompleteWithSuccess)
    })),
)
.run(SurrealJobProcessor::new(db.clone(), table));
```

See the [examples](https://github.com/prabirshrestha/mq/tree/main/examples) directory for complete working examples.

## Schema

Create the following schema in your SurrealDB database before using `mq-surreal`:

```sql
DEFINE TABLE IF NOT EXISTS queue SCHEMAFULL;
DEFINE FIELD IF NOT EXISTS created_at     ON queue TYPE datetime;
DEFINE FIELD IF NOT EXISTS updated_at     ON queue TYPE datetime;
DEFINE FIELD IF NOT EXISTS scheduled_at   ON queue TYPE datetime;
DEFINE FIELD IF NOT EXISTS locked_at      ON queue TYPE option<datetime>;
DEFINE FIELD IF NOT EXISTS queue          ON queue TYPE string;
DEFINE FIELD IF NOT EXISTS kind           ON queue TYPE string;
DEFINE FIELD IF NOT EXISTS max_attempts   ON queue TYPE number;
DEFINE FIELD IF NOT EXISTS attempts       ON queue TYPE number;
DEFINE FIELD IF NOT EXISTS priority       ON queue TYPE number;
DEFINE FIELD IF NOT EXISTS unique_key     ON queue TYPE option<string>;
DEFINE FIELD IF NOT EXISTS lease_time     ON queue TYPE number;
DEFINE FIELD IF NOT EXISTS payload        ON queue TYPE object FLEXIBLE;
DEFINE FIELD IF NOT EXISTS error_reason   ON queue TYPE option<object> FLEXIBLE;
```

## SurrealDB Compatibility

| mq-surreal version | surrealdb version    |
|---------------------|----------------------|
| `0.10.0`            | `>= 1.5.0 && < 2.x` |
| `0.20.x`            | `>= 2.0.0 && < 3.x` |
| `0.21.x`            | `>= 3.0.0 && < 4.x` |
