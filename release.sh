#!/bin/sh

mkdir -p target/release/{macos,linux}

cargo build --release
mv target/release/wait-for-rust target/release/macos/wait-for-rust
docker run -v $(pwd):/app -w /app rustlang/rust:nightly cargo build --release
mv target/release/wait-for-rust target/release/linux/wait-for-rust
