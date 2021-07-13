use serenity::client::bridge::gateway::ShardManager;
use serenity::prelude::{TypeMapKey, Mutex};
use std::sync::Arc;

pub struct ShardManagerWrapper;
impl TypeMapKey for ShardManagerWrapper {
    type Value = Arc<Mutex<ShardManager>>;
}
