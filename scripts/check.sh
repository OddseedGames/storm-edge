#!/bin/sh
set -eu
. "$(dirname "$0")/rust-env.sh"
cargo fmt --all --check
cargo test --locked
cargo check --locked --target wasm32-unknown-unknown
