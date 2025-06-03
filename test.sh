#!/bin/sh
#
# Requires:
# cargo install wasm-pack
# cargo install wasm-bindgen
# cargo install dinghy (custom fork needed for --features...)
# mingw-w64-gcc
# wine
# rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android
# rustup target add x86_64-pc-windows-gnu
# rustup toolchain install stable-x86_64-pc-windows-gnu

set -e

# Linux
cargo test --all-targets --features parallel

# WindOS
WINEPREFIX=/home/jojolepro/.wine64 WINEARCH=win64 cargo test --target x86_64-pc-windows-gnu --all-targets --features parallel

# WASM
# --features parallel not supported on wasm
#wasm-pack test --node
wasm-pack test --firefox --headless -- --all-targets
# Not supported ;_;
#wasm-pack bench --features parallel

# Android
cargo dinghy -d android test --features parallel

# InsultingOS
#cargo dinghy -d ios test --features parallel
