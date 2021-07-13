use super::*;
use serenity::framework::standard::macros::group;

#[group("General")]
#[commands(cmd_example)]
pub struct General;
