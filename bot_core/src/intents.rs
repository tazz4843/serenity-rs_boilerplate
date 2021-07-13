use serenity::client::bridge::gateway::GatewayIntents;

pub const INTENTS: GatewayIntents = GatewayIntents::DIRECT_MESSAGES
    | GatewayIntents::GUILD_MESSAGES
    | GatewayIntents::DIRECT_MESSAGE_REACTIONS
    | GatewayIntents::GUILD_MESSAGE_REACTIONS
    | GatewayIntents::GUILDS;
