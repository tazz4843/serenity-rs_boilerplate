use crate::event_handlers::*;
use serenity::async_trait;
use serenity::model::prelude::{GuildId, Message, Ready};
use serenity::prelude::{Context, EventHandler};
use std::sync::atomic::AtomicBool;
use std::time::Instant;

// some notes
//
// 1: i strongly recommend you don't add any actual code to this file unless it's adding a
// extra function to the impl EventHandler
// this just helps keep it clean and easy to understand
//
// 2: add things to the struct as you need, but also make sure you're adding it to the Default
// implementation
//
// 3: you only get a immutable self reference in the handler, so you can put anything that needs
// to be mutated behind a `std::sync::Arc<tokio::sync::RwLock<T>>` and then grab a immutable ref with
// `self.item.read().await` or a mutable ref with `self.item.write().await`
// MAKE SURE TO DROP THE RETURNED ITEM AS SOON AS POSSIBLE
// otherwise (in the case of writers) no other thread can access the items inside
// in the case of readers, it means only readers can access the items inside, not writers
//
// 4: if you need to run a database query in a event
//
// use bot_db::PgPoolKey;
// use sqlx::query;
// let db = {
//   let data = ctx.data.read().await;
//   data.get::<PgPoolKey>().clone()
// };
// query!("QUERY HERE").execute(db).await
//
// the pool is internally behind a Arc so is safe to clone

pub struct BotEventHandler {
    start_time: Instant,
    loop_running: AtomicBool,
}

impl Default for BotEventHandler {
    fn default() -> Self {
        Self {
            start_time: Instant::now(),
            loop_running: AtomicBool::new(false),
        }
    }
}

#[async_trait]
impl EventHandler for BotEventHandler {
    async fn cache_ready(&self, ctx: Context, guild_ids: Vec<GuildId>) {
        cache_ready::cache_ready(self, ctx, guild_ids).await;
    }

    async fn message(&self, ctx: Context, msg: Message) {
        message::message(self, ctx, msg).await;
    }

    async fn ready(&self, ctx: Context, bot_info: Ready) {
        ready::ready(self, ctx, bot_info).await;
    }
}
