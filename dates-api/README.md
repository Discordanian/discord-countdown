# dates-api

A REST API for managing the `dates/` directory, built as a [WASI preview2](https://wasi.dev/) component.

Runs with [wasmtime](https://wasmtime.dev/) using `wasmtime serve`. The `--dir` flag sandboxes the component so it can **only** read and write the `dates/` directory — nothing else on the filesystem is accessible.

## Requirements

- Rust with `wasm32-wasip2` target
- wasmtime CLI (v14+)

```bash
rustup target add wasm32-wasip2
curl https://wasmtime.dev/install.sh -sSf | bash
```

## Build

Run from the `dates-api/` directory:

```bash
cargo build --release
```

Output: `target/wasm32-wasip2/release/dates_api.wasm`

## Run

Run from the **repository root** so the `dates/` directory is in the current working directory:

```bash
wasmtime serve \
  --dir ./dates::/dates \
  dates-api/target/wasm32-wasip2/release/dates_api.wasm
```

The `--dir ./dates::/dates` flag maps the host's `./dates/` to `/dates` inside the WASM sandbox. The component **cannot** access any other path.

By default wasmtime serve listens on `127.0.0.1:8080`. To change the address:

```bash
wasmtime serve --addr 0.0.0.0:3000 --dir ./dates::/dates dates-api/target/wasm32-wasip2/release/dates_api.wasm
```

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
