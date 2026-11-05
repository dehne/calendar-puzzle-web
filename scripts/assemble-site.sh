#!/usr/bin/env bash
# SPDX-License-Identifier: MIT
set -euo pipefail
cd -- "$(dirname -- "$0")/.."

mkdir -p dist/layouts dist/pkg
cp index.html app.js style.css about.html LICENSE THIRD_PARTY_NOTICES.md dist/
cp layouts/two-swaps.txt dist/layouts/
cp pkg/*.js pkg/*.wasm dist/pkg/
if [[ -d pkg/snippets ]]; then
  cp -R pkg/snippets dist/pkg/
fi
