mq
==

Simple message queue library for [rust](https://www.rust-lang.org/).

Example
=======


## Cargo.toml
```toml
[dependencies]
mq = { path = "../mq", features = ["blackhole-broker"] }
tokio = { version = "1.19.1", features = ["full"] }
```

## main.rs

```rust
use mq::{broker::blackhole::BlackholeMessageBroker, MessageQueue, MqResult};

#[tokio::main]
async fn main() -> MqResult<()> {
    let mut mq = BlackholeMessageBroker::new();
    mq.create_queue("hello").await?;
    mq.enqueue("hello", "some message").await?;
    mq.delete_queue("hello").await?;
    Ok(())
}
```
