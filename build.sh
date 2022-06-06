#!/bin/sh

set -ex

rustup override set nightly

# must install
# cargo install -f wasm-bindgen-cli

# A couple of steps are necessary to get this build working which makes it slightly
# nonstandard compared to most other builds.
#
# * First, the Rust standard library needs to be recompiled with atomics
#   enabled. to do that we use Cargo's unstable `-Zbuild-std` feature.
#
# * Next we need to compile everything with the `atomics` and `bulk-memory`
#   features enabled, ensuring that LLVM will generate atomic instructions,
#   shared memory, passive segments, etc.

RUSTFLAGS='-C target-feature=+atomics,+bulk-memory,+mutable-globals' \
  cargo build --target wasm32-unknown-unknown --release -Z build-std=std,panic_abort

# Note the usage of `--target no-modules` here which is required for passing
# the memory import to each wasm module.

# cargo run -p wasm-bindgen-cli -- \
#  ../../target/wasm32-unknown-unknown/release/wasm_calculate.wasm \
#  --out-dir . \
#  --target no-modules

wasm-bindgen target/wasm32-unknown-unknown/release/poker_hands.wasm --out-dir . --target no-modules

python3 server.py