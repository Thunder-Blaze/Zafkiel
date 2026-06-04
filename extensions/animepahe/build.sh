#!/usr/bin/env bash
# Build the animepahe extension and package it as animepahe.zext

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "Building WASM extension..."
wasm-pack build --target web --out-dir pkg

echo "Packaging animepahe.zext..."
cp pkg/animepahe_ext.js extension.js
cp pkg/animepahe_ext_bg.wasm extension.wasm

zip -FSr animepahe.zext manifest.json extension.js extension.wasm

rm extension.js extension.wasm

SIZE=$(stat --format="%s" animepahe.zext)
echo "Done: animepahe.zext (${SIZE} bytes)"
