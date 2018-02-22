//! Argument parsing utilities.

#![cfg_attr(feature = "cargo-clippy", allow(missing_docs_in_private_items))]

use structopt::StructOpt;
use asm::Style;
use build::Type;

lazy_static! {
    pub static ref opts: ::std::sync::Mutex<Options> = {
        ::std::sync::Mutex::new(read())
    };
}

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

pub trait OptionsExt {
    fn path(&self) -> String;
    fn TRIPLE(&self) -> Option<String>;
    fn clean(&self) -> bool;
    fn no_color(&self) -> bool;
    fn asm_style(&self) -> Style;
    fn build_type(&self) -> Type;
    fn rust(&self) -> bool;
    fn comments(&self) -> bool;
    fn directives(&self) -> bool;
    fn json(&self) -> bool;
    fn debug_mode(&self) -> bool;
    fn project_path(&self) -> Option<String>;
    fn use_colors(&self) -> bool;
    fn print_comments(&self) -> bool;
    fn print_directives(&self) -> bool;
    fn set_rust(&self, value: bool);
}

impl OptionsExt for ::std::sync::Mutex<Options> {
    fn path(&self) -> String {
        self.lock().unwrap().path.clone()
    }
    fn TRIPLE(&self) -> Option<String> {
        self.lock().unwrap().TRIPLE.clone()
    }
    fn clean(&self) -> bool {
        self.lock().unwrap().clean
    }
    fn no_color(&self) -> bool {
        self.lock().unwrap().no_color
    }
    fn asm_style(&self) -> Style {
        self.lock().unwrap().asm_style
    }
    fn build_type(&self) -> Type {
        self.lock().unwrap().build_type
    }
    fn rust(&self) -> bool {
        self.lock().unwrap().rust
    }
    fn comments(&self) -> bool {
        self.lock().unwrap().comments
    }
    fn directives(&self) -> bool {
        self.lock().unwrap().directives
    }
    fn json(&self) -> bool {
        self.lock().unwrap().json
    }
    fn debug_mode(&self) -> bool {
        self.lock().unwrap().debug_mode
    }
    fn project_path(&self) -> Option<String> {
        self.lock().unwrap().project_path.clone()
    }

    fn use_colors(&self) -> bool {
        !self.no_color()
    }
    fn print_comments(&self) -> bool {
        if self.debug_mode() {
            true
        } else {
            self.comments()
        }
    }
    fn print_directives(&self) -> bool {
        if self.debug_mode() {
            true
        } else {
            self.directives()
        }
    }
    fn set_rust(&self, value: bool) {
        self.lock().unwrap().rust = value;
    }
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

fn read() -> Options {
    let o = Options_::from_args();
    match o {
        Options_::Asm(mut o) => {
            // In debug mode we always print the associated Rust code.
            if o.debug_mode {
                o.rust = true;
            }
            o
        }
    }
}
