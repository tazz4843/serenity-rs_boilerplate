use crate::PG_POOL;
use bot_config::BotConfig;
use sqlx::postgres::PgConnectOptions;
use sqlx::{query, PgPool, Pool, Postgres};

pub async fn set_db() -> Pool<Postgres> {
    let config = BotConfig::get();
    let (db_host, db_port, db_user, db_password, db_db) = config.db_connection();
    let db_conn_options = PgConnectOptions::new();
    let db = PgPool::connect_with(
        db_conn_options
            .host(db_host)
            .username(db_user)
            .port(db_port)
            .database(db_db)
            .application_name("scripty")
            .password(db_password)
            .statement_cache_capacity(1000_usize),
    )
    .await
    .expect("Couldn't connect to DB");

    query(
        "CREATE TABLE IF NOT EXISTS prefixes (
        guild_id BIGINT PRIMARY KEY,
        prefix TEXT
    )",
    )
    .execute(&db)
    .await
    .expect("Couldn't create the prefix table.");

    PG_POOL
        .set(db.clone())
        .expect("pool was already set, don't call `set_db` more than once");

    db
}
