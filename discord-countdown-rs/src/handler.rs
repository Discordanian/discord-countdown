//! Discord event handler — ready and message events.

use crate::countdown;
use crate::dates;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
use std::fs;

/// Paths relative to current working directory. Run from repo root so .env, dates/, and media.txt are found.
const DATES_DIR: &str = "./dates";
const MEDIA_FILE: &str = "./media.txt";

pub struct Handler {
    pub client_id: String,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, _data_about_bot: serenity::model::gateway::Ready) {
        println!("Connected to Discord");
    }

    async fn message(&self, ctx: Context, msg: Message) {
        // Help hint for bare "countdown" (case-insensitive)
        if msg.content.eq_ignore_ascii_case("countdown") {
            if let Err(why) = msg.channel_id.say(&ctx.http, "I see a request for help, just <AT> me").await {
                eprintln!("Error sending message: {why:?}");
            }
        }

        // Check if the bot is mentioned
        let mention = msg.mentions.iter().find(|u| u.id.to_string() == self.client_id);
        if mention.is_none() {
            return;
        }

        // If mentioned with only "countdown", show help
        if msg.content.eq_ignore_ascii_case("countdown") {
            if let Err(why) = msg.channel_id.say(&ctx.http, "I see a request for help, just <AT> me").await {
                eprintln!("Error sending message: {why:?}");
            }
            return;
        }

        // Load dates and send countdowns
        let dates = match dates::load_dates(DATES_DIR) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("Could not load dates: {e}");
                if let Err(why) = msg.channel_id.say(&ctx.http, "Error loading dates directory.").await {
                    eprintln!("Error sending message: {why:?}");
                }
                return;
            }
        };

        println!("Content [{}]", msg.content);

        for (k, v) in &dates {
            if let Some(days) = countdown::days_until(k) {
                let text = format!("{} days until {}", days, v.trim());
                if let Err(why) = msg.channel_id.say(&ctx.http, &text).await {
                    eprintln!("Error sending message: {why:?}");
                }
            }
        }

        // Send media count
        let media_contents = match fs::read_to_string(MEDIA_FILE) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Could not read media.txt: {e}");
                "".to_string()
            }
        };
        let media_msg = format!("Media Count :\n{}", media_contents);
        if let Err(why) = msg.channel_id.say(&ctx.http, &media_msg).await {
            eprintln!("Error sending message: {why:?}");
        }
    }
}
