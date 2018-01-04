#!/usr/bin/env bash
set -e

[ -e static/regex.wasm ] && rm static/regex.wasm

cargo +nightly build --target wasm32-unknown-unknown --release

mv "target/wasm32-unknown-unknown/release/regex.wasm" static/regex.wasm