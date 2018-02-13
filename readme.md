# cargo-asm

> **Work in Progress**: this works, sometimes.

`cargo-asm` is a [`cargo`] sub-command that shows you the generated assembly of
a Rust function. For example, if you have a crate called `lib_crate`, you can
view the assembly of the function at `bar::generic_add` by just providing its
whole path (need to qualify it with the crate name for now):

> $ cargo asm lib_crate::bar::generic::add --rust

which outputs:


![screenshot](https://raw.githubusercontent.com/gnzlbg/cargo-asm/images/screenshot.png)


# Quick start

To install it:

>$ cargo install cargo-asm

Quick start: given a crate named `crate`, to search:

  * a function `foo`:
  
  >$ cargo asm crate::path::to::foo
  
  * an inherent method `foo` of a type `Foo` (that is, `Foo::foo`):

  >$ cargo asm crate::path::to::Foo::foo
  
  * an implementation of the trait method `bar` of the trait `Bar` for the type `Foo`:
      
  >$ cargo asm "<crate::path::to::Foo as crate::path::to::Bar>::bar"


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
