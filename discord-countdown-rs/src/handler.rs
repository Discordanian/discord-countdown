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

const HELP_MESSAGE: &str = "**CountDownBot options:**\n\
• `media` — display media count\n\
• `dates` — show countdowns for all configured dates\n\
• `<date>` — pass a date (e.g. 2025-12-25, 12/25/2025) to get days until that date";

pub struct Handler {
    pub client_id: String,
}

/// Strip the bot mention from message content and return the trimmed argument.
fn extract_command(content: &str, client_id: &str) -> String {
    let mut s = content.to_string();
    // Remove <@123> and <@!123> mention formats
    let patterns = [
        format!("<@!{}>", client_id),
        format!("<@{}>", client_id),
    ];
    for p in &patterns {
        s = s.replace(p, "");
    }
    s.trim().to_string()
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, _data_about_bot: serenity::model::gateway::Ready) {
        println!("Connected to Discord");
    }

    async fn message(&self, ctx: Context, msg: Message) {
        // Check if the bot is mentioned
        let mention = msg.mentions.iter().find(|u| u.id.to_string() == self.client_id);
        if mention.is_none() {
            return;
        }

        let cmd = extract_command(&msg.content, &self.client_id);
        let cmd_lower = cmd.to_lowercase();

        println!("Content [{}] -> command [{}]", msg.content, cmd);

        // Empty or help-like -> show options
        if cmd_lower.is_empty() || cmd_lower == "countdown" || cmd_lower == "help" {
            let _ = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await;
            return;
        }

        // "media" -> media.txt only
        if cmd_lower == "media" {
            let media_contents = match fs::read_to_string(MEDIA_FILE) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Could not read media.txt: {e}");
                    "".to_string()
                }
            };
            let media_msg = format!("Media Count :\n{}", media_contents);
            let _ = msg.channel_id.say(&ctx.http, &media_msg).await;
            return;
        }

        // "dates" -> dates logic (all configured dates)
        if cmd_lower == "dates" {
            let dates_map = match dates::load_dates(DATES_DIR) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("Could not load dates: {e}");
                    let _ = msg.channel_id.say(&ctx.http, "Error loading dates directory.").await;
                    return;
                }
            };

            for (k, v) in &dates_map {
                if let Some(days) = countdown::days_until(k) {
                    let text = format!("{} days until {}", days, v.trim());
                    let _ = msg.channel_id.say(&ctx.http, &text).await;
                }
            }
            return;
        }

        // Try to parse as date
        if countdown::parse_date(&cmd).is_some() {
            if let Some(days) = countdown::days_until_from_str(&cmd) {
                let text = format!("{} days until {}", days, cmd.trim());
                let _ = msg.channel_id.say(&ctx.http, &text).await;
            } else {
                let _ = msg.channel_id.say(&ctx.http, "That date has already passed.").await;
            }
            return;
        }

        // Unrecognized
        let _ = msg.channel_id.say(&ctx.http, format!("Unrecognized command. {HELP_MESSAGE}")).await;
    }
}
