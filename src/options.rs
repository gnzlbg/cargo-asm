//! Argument parsing utilities.

#![cfg_attr(feature = "cargo-clippy", allow(missing_docs_in_private_items))]

use structopt::StructOpt;
use asm::Style;
use build::Type;

/// CLI options of cargo-asm.
#[derive(StructOpt, Debug)]
pub struct Options {
    /// Name of the symbol to disassembly.
    #[structopt(help = "Path of the function to disassebly, e.g., foo::bar::baz() .")]
    pub path: String,
    #[structopt(long = "target", help = "Build for the target triple.")]
    pub TRIPLE: Option<String>,
    #[structopt(long = "clean", help = "Runs cargo clean before emitting assembly.")]
    pub clean: bool,
    #[structopt(long = "no-color", help = "Disable colored output.")]
    pub no_color: bool,
    #[structopt(long = "asm-style", help = "Assembly style: intel, at&t.", default_value = "intel")]
    pub asm_style: Style,
    #[structopt(long = "build-type", help = "Build type: debug, release.", default_value = "release")]
    pub build_type: Type,
    #[structopt(long = "rust", help = "Print interleaved Rust code.")]
    pub rust: bool,
    #[structopt(long = "comments", help = "Print assembly comments.")]
    pub comments: bool,
    #[structopt(long = "directives", help = "Print assembly directives.")]
    pub directives: bool,
    #[structopt(long = "json", help = "Serialize asm AST to json (ignores most other options).")]
    pub json: bool,
    #[structopt(long = "debug-mode", help = "Prints output useful for debugging.")]
    pub debug_mode: bool,
    #[structopt(long = "project-path", help = "Runs cargo-asm in a different path.")]
    pub project_path: Option<String>,
}

#[derive(StructOpt, Debug)]
#[structopt(bin_name = "cargo")]
enum Options_ {
    #[structopt(name = "asm",
                about = "\
Shows the assembly generated for a Rust function.

Quick start: given a crate named \"crate\", to search:
  * a function \"foo\":
      cargo asm crate::path::to::foo,
  * an inherent method \"foo\" of a type \"Foo\":
      cargo asm crate::path::to::Foo::foo,
  * an implementation of the trait method \"bar\" of the trait \"Bar\" for the type \"Foo\":
      cargo asm \"<crate::path::to::Foo as crate::path::to::Bar>::bar\"
")]
    Asm(Options),
}

pub fn get() -> Options {
    let o = Options_::from_args();
    match o {
        Options_::Asm(a) => a,
    }
}
