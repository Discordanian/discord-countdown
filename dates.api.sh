#!sh

echo wasmtime serve --dir ./dates::/dates dates-api/tager/wasm32-wasip2/release/dates_api.wasm
wasmtime serve --dir ./dates::/dates dates-api/tager/wasm32-wasip2/release/dates_api.wasm
