use crate::intents::INTENTS;
use bot_config::BotConfig;
use bot_db::{DATABASE_ENABLED, PgPoolKey, dyn_prefix};
use serenity::client::{parse_token, TokenComponents};
use serenity::framework::standard::buckets::LimitedFor;
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::Client;
use std::boxed::Box;
use std::collections::HashSet;
use tracing::{info};

// if you're new to Rust this function signature might look weird
// it's just saying this function can return no error and no value
// (which is what Ok(()) at the end is) or it can return a Box
// containing a struct that implements std::error::Error.
// this lets you use ? anywhere to return errors to the main function
pub async fn entrypoint() -> Result<(), Box<dyn std::error::Error>> {
    // let's begin by changing directory to the bot executable file
    bot_utils::set_dir();

    // now load the config
    BotConfig::set("config.toml");

    // next, load the database
    let db = if DATABASE_ENABLED {
        Some(bot_db::set_db().await)
    } else {
        None
    };

    info!("parsing token...");
    // this is the BOT ID, NOT the app ID
    // we need to do a HTTP request for the app ID
    let TokenComponents { bot_user_id, .. } =
        parse_token(BotConfig::get().token()).expect("invalid token");
    info!("token is valid");

    info!("fetching bot owners...");
    // fetch bot owners
    let (owners, app_id) = {
        // make a serenity HTTP client
        let http = Http::new_with_token(BotConfig::get().token());

        // fetch app info
        let info = http
            .get_current_application_info()
            .await
            .expect("failed to fetch app info");

        let mut owners = HashSet::new();
        // check if the bot is on a team
        match info.team {
            Some(t) => {
                // if it is, iterate over the team members and insert them into the hashmap
                t.members.iter().map(|x| owners.insert(x.user.id));
            },
            None => {
                // and if it isn't, just add the owner ID
                owners.insert(info.owner.id);
            },
        };

        (owners, info.id)
    };
    info!("found {} owners", owners.len());

    info!("creating bot framework...");
    // initialize the serenity client's command framework
    let framework = StandardFramework::new()
        // make a closure to configure the framework
        .configure(|c| {
            c
                // want commands in DMs?
                .allow_dm(true)

                // need to block some users/guilds?
                // note that these lists cannot be updated at runtime
                .blocked_guilds(vec![].into_iter().collect())
                .blocked_users(vec![].into_iter().collect())

                // bot owners: this is fetched for you earlier on
                .owners(owners)

                // should commands be case insensitive?
                .case_insensitivity(true)

                // need to disable a command?
                // same note as for blocked_{guilds,users}, you can't modify at runtime
                .disabled_commands(vec![].into_iter().collect())

                // ignore bots and/or webhooks?
                // you almost always want this set to true
                .ignore_bots(true)
                .ignore_webhooks(true)

                // do you need a prefix in DMs?
                .no_dm_prefix(true)

                // should mentioning this ID also be used as a prefix?
                // setting to None will not allow mention prefixes
                // setting to Some will allow mentions of that userid to be a prefix
                .on_mention(Some(bot_user_id))

                // global, bot-wide prefixes:
                // these cannot be changed at runtime
                .prefixes(vec!["~", "bt!"]);

            if DATABASE_ENABLED {
                // dynamic prefix: this changes based on the DB being enabled or not
                c.dynamic_prefix(|ctx, msg| Box::pin(dyn_prefix(ctx, msg)));
            }
            c
        })

        // let's add a new rate limit bucket named "heavy"
        .bucket("heavy", |b| {
            b
                // rate limit is per-user
                .limit_for(LimitedFor::User)

                // cancel command invocations on rate limit
                .await_ratelimits(0)

                // uncomment this line and pass a function if you want to have a function
                // to handle rate limits
                // .delay_action()

                // rate limits are per every 300 seconds
                .time_span(300)

                // a user can only run a command in this bucket 10x every 300s
                .limit(10)

            // NO SEMICOLON AT THE END!
            // you're returning the builder instead of just modifying it
            // also note no `return` statement
        })
        // **NOTE**: if you add a bucket you *must* await the result
        // this is because the buckets are internally stored behind a asynchronous Mutex
        // that must be awaited to be unlocked
        .await

        // this is the help command: naming is a little bit different because of the way
        // the macros change things up, but just turn the function name into all uppercase
        // and you will find the actual command
        .help(&CMD_HELP)

        // this adds a group to the bot: same notes as for the help command above, but you
        // must append `_GROUP` to the struct name and turn it into all uppercase
        .group(&GENERAL_GROUP)

        // if you have a error handler, uncomment this line and pass the function (don't call it!)
        // .on_dispatch_error()

        // if you need to call a function before running a command, do it here
        // if the called function returns false, the command invocation will be cancelled
        // .before()

        // add error handlers here: if a cmd returned a error, it'll be passed as the 4th
        // argument to this function
        // .after()

        // want to do something on a normal message? you can also use `EventHandler::on_message`
        // and you should use that instead actually
        // .normal_message()

        // want to do something when a user types only the prefix and nothing else?
        // .prefix_only()

        // unknown command? use this
        // .unrecognised_command()
        ;

    info!("initializing serenity client...");
    // construct a client with the token
    let mut client = Client::builder(BotConfig::get().token())

        // modify intents in bot_core/src/intents.rs
        .intents(INTENTS)

        // pass in the framework we made above
        .framework(framework)

        // pass in the app ID we got above (required for slash commands)
        .application_id(u64::from(app_id))

        // await the client to construct it
        .await

        // check the Result returned
        .expect("failed to create the client");

    {
        // this needs to be put into a block, otherwise the lock will never be dropped and will
        // deadlock any commands trying to use anything in the data map
        let mut map = client.data.write().await;

        // if the DB pool exists, insert it
        if let Some(db) = db {
            map.insert::<PgPoolKey>(db)
        }
    }

    // start the client
    client.start_autosharded().await?;

    Ok(())
}
