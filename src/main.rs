mod handler;

use handler::handler::Handler;

use anyhow::anyhow;
use serenity::{prelude::GatewayIntents, Client};

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: shuttle_secrets::SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    let token = if let Some(token) = secret_store.get("DISCORD_BOT_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_BOT_TOKEN' was not found").into());
    };
    let intents = GatewayIntents::all();

    let client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    Ok(client.into())
}
