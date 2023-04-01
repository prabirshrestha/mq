mq
==

Generic simple message queue library for [rust](https://www.rust-lang.org/).

Example
=======

## Cargo.toml
```toml
[dependencies]
mq = { version = "0.4.1" }
# mq-surreal depends on surrealdb unreleased beta9 which is not published yet.
mq-surreal = { git = "https://github.com/prabirshrestha/mq.git", tag = "v0.4.1" }
tokio = { version = "1.27.0", features = ["full"] }
```

Refer to the examples on the usage.

# Supported Backends

* SurrealDB (requires not released beta9. tested with 50ea5c52cb9e041b344b249c82f9c278f2ebf184)

If you are interested in other backends feel free submit PR or features requests.

# LICENSE

MIT
