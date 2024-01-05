use serenity::{
    all::{ChannelId, Guild, GuildId, Member, User},
    builder::{CreateEmbed, CreateEmbedFooter, CreateMessage},
    client::Context,
    model::Timestamp,
};
use tracing::error;

use crate::utils::create_embed::get_random_color;

pub async fn run(
    ctx: &Context,
    guild_id: GuildId,
    user: User,
    _member_data_if_available: Option<Member>,
) {
    let guild = Guild::get(&ctx.http, guild_id).await.unwrap();

    let embed_builder = CreateEmbed::new()
        .title(format!("👋 | Adios de {}", guild.name))
        .description(format!("```{} Ha sido removido del servidor```", user.name))
        .thumbnail(user.face())
        .color(get_random_color())
        .timestamp(Timestamp::now())
        .footer(
            CreateEmbedFooter::new(format!(
                "Traido a ti por {}",
                &ctx.cache.current_user().name
            ))
            .icon_url(&ctx.cache.current_user().face()),
        );

    let goodbay_channel = ChannelId::new(1125842067961172099);

    let message_builder = CreateMessage::new().embed(embed_builder);

    if let Err(why) = goodbay_channel
        .send_message(&ctx.http, message_builder)
        .await
    {
        error!("Cannot send goodbay message, why: {}", why);
    }
}
