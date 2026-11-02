#!/usr/bin/env bash
# Builds the plugin into a WebAssembly component (wasm32-wasip2 emits a
# component directly; no cargo-component / wasm-tools needed).
#
#   ./build.sh          → dist/plugin.wasm
#
# Follows the conventions of openidea/plugins/build.sh. Any language with
# component-model tooling works; the contract is wit/plugin.wit.
set -euo pipefail
cd "$(dirname "$0")"

rustup target list --installed | grep -qx wasm32-wasip2 || rustup target add wasm32-wasip2

name="$(sed -n 's/^name *= *"\(.*\)"/\1/p' Cargo.toml | head -1)"
echo "building $name"
cargo build --release --target wasm32-wasip2
mkdir -p dist
cp "target/wasm32-wasip2/release/${name//-/_}.wasm" dist/plugin.wasm
echo "  -> dist/plugin.wasm ($(wc -c < dist/plugin.wasm | tr -d ' ') bytes)"
