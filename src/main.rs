
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
#[poise::command(slash_command)]
async fn system_add(ctx: Context<'_>, new_system_name: String) -> Result<()> {
    let gid = match ctx.guild_id() {
        Some(gid) => gid,
        None => {
            ctx.say("This is not a guild/server").await?;
            return Ok(());
        }
    };
    let reply = ctx.say("Processing...").await?;
    let mut message = String::from("");
    ctx.data()
        .servers
        .lock()
        .await
        .modify(|servers| match servers.get_mut(&gid) {
            Some(server) => {
                for system in server.iter() {
                    if system.name.to_lowercase() == new_system_name {
                        message = "That system is already registered in this server".into();
                        return;
                    }
                }
                message = format!("System {new_system_name} registered");
                server.push(System {
                    name: new_system_name,
                    sites: Vec::new(),
                });
            }
            None => {
                message = format!("System {new_system_name} registered");
                servers.insert(
                    gid,
                    vec![System {
                        name: new_system_name,
                        sites: Vec::new(),
                    }],
                );
            }
        })
        .unwrap();
    reply
        .edit(ctx, CreateReply::default().content(message))
        .await?;

    Ok(())
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
        .options(poise::FrameworkOptions {
            commands: vec![system_add()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    servers: Mutex::new(FileSync::load_or_new(
                        file_sync::Path::new("./servers.json"),
                        HashMap::new(),
                        true,
                    )?),
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
