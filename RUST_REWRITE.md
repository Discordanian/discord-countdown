# Rust Rewrite Game Plan: discord-countdown

## Overview

This document outlines the plan to rewrite the Node.js Discord countdown bot as a Rust project. The bot connects to a private Discord server, reads date files from a `./dates/` directory (named `YYYYMMDD.txt`), and responds to `@mention` commands with countdowns to each future date plus a media count from `media.txt`.

---

## Current Behavior Summary

- Loads bot token and client ID from a `.env` file.
- Connects to Discord using `discord.js` v12.
- Listens for `message` events.
- On any `@mention` to the bot:
  - Reads all files in `./dates/` directory.
  - Parses each filename as `YYYYMMDD` to derive a target date.
  - Calculates the number of days remaining until each future date.
  - Sends a message per future date: `"X days until <file contents>"`.
  - Sends an additional message with the contents of `media.txt`.
- If a message says `"countdown"` (with or without a mention), replies with a help hint.

---

## Technology Choices

| Concern | Rust Crate |
|---|---|
| Discord API | [`serenity`](https://github.com/serenity-rs/serenity) |
| Async runtime | [`tokio`](https://tokio.rs/) |
| `.env` file loading | [`dotenvy`](https://github.com/allan2/dotenvy) |
| Date/time handling | [`chrono`](https://github.com/chronotope/chrono) |
| Filesystem (stdlib) | `std::fs` |
| Error handling | [`anyhow`](https://github.com/dtolnay/anyhow) |

> **Note on Discord library version:** `serenity` is the most widely-used Rust Discord library and supports the current Discord Gateway API. The Node.js project used `discord.js` v12, which used the older message-based command pattern (listening to raw `message` events). The Rust rewrite will use the same pattern for parity, but a future improvement could migrate to Discord's slash commands.

---

## Project Structure

```
discord-countdown-rs/
├── Cargo.toml
├── .env                  # same format as existing .env (BOT_TOKEN, CLIENT_ID)
├── dates/                # same date file convention (YYYYMMDD.txt)
├── media.txt             # same media count file
└── src/
    ├── main.rs           # entry point: setup, login, run bot
    ├── handler.rs        # Discord event handler (EventHandler impl)
    ├── dates.rs          # date file reading and parsing logic
    └── countdown.rs      # countdown calculation logic
```

---

## Implementation Phases

### Phase 1 — Project Scaffolding

- Run `cargo new discord-countdown-rs` to initialize the project.
- Add dependencies to `Cargo.toml`:
  - `serenity` with the `client`, `gateway`, `model`, and `cache` features
  - `tokio` with the `macros` and `rt-multi-thread` features
  - `dotenvy`
  - `chrono`
  - `anyhow`
- Copy the existing `.env`, `dates/`, and `media.txt` into the new project root (or symlink them during development).

### Phase 2 — Environment & Bot Startup (`main.rs`)

- Load `.env` with `dotenvy::dotenv()`.
- Read `BOT_TOKEN` and `CLIENT_ID` from environment variables.
- Build a `serenity::Client` with a `GatewayIntents::GUILD_MESSAGES | DIRECT_MESSAGES | MESSAGE_CONTENT` intent set.
- Attach the event handler.
- Call `client.start().await` to connect.

> **Note on Gateway Intents:** Discord now requires explicit declaration of `MESSAGE_CONTENT` as a privileged intent. This must also be enabled in the Discord Developer Portal for the bot application.

### Phase 3 — Event Handler (`handler.rs`)

Implement `serenity::client::EventHandler` for a `Handler` struct:

- `ready` event: print `"Connected to Discord"` to stdout.
- `message` event:
  - Check if the message content (case-insensitive) equals `"countdown"` — if so, reply with the help hint.
  - Check if the bot's own `CLIENT_ID` is among the mentioned users.
  - If mentioned, delegate to the response logic (Phase 4).

### Phase 4 — Date Loading (`dates.rs`)

- Define a function `load_dates(dir: &str) -> anyhow::Result<HashMap<String, String>>`.
- Read the directory entries from `./dates/`.
- For each file, extract the first 8 characters of the filename as the date key (`YYYYMMDD`).
- Read the file contents as the label/description string.
- Return the populated `HashMap<String, String>`.

> This mirrors the `updateDates()` function in `index.js`. In the Rust version this can be called inline when a message is received, just as it is in the JS version.

### Phase 5 — Countdown Calculation (`countdown.rs`)

- Define a function `days_until(date_key: &str) -> Option<u64>`.
- Parse the `YYYYMMDD` string into a `chrono::NaiveDate`.
- Get today's date via `chrono::Local::now().date_naive()`.
- Calculate the difference in days. If the target date is in the future, return `Some(days)`; otherwise return `None`.

> This mirrors the `calculateTimeTill()` function in `index.js`. The JS version adds `+1` to the day count; replicate this behavior.

### Phase 6 — Message Response Logic (`handler.rs`)

When the bot is mentioned and it is not a bare `"countdown"` message:

1. Call `load_dates("./dates")` to get the date map.
2. Iterate the map; for each entry call `days_until(key)`.
3. For each future date, send a channel message: `"{days} days until {label}"`.
4. Read `media.txt` and send a channel message: `"Media Count :\n{contents}"`.
5. Use `msg.channel_id.say(&ctx.http, ...)` for all outgoing messages.

### Phase 7 — Error Handling & Logging

- Use `anyhow::Result` for all fallible operations.
- Log errors to stderr with `eprintln!` or set up `tracing`/`env_logger` for structured logging.
- If `./dates/` does not exist, log an error and exit gracefully (matching the `process.exit(1)` behavior in the JS version).

### Phase 8 — Build, Test, and Deploy

- `cargo build --release` to produce a self-contained binary.
- Test locally by running the binary with the `.env` file present.
- The resulting binary has no Node.js runtime dependency — deployment is a single file copy.
- Update `.gitignore` to exclude `target/` and the `.env` file.

---

## Key Differences from the Node.js Version

| Topic | Node.js | Rust |
|---|---|---|
| Runtime | Node.js v12 + `discord.js` v12 | Tokio async runtime + `serenity` |
| Discord event model | `client.on('message', ...)` | `EventHandler::message` async trait method |
| Date parsing | Manual substring + `new Date(y, m, d)` | `chrono::NaiveDate::parse_from_str` |
| Error handling | `try/catch` + `process.exit` | `anyhow::Result`, propagated with `?` |
| Deployment artifact | Requires Node.js installed | Single statically-linked binary |
| `.env` loading | `dotenv` npm package | `dotenvy` crate |

---

## Future Improvements (Out of Scope for Initial Rewrite)

- Migrate from legacy message-based commands to Discord slash commands.
- Allow users to submit their own countdown entries via slash commands.
- Persist user-submitted dates to a file or embedded database (e.g., `sled` or `SQLite` via `rusqlite`).
- Add a `!remove` or slash command to delete countdown entries.
- Configurable `dates/` directory path and `media.txt` path via environment variables.
