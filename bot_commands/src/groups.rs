use super::*;
use serenity::framework::standard::macros::group;

#[group("General")]
#[commands(cmd_buttons, cmd_dropdowns)]
pub struct General;

#[group("Bot Utilities")]
#[commands(cmd_ping)]
pub struct BotUtils;
