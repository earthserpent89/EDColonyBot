mod commands;
mod models;
mod db;
mod utils;

use std::env;
use dotenv::dotenv;
use serenity::async_trait;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;
use tracing::{error, info};

struct Handler {
    database: db::Database,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        // Register slash commands
        if let Ok(guild_id) = env::var("DISCORD_GUILD_ID") {
            if let Ok(guild_id) = guild_id.parse::<u64>() {
                let guild_id = GuildId(guild_id);
                
                // Here we'll register the slash commands
                match commands::register_commands(&ctx.http, guild_id).await {
                    Ok(_) => info!("Successfully registered slash commands"),
                    Err(e) => error!("Error registering slash commands: {}", e),
                }
            }
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            info!("Received command: {}", command.data.name);
            
            // Handle slash commands
            let result = commands::handle_command(&ctx, &command, &self.database).await;
            
            if let Err(e) = result {
                error!("Error handling command: {}", e);
                
                // Respond with an error message
                if let Err(e) = command
                    .create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| {
                                message
                                    .content("An error occurred while processing your command.")
                                    .ephemeral(true)
                            })
                    })
                    .await
                {
                    error!("Error sending error response: {}", e);
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv().ok();
    
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    // Initialize database
    let database = db::Database::new().await?;
    
    // Get Discord token
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    
    // Set gateway intents
    let intents = GatewayIntents::GUILD_MESSAGES 
        | GatewayIntents::GUILDS 
        | GatewayIntents::MESSAGE_CONTENT;
    
    // Build client
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler { database })
        .await
        .expect("Error creating client");
    
    // Start client
    client.start().await?;
    
    Ok(())
}