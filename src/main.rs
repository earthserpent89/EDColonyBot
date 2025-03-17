
use std::env;
use dotenv::dotenv;
use poise::serenity_prelude as serenity;
use serenity::prelude::*;
use serenity::GuildId;
use tracing::{error, info};
use anyhow::{Result, Error};
use file_sync::FileSync;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Commodity {
    name: String,
    delivered: u64,
    required: u64,
}


#[derive(Serialize, Deserialize)]
struct Site {
    name: String,
    commodities: Vec<Commodity>
}

#[derive(Serialize, Deserialize)]
struct System {
    name: String,
    sites: Vec<Site>
}

struct Data {
    servers: FileSync<HashMap<GuildId, Vec<System>>>
}

type Context<'a> = poise::Context<'a, Data, Error>;
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

    // Initialize logging
    tracing_subscriber::fmt::init();
    
    // Get Discord token
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    
    let intents = serenity::GatewayIntents::empty();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions { commands: vec![echo()], ..Default::default()})
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    servers: FileSync::load_or_new(file_sync::Path::new("./servers.json"), HashMap::new(), true)?
                })
            })
        })
        .build();
        
    // Build client
    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .await
        .expect("Error creating client");
    
    // Start client
    client.start().await?;
    
    Ok(())
}
