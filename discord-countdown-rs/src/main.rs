//! Discord countdown bot — entry point.

mod countdown;
mod dates;
mod handler;

use handler::Handler;
use serenity::model::gateway::GatewayIntents;
use serenity::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let token = std::env::var("BOT_TOKEN").expect("BOT_TOKEN must be set");
    let client_id = std::env::var("CLIENT_ID").expect("CLIENT_ID must be set");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let handler = Handler { client_id };

    let mut client = Client::builder(&token, intents)
        .event_handler(handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        eprintln!("Client error: {why:?}");
        std::process::exit(1);
    }

    Ok(())
}
