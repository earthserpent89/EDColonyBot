// This file is the entry point of the application. It initializes the Discord bot and sets up the necessary components, including command handling and event listening.

use serenity::prelude::*;
use serenity::model::gateway::GatewayIntents;
use serenity::Client;
use std::env;

mod bot;
mod data;
mod elite;

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(bot::EventHandler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}