//! Argument parsing utilities.

#![allow(clippy::missing_docs_in_private_items)]

use crate::asm::Style;
use crate::build::Type;
use lazy_static::lazy_static;
use structopt::StructOpt;

lazy_static! {
    pub static ref opts: ::parking_lot::RwLock<Options> =
        ::parking_lot::RwLock::new(read());
}

/// CLI options of cargo asm.
#[derive(StructOpt, Debug, Clone)]
// FIXME: https://github.com/rust-lang-nursery/rust-clippy/issues/2651
#[allow(clippy::similar_names)]
pub struct Asm {
    #[structopt(
        help = "Path of the function to disassembly, e.g., foo::bar::baz(). If missing then all function names will be printed."
    )]
    pub path: Option<String>,
    #[structopt(long = "target", help = "Build for the target triple.")]
    pub TRIPLE: Option<String>,
    #[structopt(long = "no-color", help = "Disable colored output.")]
    pub no_color: bool,
    #[structopt(
        long = "asm-style",
        help = "Assembly style: intel, att.",
        default_value = "intel"
    )]
    pub asm_style: Style,
    #[structopt(
        long = "build-type",
        help = "Build type: debug, release.",
        default_value = "release"
    )]
    pub build_type: Type,
    #[structopt(long = "features", help = "cargo build --features=…")]
    pub features: Vec<String>,
    #[structopt(long = "example", help = "cargo build --example=…")]
    pub example: Option<String>,
    #[structopt(long = "rust", help = "Print interleaved Rust code.")]
    pub rust: bool,
    #[structopt(long = "comments", help = "Print assembly comments.")]
    pub comments: bool,
    #[structopt(long = "directives", help = "Print assembly directives.")]
    pub directives: bool,
    #[structopt(
        long = "json",
        help = "Serialize asm AST to json (ignores most other options)."
    )]
    pub json: bool,
    #[structopt(
        long = "debug-mode",
        help = "Prints output useful for debugging."
    )]
    pub debug_mode: bool,
    #[structopt(
        long = "manifest-path",
        help = "Runs cargo-asm in a different path.",
        parse(from_os_str)
    )]
    pub manifest_path: Option<::std::path::PathBuf>,
    #[structopt(
        long = "debug-info",
        help = "Generates assembly with debugging information even if that's not required."
    )]
    pub debug_info: bool,
    #[structopt(long = "lib", help = "Builds only the lib target.")]
    pub lib: bool,
    #[structopt(
        long = "no-default-features",
        help = "Disables all cargo features when building the project."
    )]
    pub no_default_features: bool,
}

/// CLI options of cargo llvm-ir.
#[derive(StructOpt, Debug, Clone)]
// FIXME: https://github.com/rust-lang-nursery/rust-clippy/issues/2651
#[allow(clippy::similar_names)]
pub struct LlvmIr {
    #[structopt(
        help = "Path of the function to disassembly, e.g., foo::bar::baz(). If missing then all function names will be printed."
    )]
    pub path: Option<String>,
    #[structopt(long = "target", help = "Build for the target triple.")]
    pub TRIPLE: Option<String>,
    #[structopt(long = "features", help = "cargo build --features=…")]
    pub features: Vec<String>,
    #[structopt(long = "example", help = "cargo build --example=…")]
    pub example: Option<String>,
    #[structopt(long = "no-color", help = "Disable colored output.")]
    pub no_color: bool,
    #[structopt(
        long = "build-type",
        help = "Build type: debug, release.",
        default_value = "release"
    )]
    pub build_type: Type,
    #[structopt(long = "rust", help = "Print interleaved Rust code.")]
    pub rust: bool,
    #[structopt(
        long = "debug-mode",
        help = "Prints output useful for debugging."
    )]
    pub debug_mode: bool,
    #[structopt(
        long = "manifest-path",
        help = "Runs cargo-asm in a different path.",
        parse(from_os_str)
    )]
    pub manifest_path: Option<::std::path::PathBuf>,
    #[structopt(long = "lib", help = "Builds only the lib target.")]
    pub lib: bool,
    #[structopt(
        long = "no-default-features",
        help = "Disables all cargo features when building the project."
    )]
    pub no_default_features: bool,
}

pub trait Ext {
    fn path(&self) -> Option<String>;
    fn TRIPLE(&self) -> Option<String>;
    fn no_color(&self) -> bool;
    fn asm_style(&self) -> Option<Style>;
    fn build_type(&self) -> Type;
    fn rust(&self) -> bool;
    fn comments(&self) -> Option<bool>;
    fn directives(&self) -> Option<bool>;
    fn json(&self) -> bool;
    fn debug_mode(&self) -> bool;
    fn manifest_path(&self) -> Option<::std::path::PathBuf>;
    fn use_colors(&self) -> bool;
    fn print_comments(&self) -> bool;
    fn print_directives(&self) -> bool;
    fn set_rust(&self, value: bool);
    fn features(&self) -> Vec<String>;
    fn example(&self) -> Option<String>;
    fn lib(&self) -> bool;
    fn no_default_features(&self) -> bool;
}

impl Ext for ::parking_lot::RwLock<Options> {
    fn path(&self) -> Option<String> {
        match *self.read() {
            Options::Asm(ref o) => o.path.clone(),
            Options::LlvmIr(ref o) => o.path.clone(),
        }
    }
    fn TRIPLE(&self) -> Option<String> {
        match *self.read() {
            Options::Asm(ref o) => o.TRIPLE.clone(),
            Options::LlvmIr(ref o) => o.TRIPLE.clone(),
        }
    }
    fn no_color(&self) -> bool {
        match *self.read() {
            Options::Asm(ref o) => o.no_color,
            Options::LlvmIr(ref o) => o.no_color,
        }
    }
    fn asm_style(&self) -> Option<Style> {
        match *self.read() {
            Options::Asm(ref o) => Some(o.asm_style),
            Options::LlvmIr(_) => None,
        }
    }
    fn build_type(&self) -> Type {
        match *self.read() {
            Options::Asm(ref o) => o.build_type,
            Options::LlvmIr(ref o) => o.build_type,
        }
    }
    fn rust(&self) -> bool {
        match *self.read() {
            Options::Asm(ref o) => o.rust,
            Options::LlvmIr(ref o) => o.rust,
        }
    }
    fn comments(&self) -> Option<bool> {
        match *self.read() {
            Options::Asm(ref o) => Some(o.comments),
            Options::LlvmIr(_) => None,
        }
    }
    fn directives(&self) -> Option<bool> {
        match *self.read() {
            Options::Asm(ref o) => Some(o.directives),
            Options::LlvmIr(_) => None,
        }
    }
    fn json(&self) -> bool {
        match *self.read() {
            Options::Asm(ref o) => o.json,
            Options::LlvmIr(ref _o) => false,
        }
    }
    fn debug_mode(&self) -> bool {
        match *self.read() {
            Options::Asm(ref o) => o.debug_mode,
            Options::LlvmIr(ref o) => o.debug_mode,
        }
    }
    fn manifest_path(&self) -> Option<::std::path::PathBuf> {
        match *self.read() {
            Options::Asm(ref o) => o.manifest_path.clone(),
            Options::LlvmIr(ref o) => o.manifest_path.clone(),
        }
    }

    fn use_colors(&self) -> bool {
        !self.no_color()
    }
    fn print_comments(&self) -> bool {
        if self.debug_mode() {
            true
        } else {
            self.comments().unwrap()
        }
    }
    fn print_directives(&self) -> bool {
        if self.debug_mode() {
            true
        } else {
            self.directives().unwrap()
        }
    }
    fn set_rust(&self, value: bool) {
        match *self.write() {
            Options::Asm(ref mut o) => o.rust = value,
            Options::LlvmIr(ref mut o) => o.rust = value,
        }
    }
    fn features(&self) -> Vec<String> {
        match *self.read() {
            Options::Asm(ref o) => o.features.clone(),
            Options::LlvmIr(ref o) => o.features.clone(),
        }
    }
    fn example(&self) -> Option<String> {
        match *self.read() {
            Options::Asm(ref o) => o.example.clone(),
            Options::LlvmIr(ref o) => o.example.clone(),
        }
    }
    fn lib(&self) -> bool {
        match *self.read() {
            Options::Asm(ref o) => o.lib,
            Options::LlvmIr(ref o) => o.lib,
        }
    }
    fn no_default_features(&self) -> bool {
        match *self.read() {
            Options::Asm(ref o) => o.no_default_features,
            Options::LlvmIr(ref o) => o.no_default_features,
        }
    }
}

#[derive(StructOpt, Debug, Clone)]
#[structopt(bin_name = "cargo")]
pub enum Options {
    #[structopt(
        name = "asm",
        about = "\
Shows the assembly generated for a Rust function.

Quick start: given a crate named \"crate\", to search:
  * a function \"foo\":
      cargo asm crate::path::to::foo,
  * an inherent method \"foo\" of a type \"Foo\":
      cargo asm crate::path::to::Foo::foo,
  * an implementation of the trait method \"bar\" of the trait \"Bar\" for the type \"Foo\":
      cargo asm \"<crate::path::to::Foo as crate::path::to::Bar>::bar\"
"
    )]
    Asm(Asm),
    #[structopt(
        name = "llvm-ir",
        about = "\
        Shows the llvm-ir generated for a Rust function.

Quick start: given a crate named \"crate\", to search:
  * a function \"foo\":
      cargo asm crate::path::to::foo,
  * an inherent method \"foo\" of a type \"Foo\":
      cargo asm crate::path::to::Foo::foo,
  * an implementation of the trait method \"bar\" of the trait \"Bar\" for the type \"Foo\":
      cargo asm \"<crate::path::to::Foo as crate::path::to::Bar>::bar\"
"
    )]
    LlvmIr(LlvmIr),
}

fn read() -> Options {
    let mut o = Options::from_args();
    match o {
        Options::Asm(ref mut o) => {
            // In debug mode we always print the associated Rust code.
            if o.debug_mode {
                o.rust = true;
            }
        }
        Options::LlvmIr(ref mut o) => {
            // In debug mode we always print the associated Rust code.
            if o.debug_mode {
                o.rust = true;
            }
        }
    };
    o
}
