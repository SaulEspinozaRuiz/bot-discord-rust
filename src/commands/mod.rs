use serenity::{
    all::{Command, CommandInteraction},
    client::Context,
};
use tracing::error;

pub mod misc;

pub async fn register_commands(ctx: &Context) {
    let mut commands = vec![];

    // Misc
    commands.push(misc::ping::register());

    for command in commands {
        if let Err(why) = Command::create_global_command(&ctx.http, command).await {
            error!("Cannot register command, why: {}", why);
        }
    }
}

pub async fn run_command(ctx: &Context, interaction: CommandInteraction) {
    match interaction.data.name.as_str() {
        // Misc
        "ping" => misc::ping::run(&ctx, interaction).await,

        _ => {
            Command::delete_global_command(&ctx.http, interaction.data.id)
                .await
                .unwrap();

            error!("Cannot execute command");
        }
    }
}
