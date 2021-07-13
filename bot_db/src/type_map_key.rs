use serenity::prelude::TypeMapKey;
use sqlx::{Postgres, Pool};

pub struct PgPoolKey;
impl TypeMapKey for PgPoolKey {
    type Value = Pool<Postgres>;
}
