use serenity::{
    client::Context,
    framework::standard::{
        help_commands, macros::help, Args, CommandGroup, CommandResult, HelpOptions,
    },
    model::prelude::{Message, UserId},
};
use std::collections::HashSet;
use tracing::trace;

#[help("help", "commands", "cmds")]
#[suggestion_text = "**Could one of these be what you're looking for?**\n{}"]
#[max_levenshtein_distance(3)]
#[no_help_available_text = "Unknown command"]
#[usage_label = "Use it as follows:"]
#[usage_sample_label = "For example"]
#[checks_label = "Requirements"]
#[aliases_label = "Aliases"]
#[group_prefix = "Group Prefix"]
#[grouped_label = "Group"]
#[description_label = "  "]
#[indention_prefix = "  "]
#[available_text = "Use it in"]
#[dm_only_text = "DMs only"]
#[guild_only_text = "Guilds only"]
#[dm_and_guild_text = "Both guilds and DMs"]
#[individual_command_tip = "For more info, type `help [command name]`"]
#[strikethrough_commands_tip_in_dm = ""]
#[strikethrough_commands_tip_in_guild = ""]
#[lacking_role = "Nothing"]
#[lacking_permissions = "Nothing"]
#[lacking_ownership = "Nothing"]
#[lacking_conditions = "Nothing"]
#[wrong_channel = "Nothing"]
#[embed_error_colour = "#b00020"]
#[embed_success_colour = "#b29ddb"]
async fn cmd_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    trace!("user {} called help", msg.author);
    help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}
