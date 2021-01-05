#!/bin/bash

set -euo pipefail

TARGET=wasm32-unknown-unknown
BINARY=target/$TARGET/release/visard.wasm

cargo build --target $TARGET --release
wasm-strip $BINARY
mkdir -p www
wasm-opt -o www/visard.wasm -Oz $BINARY
ls -lh www/visard.wasm
