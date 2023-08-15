
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
{"v":0,"name":"z2p-test","msg":"CREATE DATABASE \"0f3d04f4-09b4-4a76-bda1-e15d9b9964c1\";; rows affected: 0, rows returned: 0, elapsed: 37.849ms","level":30,"hostname":"bo","pid":52112,"time":"2023-08-15T04:27:26.218477987Z","target":"log","line":null,"file":null,"log.target":"sqlx::query","log.module_path":"sqlx::query"}
{"v":0,"name":"z2p-test","msg":"/* SQLx ping */; rows affected: 0, rows returned: 0, elapsed: 295.921µs","level":30,"hostname":"bo","pid":52112,"time":"2023-08-15T04:27:26.312635978Z","target":"log","line":null,"file":null,"log.target":"sqlx::query","log.module_path":"sqlx::query"}
{"v":0,"name":"z2p-test","msg":"SELECT current_database(); rows affected: 0, rows returned: 1, elapsed: 851.834µs","level":30,"hostname":"bo","pid":52112,"time":"2023-08-15T04:27:26.313595008Z","target":"log","line":null,"file":null,"log.target":"sqlx::query","log.module_path":"sqlx::query"}
{"v":0,"name":"z2p-test","msg":"SELECT pg_advisory_lock($1); rows affected: 1, rows returned: 1, elapsed: 783.515µs","level":30,"hostname":"bo","pid":52112,"time":"2023-08-15T04:27:26.314451037Z","target":"log","line":null,"file":null,"log.target":"sqlx::query","log.module_path":"sqlx::query"}
{"v":0,"name":"z2p-test","msg":"CREATE TABLE IF NOT …; rows affected: 0, rows returned: 0, elapsed: 8.622ms\n\nCREATE TABLE IF NOT EXISTS _sqlx_migrations (\n  version BIGINT PRIMARY KEY,\n  description TEXT NOT NULL,\n  installed_on TIMESTAMPTZ NOT NULL DEFAULT now(),\n  success BOOLEAN NOT NULL,\n  checksum BYTEA NOT NULL,\n  execution_time BIGINT NOT NULL\n);\n","level":30,"hostname":"bo","pid":52112,"time":"2023-08-15T04:27:26.325815627Z","target":"log","line":null,"file":null,"log.module_path":"sqlx::query","log.target":"sqlx::query"}
{"v":0,"name":"z2p-test","msg":"SELECT version FROM _sqlx_migrations …; rows affected: 0, rows returned: 0, elapsed: 1.229ms\n\nSELECT\n  version\nFROM\n  _sqlx_migrations\nWHERE\n  success = false\nORDER BY\n  version\nLIMIT\n  1\n","level":30,"hostname":"bo","pid":52112,"time":"2023-08-15T04:27:26.327859088Z","target":"log","line":null,"file":null,"log.target":"sqlx::query","log.module_path":"sqlx::query"}
{"v":0,"name":"z2p-test","msg":"SELECT version, checksum FROM …; rows affected: 0, rows returned: 0, elapsed: 789.003µs\n\nSELECT\n  version,\n  checksum\nFROM\n  _sqlx_migrations\nORDER BY\n  version\n","level":30,"hostname":"bo","pid":52112,"time":"2023-08-15T04:27:26.33001624Z","target":"log","line":null,"file":null,"log.module_path":"sqlx::query","log.target":"sqlx::query"}
{"v":0,"name":"z2p-test","msg":"BEGIN; rows affected: 0, rows returned: 0, elapsed: 190.276µs","level":30,"hostname":"bo","pid":52112,"time":"2023-08-15T04:27:26.330329565Z","target":"log","line":null,"file":null,"log.module_path":"sqlx::query","log.target":"sqlx::query"}
{"v":0,"name":"z2p-test","msg":"-- Create Subscriptions Table …; rows affected: 0, rows returned: 0, elapsed: 10.181ms\n\n-- Create Subscriptions Table\nCREATE TABLE subscriptions(\n  id uuid NOT NULL,\n  PRIMARY KEY (id),\n  email TEXT NOT NULL UNIQUE,\n  name TEXT NOT NULL,\n  subscribed_at timestamptz NOT NULL\n);\n","level":30,"hostname":"bo","pid":52112,"time":"2023-08-15T04:27:26.342645545Z","target":"log","line":null,"file":null,"log.module_path":"sqlx::query","log.target":"sqlx::query"}
{"v":0,"name":"z2p-test","msg":"COMMIT; rows affected: 0, rows returned: 0, elapsed: 1.435ms","level":30,"hostname":"bo","pid":52112,"time":"2023-08-15T04:27:26.344172665Z","target":"log","line":null,"file":null,"log.module_path":"sqlx::query","log.target":"sqlx::query"}
{"v":0,"name":"z2p-test","msg":"INSERT INTO _sqlx_migrations ( …; rows affected: 1, rows returned: 0, elapsed: 1.569ms\n\nINSERT INTO\n  _sqlx_migrations (\n    version,\n    description,\n    success,\n    checksum,\n    execution_time\n  )\nVALUES\n  ($1, $2, TRUE, $3, $4)\n","level":30,"hostname":"bo","pid":52112,"time":"2023-08-15T04:27:26.3473355Z","target":"log","line":null,"file":null,"log.module_path":"sqlx::query","log.target":"sqlx::query"}
{"v":0,"name":"z2p-test","msg":"SELECT current_database(); rows affected: 0, rows returned: 1, elapsed: 236.063µs","level":30,"hostname":"bo","pid":52112,"time":"2023-08-15T04:27:26.347669305Z","target":"log","line":null,"file":null,"log.module_path":"sqlx::query","log.target":"sqlx::query"}
{"v":0,"name":"z2p-test","msg":"SELECT pg_advisory_unlock($1); rows affected: 1, rows returned: 1, elapsed: 437.981µs","level":30,"hostname":"bo","pid":52112,"time":"2023-08-15T04:27:26.348191903Z","target":"log","line":null,"file":null,"log.module_path":"sqlx::query","log.target":"sqlx::query"}
{"v":0,"name":"z2p-test","msg":"starting 6 workers","level":30,"hostname":"bo","pid":52112,"time":"2023-08-15T04:27:26.350441122Z","target":"actix_server::builder","line":200,"file":"/home/bo/.cargo/registry/src/index.crates.io-6f17d22bba15001f/actix-server-2.2.0/src/builder.rs"}
{"v":0,"name":"z2p-test","msg":"Tokio runtime found; starting in existing Tokio runtime","level":30,"hostname":"bo","pid":52112,"time":"2023-08-15T04:27:26.403132863Z","target":"actix_server::server","line":197,"file":"/home/bo/.cargo/registry/src/index.crates.io-6f17d22bba15001f/actix-server-2.2.0/src/server.rs"}
{"v":0,"name":"z2p-test","msg":"/* SQLx ping */; rows affected: 0, rows returned: 0, elapsed: 9.419ms","level":30,"hostname":"bo","pid":52112,"time":"2023-08-15T04:27:26.412484852Z","target":"log","line":null,"file":null,"log.target":"sqlx::query","log.module_path":"sqlx::query"}
{"v":0,"name":"z2p-test","msg":"127.0.0.1 \"GET /health_check HTTP/1.1\" 200 0 \"-\" \"-\" 0.000070","level":30,"hostname":"bo","pid":52112,"time":"2023-08-15T04:27:26.412881734Z","target":"log","line":null,"file":null,"log.module_path":"actix_web::middleware::logger","log.file":"/home/bo/.cargo/registry/src/index.crates.io-6f17d22bba15001f/actix-web-4.3.1/src/middleware/logger.rs","log.line":421,"log.target":"actix_web::middleware::logger"}
test health_check_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.34s

