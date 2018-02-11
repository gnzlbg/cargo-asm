//! Argument parsing utilities.

use structopt::StructOpt;
use asm::Style;
use build::Type;

#[derive(StructOpt, Debug)]
#[structopt(name = "asm",
            about = "Shows the assembly generated for a Rust function.")]
pub struct Options {
    /// Name of the symbol to disassembly.
          #[structopt(help = "Path of the function to disassebly, e.g., foo::bar::baz() .")]
          pub path: String,
          #[structopt(long = "target", help = "Build for the target triple.")]
          pub TRIPLE: Option<String>,
          #[structopt(long = "clean", help = "Runs cargo clean before emitting assembly.")]
          pub clean: bool,
          #[structopt(long = "verbose", help = "Verbose mode.")]
          pub verbose: bool,
          #[structopt(long = "color", help = "Color output.")]
          pub color: bool,
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
      }

#[derive(StructOpt, Debug)]
#[structopt(bin_name = "cargo")]
enum Options_ {
    Asm(Options),
}

pub fn get() -> Options {
    let o = Options_::from_args();
    match o {
        Options_::Asm(a) => a,
    }
}
