#![feature(once_cell)]

/// The bot-wide configuration.
///
/// This module contains a struct for the config, and
/// a static OnceCell that will contain the config once set up.
use serde::{Deserialize, Serialize};
use std::lazy::SyncOnceCell as OnceCell;
use std::{fs, io};
use tracing::{debug, error, info};

/// A struct containing the configuration. Use the getter functions, in union with `Self::get`
/// to get values out of the config.
#[derive(Serialize, Deserialize)]
pub struct BotConfig {
    token: String,
    invite: String,
    host: String,
    user: String,
    password: String,
    port: u16,
    db: String,
}

pub static BOT_CONFIG: OnceCell<BotConfig> = OnceCell::new();

impl BotConfig {
    pub fn set(config_path: &str) {
        if BOT_CONFIG.get().is_some() {
            panic!("trying to initialize config after it's already been initialized... bad human")
        }

        debug!("trying to load config from {}...", config_path);
        let config: BotConfig =
            toml::from_str(&fs::read_to_string(config_path).unwrap_or_else(|err| {
                if err.kind() == io::ErrorKind::NotFound {
                    error!("config not found, writing new configuration file to {}", config_path);
                    let cfg_str = toml::to_string_pretty(&BotConfig {
                        // no this is not a valid token
                        // i'm not that stupid
                        token: "Njk3NTQxMzAxMTQyNDIxNTc0.Xo4x9Q.3lMbKzFgMKSH-qoieLSQcT5bpZc".to_string(),
                        // this invite, however, is valid
                        // if you try to invite it tho you'll get a "bot private" error
                        invite: "https://discord.com/api/oauth2/authorize?client_id=697541301142421574&permissions=8\
    &redirect_uri=https%3A%2F%2Fdiscord.com%2Foauth2%2Fauthorized&scope=bot".to_string(),
                        host: "localhost".to_string(),
                        user: "test_bot".to_string(),
                        password: "test_bot".to_string(),
                        port: 5432,
                        db: "test_bot".to_string()
                    }).expect("failed to create default configuration");

                    fs::write(config_path, <std::string::String as AsRef<[u8]>>::as_ref(&cfg_str)).unwrap_or_else(|e| {
                        error!("failed to write config: {}", e);
                        info!("dumping config to stdout instead");
                        println!("{}", cfg_str);
                    });
                    panic!("config not found, wrote it to either stdout or to a file")
                } else {
                    panic!("{}", err)
                }
            })).expect("you seem to have a invalid config... perhaps double check it? {}");

        let _ = BOT_CONFIG.set(config);
    }

    pub fn get() -> &'static BotConfig {
        BOT_CONFIG
            .get()
            .expect("bot configuration not loaded, are you calling this before `BotConfig::set`?")
    }

    pub fn token(&self) -> &String {
        &self.token
    }
    pub fn invite(&self) -> &String {
        &self.invite
    }
    pub fn db_connection(&self) -> (&String, u16, &String, &String, &String) {
        (&self.host, self.port, &self.user, &self.password, &self.db)
    }
}
