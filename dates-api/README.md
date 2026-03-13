# dates-api

A REST API for managing the `dates/` directory, built as a [WASI Preview 2](https://wasi.dev/) component.

Runs with [wasmtime](https://wasmtime.dev/) using `wasmtime serve`. The `--dir` flag sandboxes the component so it can **only** read and write the `dates/` directory — nothing else on the filesystem is accessible.

Because the output is a `.wasm` file, it is architecture-independent. Build once on any platform (macOS, Linux, etc.) and run the same binary everywhere wasmtime is installed.

## Requirements

| Tool | Version | Purpose |
|---|---|---|
| Rust (nightly) | ≥ 1.82 | Compiler with `wasm32-wasip2` target |
| [wkg](https://github.com/bytecodealliance/wasm-pkg-tools) | 0.15+ | Fetches WASI WIT interface definitions |
| [wasm-tools](https://github.com/bytecodealliance/wasm-tools) | 1.200+ | Inspect/validate `.wasm` components |
| [wasmtime](https://wasmtime.dev/) | 14+ | Run the component |

### Install Rust target

```bash
rustup target add wasm32-wasip2
```

### Install wkg

```bash
cargo install wkg
```

### Install wasmtime

```bash
curl https://wasmtime.dev/install.sh -sSf | bash
```

## Build

Run from the `dates-api/` directory:

```bash
# 1. Fetch WASI WIT interface definitions (only needed once, or after wkg.lock changes)
wkg wit fetch --type wit

# 2. Compile
cargo build --release
```

Output: `target/wasm32-wasip2/release/dates_api.wasm`

> `wit/deps/` is populated by `wkg wit fetch` and is gitignored. `wkg.lock` is committed to pin
> the exact WASI interface versions used.

### Inspect the component (optional)

```bash
wasm-tools component wit target/wasm32-wasip2/release/dates_api.wasm
```

## Run

Run from the **repository root** so the `dates/` directory is in the current working directory:

```bash
wasmtime serve \
  -S cli \
  --dir ./dates::/dates \
  dates-api/target/wasm32-wasip2/release/dates_api.wasm
```

**`-S cli`** is required. `wasmtime serve` runs in HTTP-proxy mode by default and does not provide the WASI CLI interface (`wasi:cli/environment`, etc.). The Rust standard library always imports these even when unused, so the flag must be present or wasmtime will refuse to link the component.

**`--dir ./dates::/dates`** maps the host's `./dates/` to `/dates` inside the WASM sandbox. The component **cannot** access any other path.

By default `wasmtime serve` listens on `0.0.0.0:8080`. To change the address:

```bash
wasmtime serve -S cli --addr 127.0.0.1:3000 --dir ./dates::/dates \
  dates-api/target/wasm32-wasip2/release/dates_api.wasm
```

### Arch Linux
`ufw allow <PORT>/tcp` if you need to open a public hole in the firewall

## API

### `GET /dates`

Returns a JSON array of all date entries.

```bash
curl http://localhost:8080/dates
```

```json
[
  {"key":"20251225","label":"Christmas Day"},
  {"key":"20260101","label":"New Year 2026"}
]
```

### `POST /dates/{YYYYMMDD}`

Creates a date entry. The request body is the label text.

```bash
curl -X POST http://localhost:8080/dates/20251225 \
  -d "Christmas Day"
```

```
Created 20251225
```

### `DELETE /dates/{YYYYMMDD}`

Removes a date entry.

```bash
curl -X DELETE http://localhost:8080/dates/20251225
```

```
Deleted 20251225
```

### `GET /`

Returns available routes.

## File format

Each date is stored as a `.txt` file in `dates/` named `YYYYMMDD.txt`. The filename prefix (first 8 characters) is the date key; the file contents are the label. This is the same format the Discord countdown bot reads.

## Integration with the Discord bot

The Discord bot (`discord-countdown-rs`) reads directly from `./dates/` at runtime. Any changes made through this API are immediately visible to the bot — no restart required.
