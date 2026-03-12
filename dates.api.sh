#!sh

export PORT=19697
export DATE=20250123

curl https://localhost:$PORT
curl -X POST https://localhost:$PORT/dates/$DATE -d "Add this date"
curl https://localhost:$PORT/
curl https://localhost:$PORT/dates
curl https://localhost:$PORT/dates/$DATE 
curl -X DELETE https://localhost:$PORT/dates/$DATE

# ho wasmtime serve --addr 0.0.0.0:49151
# echo wasmtime serve --addr 0.0.0.0:$PORT --dir ./dates::/dates dates-api/tager/wasm32-wasip2/release/dates_api.wasm
#      wasmtime serve --addr 0.0.0.0:$PORT --dir ./dates::/dates dates-api/tager/wasm32-wasip2/release/dates_api.wasm
