mq
==

Simple message queue library for [rust](https://www.rust-lang.org/).

Example
=======

## Cargo.toml
```toml
[dependencies]
mq = { version = "0.3.0" }
tokio = { version = "1.19.1", features = ["full"] }
```

## main.rs

```rust
#[tokio::main]
async fn main() -> MqResult<()> {
    let mut p = NullProducer::new();

    let j1 = Job::new("foo", "hello")
        .on_queue("default".into())
        .schedule_immediate();

    p.enqueue(j1).await.unwrap();

    let mut c = NullConsumer::new();
    c.register("foo", |j| {
        println!("foo: {}", j.id());
        Ok(())
    })
    .register("bar", |j| {
        println!("bar: {}", j.id());
        Ok(())
    });

    c.run([ConsumerQueueOptions::new("default", 1)].into_iter())?;

    Ok(())
}
```
