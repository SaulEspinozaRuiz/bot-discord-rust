use serenity::{
    async_trait,
    model::prelude::*,
    prelude::{Context, EventHandler},
};
use tracing::info;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("{} is connected", ready.user.name);
    }
}