use serenity::{
    builder::CreateEmbed,
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message,
};

// command name is example
#[command("example")]
// aliases are ex and ax
#[aliases("ex", "ax")]
// rate limit bucket is "heavy" (the one we made in fn entrypoint)
#[bucket = "heavy"]
#[description = "A example command."]
async fn cmd_example(ctx: &Context, msg: &Message) -> CommandResult {
    let mut embed = CreateEmbed::default();
    embed.title("Template Command");
    msg.channel_id
        .send_message(&ctx, |m| {
            m.embed(|e| {
                *e = embed;
                e
            })
        })
        .await?;
    Ok(())
}
