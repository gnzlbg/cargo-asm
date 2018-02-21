#!/usr/bin/env bash

set -ex

: ${TARGET?"The TARGET environment variable must be set."}

cargo build --release
cargo test --release --target=$TARGET
