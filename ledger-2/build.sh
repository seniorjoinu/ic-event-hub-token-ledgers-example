#!/usr/bin/env bash

SCRIPT=$(readlink -f "$0")
SCRIPTPATH=$(dirname "$SCRIPT")
cd "$SCRIPTPATH" || exit

cargo build --target wasm32-unknown-unknown --release --package ledger-2 && \
 ic-cdk-optimizer ./target/wasm32-unknown-unknown/release/ledger_2.wasm -o ./target/wasm32-unknown-unknown/release/ledger-2-opt.wasm
