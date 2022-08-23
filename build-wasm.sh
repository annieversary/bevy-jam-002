#!/bin/zsh
set -e

cargo build --target wasm32-unknown-unknown --profile wasm-release
wasm-bindgen --out-name luminity \
              --out-dir wasm_out/ \
              --target web \
              target/wasm32-unknown-unknown/wasm-release/luminity.wasm

cp index.html wasm_out
cp -r assets wasm_out

http wasm_out
