#!/usr/bin/env bash

set -ex

: ${TARGET?"The TARGET environment variable must be set."}

cargo install --force
cd cargo-asm-test/lib_crate

cargo asm lib_crate::bar::double_n --target=$TARGET
cargo asm lib_crate::bar::double_n --rust --target=$TARGET
cargo asm lib_crate::bar::double_n --directives --comments --rust --verbose --target=$TARGET
cargo asm lib_crate::bar::double_n --json --target=$TARGET

cargo asm lib_crate::bar::generic_add --target=$TARGET
cargo asm lib_crate::bar::generic_add --rust --target=$TARGET
cargo asm lib_crate::bar::generic_add --directives --comments --rust --verbose --target=$TARGET
cargo asm lib_crate::bar::generic_add --json --target=$TARGET

cargo asm lib_crate::sum_array --target=$TARGET
cargo asm lib_crate::sum_array --rust --target=$TARGET
cargo asm lib_crate::sum_array --directives --comments --rust --verbose --target=$TARGET
cargo asm lib_crate::sum_array --json --target=$TARGET

cargo asm lib_crate::bar::double_n --target=$TARGET --raw
