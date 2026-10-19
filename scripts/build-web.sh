#!/bin/sh
set -eu
. "$(dirname "$0")/rust-env.sh"
cargo build --locked --target wasm32-unknown-unknown --profile web
if command -v wasm-bindgen >/dev/null 2>&1; then
  task_bindgen=wasm-bindgen
elif [ -x tools/wasm-bindgen-0.2.129-aarch64-apple-darwin/wasm-bindgen ]; then
  task_bindgen=tools/wasm-bindgen-0.2.129-aarch64-apple-darwin/wasm-bindgen
else
  echo 'Install wasm-bindgen-cli matching Cargo.lock: cargo install wasm-bindgen-cli --version 0.2.129 --locked' >&2
  exit 1
fi
task_build_dir=$(mktemp -d "${TMPDIR:-/tmp}/storm-edge-web.XXXXXX")
trap 'rm -rf "$task_build_dir"' EXIT HUP INT TERM
"$task_bindgen" --target web --out-dir "$task_build_dir" --out-name storm-edge target/wasm32-unknown-unknown/web/storm-edge.wasm
python3 scripts/package-web.py "$task_build_dir"
python3 scripts/verify-web.py "$task_build_dir"
rm -rf dist
mv "$task_build_dir" dist
