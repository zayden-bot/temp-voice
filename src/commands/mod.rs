mod bitrate;
mod block;
mod claim;
mod create;
mod delete;
mod invite;
mod join;
mod kick;
mod limit;
mod name;
mod password;
mod privacy;
mod region;
mod reset;
mod transfer;
mod trust;
mod unblock;
mod untrust;

use bitrate::bitrate;
use block::block;
use claim::claim;
use create::create;
use delete::delete;
use invite::invite;
use join::join;
use kick::kick;
use limit::limit;
use name::name;
use password::password;
use privacy::privacy;
use region::region;
use reset::reset;
use transfer::transfer;
use trust::trust;
use unblock::unblock;
use untrust::untrust;

use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    PermissionOverwriteType, Permissions, ResolvedValue,
};

use crate::{error::PermissionError, get_voice_state, Error, Result};

pub struct VoiceCommand;

impl VoiceCommand {
    pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
        let guild_id = interaction.guild_id.ok_or(Error::CommandNotInGuild)?;

        let command = &interaction.data.options()[0];

        let options = match &command.value {
            ResolvedValue::SubCommand(options) => options,
            _ => unreachable!("Subcommand is required"),
        };

        match command.name {
            "create" => {
                create(ctx, interaction, guild_id, options).await?;
                return Ok(());
            }
            "join" => {
                join(ctx, interaction, options, guild_id).await?;
                return Ok(());
            }
            _ => {}
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
            return Err(Error::MissingPermissions(PermissionError::NotTrusted));
        }

        let everyone_role = guild_id.everyone_role();

        match command.name {
            "name" => {
                name(ctx, interaction, options, channel).await?;
            }
            "limit" => {
                limit(ctx, interaction, options, channel).await?;
            }
            "privacy" => {
                privacy(ctx, interaction, options, everyone_role, channel).await?;
            }
            "waiting" => {
                // waiting(ctx, interaction, guild_id, options).await?;
            }
            "trust" => {
                trust(ctx, interaction, options, channel).await?;
            }
            "untrust" => {
                untrust(ctx, interaction, options, channel).await?;
            }
            "invite" => {
                invite(ctx, interaction, options, channel).await?;
            }
            "kick" => {
                kick(ctx, interaction, options, guild_id).await?;
            }
            "region" => {
                region(ctx, interaction, options, channel).await?;
            }
            "block" => {
                block(ctx, interaction, options, guild_id, channel).await?;
            }
            "unblock" => {
                unblock(ctx, interaction, options, channel).await?;
            }
            "claim" => {
                claim(ctx, interaction, channel).await?;
            }
            "transfer" => {
                transfer(ctx, interaction, options, channel).await?;
            }
            "delete" => {
                delete(ctx, interaction, channel).await?;
            }
            "bitrate" => {
                bitrate(ctx, interaction, options, channel).await?;
            }
            "info" => {
                // info(ctx, interaction, guild_id, options).await?;
            }
            "password" => {
                password(ctx, interaction, options, everyone_role, channel).await?;
            }
            "reset" => {
                reset(ctx, interaction, channel, everyone_role).await?;
            }
            _ => unreachable!("Invalid subcommand name"),
        };

        Ok(())
    }

    pub fn register() -> CreateCommand {
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
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::Integer,
                        "user_limit",
                        "The new user limit of the voice channel (0-99).",
                    )
                    .required(true),
                ),
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
                    .add_string_choice("Visible", "visible")
                    .required(true),
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
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "invite",
                    "Invite a user to the voice channel.",
                )
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::User,
                        "user",
                        "The user to invite.",
                    )
                    .required(true),
                ),
            )
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
                    .add_string_choice("US West", "us-west")
                    .required(true),
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
            .add_option(CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "claim",
                "Claim the voice channel as your own.",
            ))
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "transfer",
                    "Transfer ownership of the voice channel.",
                )
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::User,
                        "user",
                        "The user to transfer ownership to.",
                    )
                    .required(true),
                ),
            )
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
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "password",
                    "Set a password for the voice channel.",
                )
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "password",
                        "The password for the voice channel.",
                    )
                    .required(true),
                ),
            )
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "join",
                    "Join a password protected voice channel.",
                )
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::Channel,
                        "channel",
                        "The voice channel to join.",
                    )
                    .required(true)
                    .add_sub_option(
                        CreateCommandOption::new(
                            CommandOptionType::String,
                            "password",
                            "The password for the voice channel.",
                        )
                        .required(true),
                    ),
                ),
            )
            .add_option(CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "reset",
                "Reset the voice channel to default settings.",
            ))
    }
}
