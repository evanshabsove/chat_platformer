
set -e
cargo build --target wasm32-unknown-unknown --release
wasm-server-runner target/wasm32-unknown-unknown/release/chat-platformer.wasm
