//! Argument parsing utilities.

use structopt::StructOpt;
use asm::Style;
use build::Type;

#[derive(StructOpt, Debug)]
#[structopt(bin_name = "cargo")]
enum Options_ {
      #[structopt(name = "asm", about = "Shows the assembly generated for a Rust function.")]
      Asm {
    /// Name of the symbol to disassembly.
          #[structopt(help = "Path of the function to disassebly, e.g., foo::bar::baz() .")]
          path: String,
          #[structopt(long = "target", help = "Build for the target triple.")]
          TRIPLE: Option<String>,
          #[structopt(long = "clean", help = "Runs cargo clean before emitting assembly.")]
          clean: bool,
          #[structopt(long = "verbose", help = "Verbose mode.")]
          verbose: bool,
          #[structopt(long = "color", help = "Color output.")]
          color: bool,
          #[structopt(long = "asm-style", help = "Assembly style: intel, at&t.", default_value = "intel")]
          asm_style: Style,
          #[structopt(long = "build-type", help = "Build type: debug, release.", default_value = "release")]
          build_type: Type,
          #[structopt(long = "rust", help = "Print interleaved Rust code.")]
          rust: bool,
          #[structopt(long = "comments", help = "Print assembly comments.")]
          comments: bool,
          #[structopt(long = "directives", help = "Print assembly directives.")]
          directives: bool,
      }
}

#[derive(Debug)]
pub struct Options {
    pub path: String,
    pub TRIPLE: Option<String>,
    pub clean: bool,
    pub verbose: bool,
    pub color: bool,
    pub asm_style: Style,
    pub build_type: Type,
    pub rust: bool,
    pub comments: bool,
    pub directives: bool,
}

pub fn get() -> Options {
    let o = Options_::from_args();
    match o {
        Options_::Asm {
            path,
            TRIPLE,
            clean,
            verbose,
            color,
            asm_style,
            build_type,
            rust,
            comments,
            directives,
        } => Options {
            path,
            TRIPLE,
            clean,
            verbose,
            color,
            asm_style,
            build_type,
            rust,
            comments,
            directives,
        },
    }
}
