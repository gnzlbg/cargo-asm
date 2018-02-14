//! cargo-asm driver

#![allow(non_snake_case)]
#![feature(match_default_bindings)]
#![cfg_attr(feature = "cargo-clippy",
            allow(missing_docs_in_private_items, option_unwrap_used,
                  result_unwrap_used))]

extern crate edit_distance;
extern crate rustc_demangle;
#[macro_use]
extern crate structopt;
extern crate termcolor;
extern crate walkdir;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod options;
mod process;
mod build;
mod asm;
mod demangle;
mod display;
mod rust;
mod path;

fn parse_files(
    files: &[std::path::PathBuf], mut opts: &mut options::Options
) -> asm::parse::Result {
    use asm::parse::Result;
    let mut function_table = Vec::<String>::new();
    for f in files {
        assert!(f.exists(), "path does not exist: {}", f.display());
        match asm::parse::function(f.as_path(), &mut opts) {
            Result::Found(function, files) => {
                return Result::Found(function, files)
            }
            Result::NotFound(table) => for f in table {
                function_table.push(f);
            },
        }
    }
    Result::NotFound(function_table)
}

#[cfg_attr(feature = "cargo-clippy", allow(print_stdout, use_debug))]
fn main() {
    let mut opts = options::get();
    if opts.verbose {
        println!("Options: {:?}", opts);
        println!("Input path: {}", opts.path);
    }

    let asm_files = build::project(&opts);

    if asm_files.is_empty() {
        display::write_error("cargo build did not emit any assembly or cargo asm could not find it!", &opts);
        ::std::process::exit(1);
    }
    if opts.verbose {
        println!("Assembly files found: {:?}", asm_files);
    }
    match parse_files(&asm_files, &mut opts) {
        asm::parse::Result::Found(function, file_table) => {
            let rust = rust::parse(&function, &file_table, &mut opts);
            if !opts.json {
                display::print(&function, rust.clone(), &opts);
            } else {
                if let Some(s) = display::to_json(&function, &rust) {
                    println!("{}", s);
                }
            }
        }
        asm::parse::Result::NotFound(mut table) => {
            let mut msg = format!("could not find function at path \"{}\" in the generated assembly.\nMaybe you meant one of the following functions?\n", &opts.path);

            let last_path = opts.path.split(':').next_back().unwrap();
            table.sort_by(|a, b| {
                use edit_distance::edit_distance;

                edit_distance(a.split(':').next_back().unwrap(), last_path)
                    .cmp(&edit_distance(
                        b.split(':').next_back().unwrap(),
                        last_path,
                    ))
            });

            for f in table.iter().take(5) {
                msg.push_str(&format!("  {}\n", f));
            }

            msg.push_str("If not maybe the assembly output was not properly built and you might need to do a `--clean` build. If you are trying to print the assembly output of a generic function or method make sure that it is monomorphized into the final binary (otherwise no assembly will be generated).\n"
            );

            display::write_error(&msg, &opts);
            ::std::process::exit(1);
        }
    }
}
