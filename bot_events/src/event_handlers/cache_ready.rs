use crate::event_handler::BotEventHandler;
use serenity::model::prelude::GuildId;
use serenity::prelude::Context;

// spawn any background tasks here!
// make sure to swap the boolean in the event handler
pub async fn cache_ready(handler: &BotEventHandler, ctx: Context, _guild_ids: Vec<GuildId>) {}
