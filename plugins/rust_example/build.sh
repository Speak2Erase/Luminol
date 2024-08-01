#! /usr/bin/bash

cargo build -p rust_example
# check if reactor is downloaded
if [ ! -f wasi_snapshot_preview1.reactor.wasm ]; then
  curl -L https://github.com/bytecodealliance/wasmtime/releases/download/v17.0.0/wasi_snapshot_preview1.reactor.wasm -o wasi_snapshot_preview1.reactor.wasm
fi
wasm-tools component new ../../target/wasm32-wasip1/debug/rust_example.wasm --adapt ./wasi_snapshot_preview1.reactor.wasm -o rust_example.wasm

