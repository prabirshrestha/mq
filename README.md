mq
==

Simple message queue library for [rust](https://www.rust-lang.org/).

Example
=======

## Cargo.toml
```toml
[dependencies]
mq = { version = "0.3.0", features = ["blackhole-broker"] }
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

    let msg = mq.dequeue::<String>("hello", None).await?;
    match msg {
        Some(msg) => {
            println!("message id: {}", msg.id);
            println!("message data: {}", msg.data);
        }
        None => println!("no message"),
    }

    mq.delete_queue("hello").await?;

    Ok(())
}
```
