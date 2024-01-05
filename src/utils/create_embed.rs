use rand::Rng;
use serenity::{
    builder::{CreateEmbed, CreateEmbedFooter},
    client::Context,
    model::{Colour, Timestamp},
};

pub fn embed(ctx: &Context, title: &str, description: &str) -> CreateEmbed {
    let footer_message = format!("Traido a ti por {}", &ctx.cache.current_user().name);
    let footer_avatar_url = &ctx.cache.current_user().avatar_url().unwrap();

    CreateEmbed::new()
        .title(title)
        .description(description)
        .color(get_random_color())
        .timestamp(Timestamp::now())
        .footer(CreateEmbedFooter::new(footer_message).icon_url(footer_avatar_url))
}

pub fn get_random_color() -> Colour {
    let mut rng = rand::thread_rng();

    Colour::from_rgb(
        rng.gen_range(0..255),
        rng.gen_range(0..255),
        rng.gen_range(0..255),
    )
}
