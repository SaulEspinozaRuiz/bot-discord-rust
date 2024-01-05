mod commands;
mod events;
mod handlers;
mod utils;

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: shuttle_secrets::SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    let token = if let Some(token) = secret_store.get("DISCORD_BOT_TOKEN") {
        token
    } else {
        return Err(anyhow::anyhow!("'DISCORD_BOT_TOKEN' was not found").into());
    };

    let intents = serenity::all::GatewayIntents::all();

    let client = serenity::Client::builder(token, intents)
        .event_handler(handlers::handler::Handler)
        .await
        .expect("Err creating client");

    Ok(client.into())
}
