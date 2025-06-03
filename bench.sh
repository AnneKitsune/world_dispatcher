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
cargo bench --features parallel --all-targets

# WindOS
WINEPREFIX=/home/jojolepro/.wine64 WINEARCH=win64 cargo bench --target x86_64-pc-windows-gnu --features parallel --all-targets

# WASM
# --features parallel not supported on wasm
# benchmarking not supported in wasm
#wasm-pack bench --features parallel

# Android
#cargo dinghy -d android test --features parallel
cargo dinghy -d android bench --features parallel

# InsultingOS
#cargo dinghy -d ios test --features parallel
