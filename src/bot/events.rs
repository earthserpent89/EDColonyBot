// This file handles events emitted by Discord, such as message creation or user status changes.
// It exports functions that respond to these events.

use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::model::event::GuildMemberUpdateEvent;

pub async fn message_create(ctx: Context, msg: Message) {
    // Handle message creation events here
    if msg.author.id == ctx.cache.current_user.id {
        return; // Ignore messages from the bot itself
    }

    // Example: Respond to a specific command
    if msg.content == "!track" {
        if let Err(why) = msg.channel_id.say(&ctx.http, "Tracking construction sites!").await {
            println!("Error sending message: {:?}", why);
        }
    }
}

pub async fn guild_member_update(ctx: Context, _old: Option<GuildMember>, new: GuildMember) {
    // Handle guild member updates here
    println!("Member updated: {:?}", new.user.name);
}