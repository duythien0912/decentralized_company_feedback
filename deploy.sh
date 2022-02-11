export file=target/wasm32-unknown-unknown/release/rust_token.wasm

cargo build --target wasm32-unknown-unknown --release
near dev-deploy -f --wasmFile $file

source neardev/dev-account.env
near call $CONTRACT_NAME new '{"owner_id": "'$CONTRACT_NAME'"}' --accountId=$CONTRACT_NAME