#!/bin/sh
set -eu
. "$(dirname "$0")/rust-env.sh"
cargo run --locked -- "$@"
