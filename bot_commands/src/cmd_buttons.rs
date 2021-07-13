use serenity::collector::CollectComponentInteraction;
use serenity::model::interactions::InteractionResponseType;
use serenity::model::prelude::message_component::ButtonStyle;
use serenity::model::prelude::{InteractionApplicationCommandCallbackDataFlags, ReactionType};
use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message,
};
use std::time::Duration;

#[command("buttons")]
#[description = "AKA clicky bois"]
async fn cmd_example(ctx: &Context, msg: &Message) -> CommandResult {
    let mut m = msg
        .channel_id
        .send_message(ctx, |m| {
            m.content("here's some clicky bois").components(|c| {
                c.create_action_row(|r| {
                    r.create_button(|b| {
                        b.custom_id("clickyboi1")
                         .label("click me first!")
                         .disabled(false)
                         .style(ButtonStyle::Success)
                         .emoji(ReactionType::Unicode("🧠".to_string()))
                    })
                     .create_button(|b| {
                         b.custom_id("clickyboi2")
                          .label("and then click me")
                          .disabled(true)
                          .style(ButtonStyle::Danger)
                          .emoji(ReactionType::Unicode("🛌".to_string()))
                     })
                })
            })
        })
        .await?;

    // these massive match statements could probably be moved into macros but that'd be too complex
    // for a boilerplate like this
    // maybe later, or maybe this annoys you enough to make a PR 🙃
    match CollectComponentInteraction::new(ctx)
        .author_id(msg.author.id)
        .channel_id(msg.channel_id)
        .message_id(m.id)
        .collect_limit(1)
        .filter(|x| x.data.custom_id.as_str() == "clickyboi1")
        .timeout(Duration::from_secs(60))
        .await
    {
        Some(inter) => {
            inter
                .create_interaction_response(ctx, |r| {
                    r.kind(InteractionResponseType::ChannelMessageWithSource)
                     .interaction_response_data(|d| {
                         d.content("thanks for clicking the second clickyboi")
                          .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                     })
                })
                .await?;
            m.edit(ctx, |m| {
                m.components(|c| {
                    c.create_action_row(|r| {
                        r.create_button(|b| {
                            b.custom_id("clickyboi1")
                             .label("don't click me now")
                             .disabled(true)
                             .style(ButtonStyle::Danger)
                             .emoji(ReactionType::Unicode("🧠".to_string()))
                        })
                         .create_button(|b| {
                             b.custom_id("clickyboi2")
                              .label("click me now")
                              .disabled(false)
                              .style(ButtonStyle::Success)
                              .emoji(ReactionType::Unicode("🛌".to_string()))
                         })
                    })
                })
            })
             .await?;
        }
        None => {
            m.edit(ctx, |m| {
                m.content("the clicky bois didn't get a click in time 😭")
                 .components(|c| {
                     c.create_action_row(|r| {
                         r.create_button(|b| {
                             b.custom_id("clickyboi1")
                              .label("no clicky")
                              .disabled(true)
                              .style(ButtonStyle::Danger)
                              .emoji(ReactionType::Unicode("🧠".to_string()))
                         })
                          .create_button(|b| {
                              b.custom_id("clickyboi2")
                               .label("also no clicky")
                               .disabled(true)
                               .style(ButtonStyle::Danger)
                               .emoji(ReactionType::Unicode("🛌".to_string()))
                          })
                     })
                 })
            })
             .await?;
            return Ok(());
        }
    };

    match CollectComponentInteraction::new(ctx)
        .author_id(msg.author.id)
        .channel_id(msg.channel_id)
        .message_id(m.id)
        .collect_limit(1)
        .filter(|x| x.data.custom_id.as_str() == "clickyboi2")
        .timeout(Duration::from_secs(60))
        .await
    {
        Some(inter) => {
            inter
                .create_interaction_response(ctx, |r| {
                    r.kind(InteractionResponseType::ChannelMessageWithSource)
                     .interaction_response_data(|d| {
                         d.content("thanks for clicking the first clickyboi")
                          .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                     })
                })
                .await?;
        }
        None => {
            m.edit(ctx, |m| {
                m.content("the clicky bois didn't get a click in time 😭")
                 .components(|c| {
                     c.create_action_row(|r| {
                         r.create_button(|b| {
                             b.custom_id("clickyboi1")
                              .label("no clicky")
                              .disabled(true)
                              .style(ButtonStyle::Danger)
                              .emoji(ReactionType::Unicode("🧠".to_string()))
                         })
                          .create_button(|b| {
                              b.custom_id("clickyboi2")
                               .label("also no clicky")
                               .disabled(true)
                               .style(ButtonStyle::Danger)
                               .emoji(ReactionType::Unicode("🛌".to_string()))
                          })
                     })
                 })
            })
             .await?;
            return Ok(());
        }
    }

    Ok(())
}

