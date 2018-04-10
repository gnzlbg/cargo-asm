//! Argument parsing utilities.

#![cfg_attr(feature = "cargo-clippy", allow(missing_docs_in_private_items))]

use asm::Style;
use build::Type;
use structopt::StructOpt;

lazy_static! {
    pub static ref opts: ::parking_lot::RwLock<Options> =
        { ::parking_lot::RwLock::new(read()) };
}

/// CLI options of cargo asm.
#[derive(StructOpt, Debug, Clone)]
pub struct AsmOptions {
    #[structopt(
        help = "Path of the function to disassembly, e.g., foo::bar::baz() ."
    )]
    pub path: String,
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
    #[structopt(long = "features", help = "cargo --features")]
    pub features: Vec<String>,
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
        long = "debug-mode", help = "Prints output useful for debugging."
    )]
    pub debug_mode: bool,
    #[structopt(
        long = "manifest-path",
        help = "Runs cargo-asm in a different path.",
        parse(from_os_str)
    )]
    pub manifest_path: Option<::std::path::PathBuf>,
    #[structopt(
        long = "-debug-info",
        help = "Generates assembly with debugging information even if that's not required."
    )]
    pub debug_info: bool,
}

/// CLI options of cargo llvm-ir.
#[derive(StructOpt, Debug, Clone)]
pub struct LlvmIrOptions {
    #[structopt(
        help = "Path of the function to disassembly, e.g., foo::bar::baz() ."
    )]
    pub path: String,
    #[structopt(long = "target", help = "Build for the target triple.")]
    pub TRIPLE: Option<String>,
    #[structopt(long = "features", help = "cargo --features")]
    pub features: Vec<String>,
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
        long = "debug-mode", help = "Prints output useful for debugging."
    )]
    pub debug_mode: bool,
    #[structopt(
        long = "manifest-path",
        help = "Runs cargo-asm in a different path.",
        parse(from_os_str)
    )]
    pub manifest_path: Option<::std::path::PathBuf>,
}

pub trait OptionsExt {
    fn path(&self) -> String;
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
}

impl OptionsExt for ::parking_lot::RwLock<Options> {
    fn path(&self) -> String {
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
    Asm(AsmOptions),
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
    LlvmIr(LlvmIrOptions),
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
