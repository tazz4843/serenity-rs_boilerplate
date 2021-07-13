use crate::PgPoolKey;
use serenity::model::prelude::Message;
use serenity::prelude::Context;
use tracing::warn;
use sqlx::query;

pub async fn dyn_prefix(ctx: &Context, msg: &Message) -> Option<String> {
    let guild_id = msg.guild_id?;

    let data = ctx.data.read().await;
    let db = unsafe { data.get::<PgPoolKey>().unwrap_unchecked() };

    match query!(
        "SELECT
           prefix
         FROM
           prefixes
         WHERE
           guild_id = $1",
        guild_id.0 as i64
    )
        .fetch_optional(db)
        .await
    {
        Err(err) => {
            warn!(
                "couldn't fetch prefix from DB: {:?}",
                err,
            );
            None
        }
        Ok(row) => row.map_or(None, |x| x.prefix),
    }
}
