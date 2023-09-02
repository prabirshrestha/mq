mq
==

Generic simple message queue library for [rust](https://www.rust-lang.org/).

Example
=======

## Cargo.toml

```toml
[dependencies]
mq = "0.5.0"
mq-surreal = "0.5.0"
tokio = { version = "1.27.0", features = ["full"] }
```

Refer to the examples on the usage.

# Supported Backends

* SurrealDB (requires v1.0.0-beta10+). For SurrealDB v1.0.0-beta9 use v0.4.8 version for mq.

If you are interested in other backends feel free submit PR or features requests.

# LICENSE

MIT
