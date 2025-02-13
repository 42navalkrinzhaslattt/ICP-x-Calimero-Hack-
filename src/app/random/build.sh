#!/bin/bash
set -e # Exit on error

# Change to the directory of the script
cd "$(dirname $0)"

TARGET="./target"

# Add the wasm target if it's not already installed
rustup target add wasm32-unknown-unknown

# Build the wasm binary
cargo build --target wasm32-unknown-unknown --profile app-release

# Create the res directory if it doesn't exist
mkdir -p res

# Copy the wasm binary to the res directory
cp $TARGET/wasm32-unknown-unknown/app-release/quantum_rng.wasm ./res/

# Optimize the wasm binary to reduce its size
if command -v wasm-opt > /dev/null; then
  wasm-opt -Oz ./res/quantum_rng.wasm -o ./res/quantum_rng.wasm
fi

