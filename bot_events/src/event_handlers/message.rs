use crate::event_handler::BotEventHandler;
use serenity::model::prelude::Message;
use serenity::prelude::Context;

pub async fn message(handler: &BotEventHandler, ctx: Context, new_message: Message) {
    // do what you want here! this function gets called in `crate::event_handler::BotEventHandler`'s
    // message() function. to see this, press ctrl+b with the cursor over the function name
}
