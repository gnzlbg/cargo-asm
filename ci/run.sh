#!/usr/bin/env bash

set -ex

: ${TARGET?"The TARGET environment variable must be set."}

rustup component add rust-src

cargo build --release
cargo test --release --target=$TARGET
