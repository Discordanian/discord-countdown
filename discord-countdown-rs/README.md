# Discord Countdown Bot (Rust)

Rust rewrite of the Discord countdown bot. See [RUST_REWRITE.md](../RUST_REWRITE.md) for the full game plan.

## Requirements

- Rust 1.74+
- `.env` file with `BOT_TOKEN` and `CLIENT_ID`
- `dates/` directory with `YYYYMMDD.txt` files
- `media.txt` in the project root

## Build

```bash
cargo build --release
```

## Run

Run from the **repository root** (parent of `discord-countdown-rs`) so the bot finds `.env`, `dates/`, and `media.txt`:

```bash
cd /path/to/discord-countdown
cargo run --manifest-path discord-countdown-rs/Cargo.toml
```

Or run the release binary from the repo root:

```bash
./discord-countdown-rs/target/release/discord-countdown
```

> **Note:** Enable the `MESSAGE_CONTENT` privileged intent in the [Discord Developer Portal](https://discord.com/developers/applications) for your bot application.
