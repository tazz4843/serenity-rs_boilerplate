use serenity::prelude::TypeMapKey;
use sqlx::{Pool, Postgres};

pub struct PgPoolKey;
impl TypeMapKey for PgPoolKey {
    type Value = Pool<Postgres>;
}
