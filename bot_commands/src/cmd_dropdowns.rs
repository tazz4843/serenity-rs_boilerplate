use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message,
};
use serenity::collector::CollectComponentInteraction;
use serenity::model::interactions::InteractionResponseType;

#[command("dropdowns")]
#[description = "dropdowns"]
async fn cmd_dropdowns(ctx: &Context, msg: &Message) -> CommandResult {
    let m = msg.channel_id
       .send_message(&ctx, |m| {
           m.content("dropdowns")
               .components(|c| {
                   c.create_action_row(|r| {
                       r.create_select_menu(|m| {
                           m.custom_id("dropdown1")
                               .placeholder("pick something you lazy ass")
                               .options(|o| {
                                   for (val, name) in vec!["one", "two", "three", "four"].into_iter().enumerate() {
                                       o.create_option(|c| {
                                           c.label(name)
                                               .description((val+1).to_string())
                                               .value(name)
                                       });
                                   }
                                   o
                               })
                       })
                   })
               })
       })
       .await?;

    if let Some(i) = CollectComponentInteraction::new(&ctx)
        .message_id(m.id)
        .filter(|x| x.data.custom_id.as_str() == "dropdown1")
        .await {
        match i.data.values.get(0) {
            Some(val) => {
                i.create_interaction_response(&ctx, |r| {
                    r.kind(InteractionResponseType::ChannelMessageWithSource)
                     .interaction_response_data(|d| {
                         d.content(format!("you picked {}", val))
                     })
                }).await?
            }
            None => {
                i.create_interaction_response(&ctx, |r| {
                    r.kind(InteractionResponseType::ChannelMessageWithSource)
                     .interaction_response_data(|d| {
                         d.content("blame discord™")
                     })
                }).await?
            }
        }
    }

    Ok(())
}
