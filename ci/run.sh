#!/usr/bin/env bash

: ${TARGET?"The TARGET environment variable must be set."}

export RUST_TEST_THREADS=1
export RUST_BACKTRACE=1

rustup component add rust-src

cargo build --release
cargo test --release --target=$TARGET

cargo run -- asm --project-path cargo-asm-test/lib_crate lib_crate::sum_array --rust --debug-mode
