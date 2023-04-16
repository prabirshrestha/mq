# mq-surreal

## SQL

### Define Schema

```sql
DEFINE TABLE queue SCHEMAFULL;
DEFINE FIELD created_at     ON queue TYPE datetime    ASSERT $value != NONE;
DEFINE FIELD updated_at     ON queue TYPE datetime    ASSERT $value != NONE;
DEFINE FIELD scheduled_at   ON queue TYPE datetime    ASSERT $value != NONE;
DEFINE FIELD locked_at      ON queue TYPE datetime;
DEFINE FIELD queue          ON queue TYPE string      ASSERT $value != NONE;
DEFINE FIELD kind           ON queue TYPE string      ASSERT $value != NONE;
DEFINE FIELD max_attempts   ON queue TYPE number      ASSERT $value != NONE;
DEFINE FIELD attempts       ON queue TYPE number      ASSERT $value != NONE;
DEFINE FIELD lease_time     ON queue TYPE number      ASSERT $value != NONE;
DEFINE FIELD payload        ON queue FLEXIBLE TYPE object;
DEFINE FIELD error_reason   ON queue FLEXIBLE TYPE object;
```

Requires SurrealDB v1.0.0-beta9+
