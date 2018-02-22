# cargo-asm

[![crates.io version][crate-shield]][crate] [![Travis build status][travis-shield]][travis] [![Appveyor build status][appveyor-shield]][appveyor] [![License][license-shield]][license]


> A [`cargo`] subcommand that displays the generated assembly generated from Rust source code.

# Install

>cargo install cargo-asm

# Example 

To view the assembly of the function `double_n` in the module `bar` of the crate
[`lib_crate`] annotated with its corresponding Rust code, go to the crate's root
directory and type:

> cargo asm lib_crate::bar::double_n --rust

which outputs:


![screenshot](https://raw.githubusercontent.com/gnzlbg/cargo-asm/images/screenshot.png)


#  Features

* Platform support:

  * OS: Linux, Windows, and MacOSX. 
  * Rust: nightly and stable.
  * Architectures: x86, x86_64, arm, aarch64, powerpc, mips, sparc.

* Displaying:

  * Assembly in Intel or AT&T syntax.
  * Corresponding Rust source code alongside assembly.
  * JSON AST for further processing.

* Querying:

  * functions, for example: `foo`:

  >cargo asm crate::path::to::foo
  
  * inherent method, for example: `foo` of a type `Foo` (that is, `Foo::foo`):

  >cargo asm crate::path::to::Foo::foo
  
  * trait method implementations, for example: `bar` of the trait `Bar` for the type `Foo`:
      
  >cargo asm "<crate::path::to::Foo as crate::path::to::Bar>::bar"

  * generic functions, methods, ...

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

[travis-shield]: https://img.shields.io/travis/gnzlbg/cargo-asm.svg?style=flat-square
[travis]: https://travis-ci.org/gnzlbg/cargo-asm
[appveyor-shield]: https://img.shields.io/appveyor/ci/gnzlbg/cargo-asm.svg?style=flat-square
[appveyor]: https://ci.appveyor.com/project/gnzlbg/cargo-asm/branch/master
[license-shield]: https://img.shields.io/badge/License-MIT%2FApache2.0-green.svg?style=flat-square
[license]: https://github.com/gnzlbg/cargo-asm/blob/master/license.md
[crate-shield]: https://img.shields.io/crates/v/cargo-asm.svg?style=flat-square
[crate]: https://crates.io/crates/cargo-asm

