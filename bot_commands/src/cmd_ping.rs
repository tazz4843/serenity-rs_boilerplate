// you shouldn't need to modify this one

use serenity::{
    builder::CreateEmbed,
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message,
};
use bot_utils::ShardManagerWrapper;
use std::time::Instant;
use bot_db::{DATABASE_ENABLED, PgPoolKey};
use sqlx::query;

#[command("ping")]
#[aliases("p")]
#[description = "🏓"]
async fn cmd_ping(ctx: &Context, msg: &Message) -> CommandResult {
    let mut embed = CreateEmbed::default();
    embed.title("🏓");

    embed.field("WebSocket Latency", {
        let shard_manager_lock = {
            let data = ctx.data.read().await;
            data.get::<ShardManagerWrapper>()
                .expect("failed to get shard manager out of bot TypeMap")
                .clone()
        };
        let shard_manager = shard_manager_lock.lock().await;
        let mut total: u8 = 0;
        let mut latency: u128 = 0;
        for i in shard_manager.runners.lock().await.iter() {
            if let Some(l) = i.1.latency {
                total += 1;
                latency += l.as_millis();
            }
        }
        if total == 0 {
            // no shards ready
            latency = 0
        } else {
            latency /= total as u128;
        }
        format!("{:.3}ms", latency as f64 / 1_000_000.0)
    }, false);

    embed.field("Discord REST API", {
        let ft = msg.channel_id.broadcast_typing(&ctx);
        let st = Instant::now();
        ft.await?;
        let et = Instant::now();
        format!("{:.3}ms", et.duration_since(st).as_nanos() as f64 / 1_000_000.0)
    }, false);

    if DATABASE_ENABLED {
        embed.field("PostgreSQL", if let Some(id) = msg.guild_id {
            let data = ctx.data.read().await;
            let db = data.get::<PgPoolKey>().expect("pool not inserted in at start time");
            let ft = query!("SELECT prefix FROM prefixes WHERE guild_id = $1", id.0 as i64).fetch_one(db);
            let st = Instant::now();
            let _ = ft.await;
            let et = Instant::now();
            format!("{:.3}ms", et.duration_since(st).as_nanos() as f64 / 1_000_000.0)
        } else {
            "only available in guilds".to_string()
        }, false);
    }

    msg.channel_id
       .send_message(&ctx, |m| {
           m.embed(|e| {
               *e = embed;
               e
           })
       })
       .await?;
    Ok(())
}

