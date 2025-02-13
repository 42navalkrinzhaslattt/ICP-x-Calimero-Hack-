#!/bin/bash
set -e

cd "$(dirname $0)"

TARGET="${CARGO_TARGET_DIR:-../../target}"

rustup target add wasm32-unknown-unknown

cargo build --target wasm32-unknown-unknown --profile app-release

mkdir -p res

cp $TARGET/wasm32-unknown-unknown/app-release/kv_store.wasm ./res/

# You can optionally choose to install and use wasm-opt, for an additional optimization step in the build script. This step is not required but can help reduce the size of the generated Wasm file:
# if command -v wasm-opt > /dev/null; then
#   wasm-opt -Oz ./res/kv_store.wasm -o ./res/kv_store.wasm
# fi