//! Argument parsing utilities.

use structopt::StructOpt;

pub struct Options {
    pub path: String,
    pub TRIPLE: Option<String>,
}

#[derive(StructOpt, Debug)]
#[structopt(bin_name = "cargo")]
  enum Options_ {
      #[structopt(name = "asm", about = "Shows the assembly generated for a Rust function.")]
      Asm {
          /// Name of the symbol to disassembly.
          #[structopt(help = "Path of the function to disassebly, e.g., foo::bar::baz()")]
          path: String,
          #[structopt(long = "target", help = "Build for the target triple")]
          TRIPLE: Option<String>,
      }
  }

  pub fn get() -> Options {
      let o = Options_::from_args();
      match o {
          Options_::Asm { path, TRIPLE } => Options { path, TRIPLE }
      }
  }
