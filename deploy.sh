export file=target/wasm32-unknown-unknown/release/rust_decentralized_company_feedback.wasm

cargo build --target wasm32-unknown-unknown --release
near dev-deploy -f --wasmFile $file

source neardev/dev-account.env
near call $CONTRACT_NAME new '{"owner_id": "'$CONTRACT_NAME'"}' --accountId=$CONTRACT_NAME

rm -rf web/.env
touch web/.env
cp neardev/dev-account.env web/.env