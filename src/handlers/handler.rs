use serenity::{
    async_trait,
    client::{Context, EventHandler},
    gateway::ActivityData,
    model::prelude::*,
};
use tracing::info;

use crate::{commands, events};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        commands::register_commands(&ctx).await;

        ctx.set_activity(ActivityData::custom("Probando...").into());

        info!("{} is connected", ready.user.name);
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction.clone() {
            commands::run_command(&ctx, command).await;
        }
    }

    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        events::guild_member_add::run(&ctx, new_member).await;
    }

    async fn guild_member_removal(
        &self,
        ctx: Context,
        guild_id: GuildId,
        user: User,
        member_data_if_available: Option<Member>,
    ) {
        events::guild_member_remove::run(&ctx, guild_id, user, member_data_if_available).await;
    }

    async fn voice_state_update(&self, ctx: Context, old: Option<VoiceState>, new: VoiceState) {
        events::join_to_create::run(&ctx, &old, &new).await;
    }
}
