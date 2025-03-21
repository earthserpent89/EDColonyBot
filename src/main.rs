use anyhow::{Error, Result};
use dotenv::dotenv;
use file_sync::FileSync;
use poise::CreateReply;
use poise::serenity_prelude as serenity;
use serde::{Deserialize, Serialize};
use serenity::GuildId;
use serenity::prelude::*;
use std::collections::HashMap;
use std::env;
use tokio::sync::Mutex;

#[allow(unused_imports)]
#[rustfmt::skip]
use tracing::{trace, debug, info, warn, error};

mod commodities;

#[derive(Serialize, Deserialize)]
struct Commodity {
    r#type: commodities::CommodityType,
    delivered: u64,
    required: u64,
}

#[derive(Serialize, Deserialize)]
struct Site {
    name: String,
    commodities: Vec<Commodity>,
}

impl Site {
    fn new(name: String) -> Site {
        Site {
            name,
            commodities: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct System {
    name: String,
    sites: Vec<Site>,
}

struct Data {
    servers: Mutex<FileSync<HashMap<GuildId, Vec<System>>>>,
}

type Context<'a> = poise::Context<'a, Data, Error>;

// system add <name>
// system remove <name>
// site add <system> <name>
// site remove <system> <name>
// commodity add <system> <site> <commodity> <amount>
// commodity remove <system> <site> <commodity>
// deliver <system> <site> <commodity> <amount>

#[poise::command(slash_command)]
async fn system_add(ctx: Context<'_>, new_system_name: String) -> Result<()> {
    info!(
        "system_add command invoked with new_system_name: {new_system_name}",
    );
    let gid = match ctx.guild_id() {
        Some(gid) => gid,
        None => {
            warn!("system_add called outside of a guild/server");
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
                        warn!(
                            "System {} already exists in server {}",
                            new_system_name, gid
                        );
                        message = "That system is already registered in this server".into();
                        return;
                    }
                }
                info!(
                    "Registering new system: {new_system_name} in server {gid}",
                );
                message = format!("System {new_system_name} registered");
                server.push(System {
                    name: new_system_name,
                    sites: Vec::new(),
                });
            }
            None => {
                info!(
                    "Registering new system: {new_system_name} in a new server entry {gid}",
                );
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
    info!("system_add command completed");

    Ok(())
}

#[poise::command(slash_command)]
async fn system_remove(ctx: Context<'_>, system_name: String) -> Result<()> {
    info!(
        "system_remove command invoked with system_name: {}",
        system_name
    );
    let gid = match ctx.guild_id() {
        Some(gid) => gid,
        None => {
            warn!("system_remove called outside of a guild/server");
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
                let index_to_remove = server
                    .iter()
                    .enumerate()
                    .find(|(_index, system)| system.name.to_lowercase() == system_name)
                    .map(|(index, _system)| index);
                match index_to_remove {
                    Some(index) => {
                        info!("Removing system: {system_name} from server {gid}");
                        server.remove(index);
                        message = "Done".into();
                    }
                    None => {
                        warn!("System {system_name} not found in server {gid}");
                        message = "That system is not registered in this server".into();
                    }
                }
            }
            None => {
                warn!("No systems registered in server {gid}");
                message = "There are no systems registered in this server".into();
            }
        })
        .unwrap();
    reply
        .edit(ctx, CreateReply::default().content(message))
        .await?;
    info!(
        "system_remove command completed for system_name: {system_name}",
    );

    Ok(())
}

#[poise::command(slash_command)]
async fn site_add(ctx: Context<'_>, system_name: String, new_site_name: String) -> Result<()> {
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
                for system in server.iter_mut() {
                    if system.name.to_lowercase() == system_name {
                        for site in &system.sites {
                            if site.name == new_site_name {
                                message = "That site is already registered in that system".into();
                                return;
                            }
                        }
                        message =
                            format!("Site {new_site_name} registered in system {system_name}");
                        system.sites.push(Site::new(new_site_name));
                        return;
                    }
                }
                message = "That is not a system that is registered in this server".into();
            }
            None => {
                message = "There are no systems registered in this server".into();
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
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = serenity::GatewayIntents::empty();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![system_add(), system_remove(), site_add()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                info!("Commands: {:#?}", framework.options().commands);
                //poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                poise::builtins::register_in_guild(
                    ctx,
                    &framework.options().commands,
                    GuildId::new(1207703559240679474),
                )
                .await?;
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
