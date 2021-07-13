use serenity::prelude::{ Context};
use serenity::model::prelude::Event;
use crate::BotRawEventHandler;

pub async fn raw_event(handler: &BotRawEventHandler, ctx: Context, event: Event) {
    // handle raw events here
}
