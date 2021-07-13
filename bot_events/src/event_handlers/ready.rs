use crate::event_handler::BotEventHandler;
use serenity::model::prelude::Ready;
use serenity::prelude::Context;
use tracing::info;

// IMPORTANT: this being fired does NOT mean the cache is filled:
// cache_ready exists for that.
// background tasks, unless they do not rely on cache, should be started in
// cache_ready instead.
pub async fn ready(handler: &BotEventHandler, _ctx: Context, bot_info: Ready) {
    info!("bot ready, in {} guilds", bot_info.guilds.len());
}
