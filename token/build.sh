#!/usr/bin/env bash

SCRIPT=$(readlink -f "$0")
SCRIPTPATH=$(dirname "$SCRIPT")
cd "$SCRIPTPATH" || exit

cargo build --target wasm32-unknown-unknown --release --package token && \
 ic-cdk-optimizer ./target/wasm32-unknown-unknown/release/token.wasm -o ./target/wasm32-unknown-unknown/release/token-opt.wasm
