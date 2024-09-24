# mq-surreal

## SQL

### Define Schema

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
DEFINE FIELD IF NOT EXISTS payload        ON queue FLEXIBLE TYPE object;
DEFINE FIELD IF NOT EXISTS error_reason   ON queue FLEXIBLE TYPE option<object>;
```

# Surrealdb Compatibility

mq-surreal version          | surrealdb version
----------------------------|------------------
`0.10.0`                    | `>= 1.5.0  && < 2.x`
`0.20.x`                    | `>= 2.0.0  && < 3.x`
