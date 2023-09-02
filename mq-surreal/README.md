# mq-surreal

## SQL

### Define Schema

```sql
DEFINE TABLE queue SCHEMAFULL;
DEFINE FIELD created_at     ON queue TYPE datetime;
DEFINE FIELD updated_at     ON queue TYPE datetime;
DEFINE FIELD scheduled_at   ON queue TYPE datetime;
DEFINE FIELD locked_at      ON queue TYPE option<datetime>;
DEFINE FIELD queue          ON queue TYPE string;
DEFINE FIELD kind           ON queue TYPE string;
DEFINE FIELD max_attempts   ON queue TYPE number;
DEFINE FIELD attempts       ON queue TYPE number;
DEFINE FIELD priority       ON queue TYPE number;
DEFINE FIELD unique_key     ON queue TYPE option<string>;
DEFINE FIELD lease_time     ON queue TYPE number;
DEFINE FIELD payload        ON queue FLEXIBLE TYPE object;
DEFINE FIELD error_reason   ON queue FLEXIBLE TYPE option<object>;
```

Requires SurrealDB v1.0.0-beta9+
