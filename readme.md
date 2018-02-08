# cargo-asm

> **Work in Progress**: this works, for some definition of "works".

`cargo-asm` is a [`cargo`] sub-command that shows you the generated assembly of
a Rust function. For example, if you have a crate called `lib_crate`, you can
view the assembly of the function at `bar::generic_add` by just providing its
whole path (need to qualify it with the crate name for now):

> $ cargo asm lib_crate::bar::generic::add

which prints:

```asm
lib_crate::bar::generic_add:
    push rbp
    mov rbp, rsp
    lea rax, [rdi + rsi]
    pop rbp
    ret
```

Happy hacking. 

# License
This project is licensed under either of

* Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.

# Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.

[`cargo`]: https://crates.io/
