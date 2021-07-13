use serenity::async_trait;
use serenity::model::prelude::Event;
use serenity::prelude::{Context, RawEventHandler};
use crate::raw_event_handlers::*;

// go read the notes in event_handler.rs before fiddling around in here

pub struct BotRawEventHandler;

impl Default for BotRawEventHandler {
    fn default() -> Self {
        Self
    }
}

#[async_trait]
impl RawEventHandler for BotRawEventHandler {
    async fn raw_event(&self, ctx: Context, event: Event) {
        raw_event::raw_event(self, ctx, event).await;
    }
}
