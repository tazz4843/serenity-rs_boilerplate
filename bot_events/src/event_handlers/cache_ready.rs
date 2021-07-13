use crate::event_handler::BotEventHandler;
use serenity::model::prelude::GuildId;
use serenity::prelude::Context;
use std::sync::atomic::Ordering;

pub async fn cache_ready(handler: &BotEventHandler, _ctx: Context, _guild_ids: Vec<GuildId>) {
    if handler.loop_running.swap(true, Ordering::Relaxed) {
        // spawn any background tasks here! use tokio::spawn for that
    }
}
