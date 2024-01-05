use serenity::{
    all::{ChannelId, Guild, GuildId, Member, RoleId},
    builder::{CreateEmbed, CreateEmbedFooter, CreateMessage},
    client::Context,
    model::Timestamp,
};
use tracing::error;

use crate::utils::create_embed::get_random_color;

pub async fn run(ctx: &Context, new_member: Member) {
    let guild = Guild::get(&ctx.http, GuildId::new(new_member.guild_id.get()))
        .await
        .unwrap();

    let member_role = RoleId::new(1042301131377872907);

    if let Err(why) = new_member.add_role(&ctx.http, member_role).await {
        error!("Cannot add role to new member, why: {}", why);
    }

    let embed_builder = CreateEmbed::new()
        .title(format!("👋 | Bienvenido a {}", guild.name))
        .description(format!(
            "```{} Se ha unido al servidor```",
            new_member.user.name
        ))
        .thumbnail(new_member.face())
        .color(get_random_color())
        .timestamp(Timestamp::now())
        .footer(
            CreateEmbedFooter::new(format!(
                "Traido a ti por {}",
                &ctx.cache.current_user().name
            ))
            .icon_url(&ctx.cache.current_user().face()),
        );

    let welcome_channel = ChannelId::new(1125842038622003251);

    let message_builder = CreateMessage::new().embed(embed_builder);

    if let Err(why) = welcome_channel
        .send_message(&ctx.http, message_builder)
        .await
    {
        error!("Cannot send welcome message, why: {}", why);
    }
}
