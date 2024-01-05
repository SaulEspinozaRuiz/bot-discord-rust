use serenity::{
    all::{ChannelId, ChannelType, Guild, VoiceState},
    builder::CreateChannel,
    client::Context,
};

// 1192697141479604265 - test
// 1168809502380933130 - real
const JOIN_TO_CREATE_CHANNEL: ChannelId = ChannelId::new(1168809502380933130);

pub async fn run(ctx: &Context, old: &Option<VoiceState>, new: &VoiceState) {
    if new.channel_id.is_none() {
        on_voice_channel_leave(ctx, old.as_ref().unwrap()).await;
        return;
    }

    if let Some(v) = old {
        on_voice_channel_leave(ctx, v).await;
    }

    on_voice_channel_join(ctx, new).await;
}

async fn on_voice_channel_join(ctx: &Context, state: &VoiceState) {
    let guild_id = state.guild_id.unwrap();
    let guild = Guild::get(&ctx.http, guild_id).await.unwrap();

    let channels = guild.channels(&ctx.http).await.unwrap();

    let channel_id = state.channel_id.unwrap();
    let channel = channels.get(&channel_id).unwrap();

    let user_id = state.user_id;
    let user = guild.member(&ctx.http, user_id).await.unwrap();

    let category_id = channel.parent_id.unwrap();

    if channel.id == JOIN_TO_CREATE_CHANNEL {
        let new_channel = CreateChannel::new(format!("『🔊』 | Canal de {}", user.display_name()))
            .category(category_id)
            .kind(ChannelType::Voice);

        let create_new_channel = guild.create_channel(&ctx.http, new_channel).await.unwrap();

        guild
            .move_member(&ctx.http, user_id, create_new_channel.id)
            .await
            .unwrap();
    }
}

async fn on_voice_channel_leave(ctx: &Context, state: &VoiceState) {
    let guild_id = state.guild_id.unwrap();
    let guild = Guild::get(&ctx.http, guild_id).await.unwrap();

    let channels = guild.channels(&ctx.http).await.unwrap();

    let channel_id = state.channel_id.unwrap();
    let channel = channels.get(&channel_id).unwrap();

    let member_count = match channel.members(&ctx.cache) {
        Ok(v) => v.len(),
        _ => return,
    };

    if member_count != 0 || channel.id == JOIN_TO_CREATE_CHANNEL {
        return;
    }

    channel.delete(&ctx.http).await.unwrap();
}
