pub mod commands;
mod error;
pub mod events;

use std::collections::HashMap;

use serenity::all::{
    ChannelId, CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    GuildId, LightMethod, PermissionOverwriteType, Permissions, Request, ResolvedValue, Route,
    UserId, VoiceState,
};
use serenity::prelude::TypeMapKey;

pub use error::Error;
use error::Result;

struct VoiceStateCache;

impl TypeMapKey for VoiceStateCache {
    type Value = HashMap<UserId, Option<ChannelId>>;
}

pub struct VoiceCommand;

impl VoiceCommand {
    async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
        let guild_id = interaction.guild_id.ok_or(Error::CommandNotInGuild)?;

        let command = &interaction.data.options()[0];

        let options = match &command.value {
            ResolvedValue::SubCommand(options) => options,
            _ => unreachable!("Subcommand is required"),
        };

        if command.name == "create" {
            commands::create(ctx, interaction, guild_id, options).await?;
            return Ok(());
        }

        let voice_state = get_voice_state(ctx, guild_id, interaction.user.id)
            .await
            .map_err(|_| Error::MemberNotInVoiceChannel)?;

        let voice_channel = voice_state
            .channel_id
            .ok_or(Error::MemberNotInVoiceChannel)?;

        let mut channels = guild_id.channels(ctx).await?;
        let channel = channels
            .remove(&voice_channel)
            .expect("Voice channel should exist if member is in it");

        if !channel.permission_overwrites.iter().any(|overwrite| {
            overwrite.kind == PermissionOverwriteType::Member(interaction.user.id)
                && overwrite.allow.contains(Permissions::MANAGE_CHANNELS)
        }) {
            return Err(Error::MissingPermissions);
        }

        match command.name {
            "name" => {
                commands::name(ctx, interaction, options, channel).await?;
            }
            "limit" => {
                commands::limit(ctx, interaction, options, channel).await?;
            }
            "privacy" => {
                commands::privacy(ctx, interaction, options, guild_id.everyone_role(), channel)
                    .await?;
            }
            "waiting" => {
                // waiting(ctx, interaction, guild_id, options).await?;
            }
            "trust" => {
                commands::trust(ctx, interaction, options, channel).await?;
            }
            "untrust" => {
                commands::untrust(ctx, interaction, options, channel).await?;
            }
            "invite" => {
                // invite(ctx, interaction, guild_id, options).await?;
            }
            "kick" => {
                commands::kick(ctx, interaction, options, guild_id).await?;
            }
            "region" => {
                commands::region(ctx, interaction, options, channel).await?;
            }
            "block" => {
                commands::block(ctx, interaction, options, channel).await?;
            }
            "unblock" => {
                commands::unblock(ctx, interaction, options, channel).await?;
            }
            "claim" => {
                // claim(ctx, interaction, guild_id, options).await?;
            }
            "transfer" => {
                // transfer(ctx, interaction, guild_id, options).await?;
            }
            "delete" => {
                commands::delete(ctx, interaction, channel).await?;
            }
            "bitrate" => {
                commands::bitrate(ctx, interaction, options, channel).await?;
            }
            "info" => {
                // info(ctx, interaction, guild_id, options).await?;
            }
            "password" => {
                // password(ctx, interaction, guild_id, options).await?;
            }
            "join" => {
                // join(ctx, interaction, guild_id, options).await?;
            }
            "reset" => {
                // reset(ctx, interaction, guild_id, options).await?;
            }
            _ => unreachable!("Invalid subcommand name"),
        };

        Ok(())
    }

    fn register() -> CreateCommand {
        CreateCommand::new("voice")
            .description("Commands for creating and managing temporary voice channels.")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "create",
                    "Create a temporary voice channel.",
                )
                .add_sub_option(CreateCommandOption::new(
                    CommandOptionType::String,
                    "name",
                    "The name of the voice channel.",
                ))
                .add_sub_option(CreateCommandOption::new(
                    CommandOptionType::Integer,
                    "limit",
                    "The user limit of the voice channel (0-99).",
                ))
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "privacy",
                        "Lock or hide the voice channel.",
                    )
                    .add_string_choice("Lock", "lock")
                    .add_string_choice("Unlock", "unlock")
                    .add_string_choice("Invisible", "invisible")
                    .add_string_choice("Visible", "visible"),
                ),
            )
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "name",
                    "Change the name of the voice channel.",
                )
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "name",
                        "The new name of the voice channel.",
                    )
                    .required(true),
                ),
            )
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "limit",
                    "Change the user limit of the voice channel.",
                )
                .add_sub_option(CreateCommandOption::new(
                    CommandOptionType::Integer,
                    "user_limit",
                    "The new user limit of the voice channel (0-99).",
                )),
            )
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "privacy",
                    "Change the privacy of the voice channel.",
                )
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "privacy",
                        "The new privacy of the voice channel.",
                    )
                    .add_string_choice("Lock", "lock")
                    .add_string_choice("Unlock", "unlock")
                    .add_string_choice("Invisible", "invisible")
                    .add_string_choice("Visible", "visible"),
                ),
            )
            // .add_option(CreateCommandOption::new(
            //     CommandOptionType::SubCommand,
            //     "waiting",
            //     "Create a waiting room for the voice channel.",
            // ))
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "trust",
                    "Trusted users have access to the voice channel.",
                )
                .add_sub_option(
                    CreateCommandOption::new(CommandOptionType::User, "user", "The user to trust.")
                        .required(true),
                ),
            )
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "untrust",
                    "Remove trusted users access from the voice channel.",
                )
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::User,
                        "user",
                        "The user to untrust.",
                    )
                    .required(true),
                ),
            )
            // .add_option(
            //     CreateCommandOption::new(
            //         CommandOptionType::SubCommand,
            //         "invite",
            //         "Invite a user to the voice channel.",
            //     )
            //     .add_sub_option(
            //         CreateCommandOption::new(
            //             CommandOptionType::User,
            //             "user",
            //             "The user to invite.",
            //         )
            //         .required(true),
            //     ),
            // )
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "kick",
                    "Kick a user from the voice channel.",
                )
                .add_sub_option(
                    CreateCommandOption::new(CommandOptionType::User, "user", "The user to kick.")
                        .required(true),
                ),
            )
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "region",
                    "Change the region of the voice channel.",
                )
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "region",
                        "The new region of the voice channel.",
                    )
                    .add_string_choice("Brazil", "brazil")
                    .add_string_choice("Hong Kong", "hongkong")
                    .add_string_choice("India", "india")
                    .add_string_choice("Japan", "japan")
                    .add_string_choice("Rotterdam", "rotterdam")
                    .add_string_choice("Russia", "russia")
                    .add_string_choice("Singapore", "singapore")
                    .add_string_choice("South Africa", "southafrica")
                    .add_string_choice("Sydney", "sydney")
                    .add_string_choice("US Central", "us-central")
                    .add_string_choice("US East", "us-east")
                    .add_string_choice("US South", "us-south")
                    .add_string_choice("US West", "us-west"),
                ),
            )
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "block",
                    "Block a user from the voice channel.",
                )
                .add_sub_option(
                    CreateCommandOption::new(CommandOptionType::User, "user", "The user to block.")
                        .required(true),
                ),
            )
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "unblock",
                    "Unblock a user from the voice channel.",
                )
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::User,
                        "user",
                        "The user to unblock.",
                    )
                    .required(true),
                ),
            )
            // .add_option(CreateCommandOption::new(
            //     CommandOptionType::SubCommand,
            //     "claim",
            //     "Claim the voice channel as your own.",
            // ))
            // .add_option(
            //     CreateCommandOption::new(
            //         CommandOptionType::SubCommand,
            //         "transfer",
            //         "Transfer ownership of the voice channel.",
            //     )
            //     .add_sub_option(
            //         CreateCommandOption::new(
            //             CommandOptionType::User,
            //             "user",
            //             "The user to transfer ownership to.",
            //         )
            //         .required(true),
            //     ),
            // )
            .add_option(CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "delete",
                "Delete the voice channel.",
            ))
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "bitrate",
                    "Change the bitrate of the voice channel.",
                )
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::Integer,
                        "bitrate",
                        "The new bitrate of the voice channel.",
                    )
                    .required(true),
                ),
            )
        // .add_option(CreateCommandOption::new(
        //     CommandOptionType::SubCommand,
        //     "info",
        //     "Get information about the voice channel.",
        // ))
        // .add_option(
        //     CreateCommandOption::new(
        //         CommandOptionType::SubCommand,
        //         "password",
        //         "Set a password for the voice channel.",
        //     )
        //     .add_sub_option(
        //         CreateCommandOption::new(
        //             CommandOptionType::String,
        //             "password",
        //             "The password for the voice channel.",
        //         )
        //         .required(true),
        //     ),
        // )
        // .add_option(
        //     CreateCommandOption::new(
        //         CommandOptionType::SubCommand,
        //         "join",
        //         "Join a password protected voice channel.",
        //     )
        //     .add_sub_option(
        //         CreateCommandOption::new(
        //             CommandOptionType::Channel,
        //             "channel",
        //             "The voice channel to join.",
        //         )
        //         .required(true)
        //         .add_sub_option(
        //             CreateCommandOption::new(
        //                 CommandOptionType::String,
        //                 "password",
        //                 "The password for the voice channel.",
        //             )
        //             .required(true),
        //         ),
        //     ),
        // )
        // .add_option(CreateCommandOption::new(
        //     CommandOptionType::SubCommand,
        //     "reset",
        //     "Reset the voice channel to default settings.",
        // ))
    }
}

async fn get_voice_state(
    ctx: &Context,
    guild_id: GuildId,
    user_id: UserId,
) -> serenity::Result<VoiceState> {
    ctx.http
        .fire::<VoiceState>(Request::new(
            Route::GuildVoiceStates { guild_id, user_id },
            LightMethod::Get,
        ))
        .await
}
