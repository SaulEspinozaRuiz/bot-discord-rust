use serenity::{
    all::CommandInteraction,
    builder::{CreateCommand, CreateInteractionResponse, CreateInteractionResponseMessage},
    client::Context,
};
use tracing::error;

use crate::utils::create_embed::embed;

pub fn register() -> CreateCommand {
    CreateCommand::new("ping").description("🏓 | Devuelve un mensaje")
}

// TODO: Make the response a followup response
pub async fn run(ctx: &Context, interaction: CommandInteraction) {
    let embed_builder = embed(&ctx, "🏓 | Pong!", "```Proximamente el ping```");

    let message_builder = CreateInteractionResponseMessage::new().embed(embed_builder);

    let builder = CreateInteractionResponse::Message(message_builder);

    if let Err(why) = interaction.create_response(&ctx.http, builder).await {
        error!("Cannot respond to command, why: {}", why);
    }
}
