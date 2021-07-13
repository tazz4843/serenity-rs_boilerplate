use crate::BotRawEventHandler;
use serenity::model::prelude::Event;
use serenity::prelude::Context;

pub async fn raw_event(_handler: &BotRawEventHandler, _ctx: Context, _event: Event) {
    // handle raw events here
}
