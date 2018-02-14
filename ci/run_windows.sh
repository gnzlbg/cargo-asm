#!/usr/bin/env bash

set -ex

cargo install cargo-asm
cd cargo-asm-test/lib_crate

cargo asm lib_crate::bar::double_n
cargo asm lib_crate::bar::double_n --rust
cargo asm lib_crate::bar::double_n --json

cargo asm lib_crate::bar::generic_add
cargo asm lib_crate::bar::generic_add --rust
cargo asm lib_crate::bar::generic_add --json

cargo asm lib_crate::sum_array
cargo asm lib_crate::sum_array --rust
cargo asm lib_crate::sum_array --json
