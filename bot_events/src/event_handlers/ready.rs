use crate::event_handler::BotEventHandler;
use serenity::model::prelude::Ready;
use serenity::prelude::Context;
use std::sync::atomic::Ordering;
use tracing::info;

// IMPORTANT: this being fired does NOT mean the cache is filled:
// cache_ready exists for that.
// background tasks, unless they do not rely on cache, should be started in
// cache_ready instead.
pub async fn ready(handler: &BotEventHandler, _ctx: Context, bot_info: Ready) {
    if handler.loop_running.load(Ordering::Relaxed) {
        // this is firing after a reconnect
        info!("bot ready, in {} guilds", bot_info.guilds.len())
    } else {
        // this is after a fresh startup
        info!(
            "bot ready, took {}ms, in {} guilds",
            handler.start_time.elapsed().as_millis(),
            bot_info.guilds.len(),
        );
    }
}
