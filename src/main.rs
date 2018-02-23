//! cargo-asm driver
#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(feature = "cargo-clippy",
            allow(missing_docs_in_private_items, option_unwrap_used,
                  result_unwrap_used))]

extern crate edit_distance;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate rustc_demangle;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate structopt;
extern crate termcolor;
extern crate walkdir;

mod options;
mod process;
mod build;
mod asm;
mod demangle;
mod display;
mod rust;
mod path;
mod logger;
mod target;

use options::*;

fn parse_files(files: &[std::path::PathBuf]) -> asm::parse::Result {
    use asm::parse::Result;
    if opts.debug_mode() {
        // In debug mode dump all the raw assembly that we could find.
        for f in files {
            debug!("raw file dump {}:", f.display());
            use std::io::BufRead;

            let fh = ::std::fs::File::open(f).unwrap();
            let file_buf = ::std::io::BufReader::new(&fh);
            for l in file_buf.lines() {
                debug!("{}", l.unwrap());
            }
        }
    }
    let mut function_table = Vec::<String>::new();
    for f in files {
        assert!(f.exists(), "path does not exist: {}", f.display());
        match asm::parse::function(f.as_path()) {
            Result::Found(function, files) => {
                return Result::Found(function, files)
            }
            Result::NotFound(table) => for f in table {
                function_table.push(f);
            },
        }
    }
    function_table.sort();
    function_table.dedup();
    Result::NotFound(function_table)
}

#[cfg_attr(feature = "cargo-clippy", allow(print_stdout, use_debug))]
fn main() {
    // Initialize logger and options:
    if let Err(err) = logger::Logger::init() {
        eprintln!("failed to initialize logger: {}", err);
        ::std::process::exit(1);
    }

    if opts.debug_mode() {
        log::set_max_level(log::LevelFilter::Debug);
    } else {
        log::set_max_level(log::LevelFilter::Error);
    }

    debug!("Options: {:?}", *opts);

    // Executing cargo asm into a different path via --project-path=... is done
    // by changing the current working directory.
    if let Some(ref new_path) = opts.project_path() {
        assert!(
            ::std::env::set_current_dir(&new_path).is_ok(),
            "failed to change the working path to {}",
            new_path.display()
        );
    }

    // Builds the project and returns a list of all relevant assembly files:
    let asm_files = build::project();

    if asm_files.is_empty() {
        display::write_error("cargo build did not emit any assembly or cargo asm could not find it!");
        ::std::process::exit(1);
    }

    debug!("Found following assembly files:");
    for f in &asm_files {
        debug!("  {}", f.display());
    }

    // Parse the files
    match parse_files(&asm_files) {
        asm::parse::Result::Found(mut function, file_table) => {
            // If we found the assembly for the path, we parse the assembly:
            let rust = rust::parse(&function, &file_table);

            if opts.json() || opts.debug_mode() {
                if let Some(s) = display::to_json(&function, &rust) {
                    println!("{}", s);
                } else {
                    error!("failed to emit json output");
                }
            }

            if !opts.json() {
                display::print(&mut function, rust.clone());
            }
        }
        asm::parse::Result::NotFound(mut table) => {
            use edit_distance::edit_distance;
            let mut msg = format!("could not find function at path \"{}\" in the generated assembly.\nMaybe you meant one of the following functions?\n\n", &opts.path());

            let last_path = opts.path();
            let last_path = last_path.split(':').next_back().unwrap();
            table.sort_by(|a, b| {
                edit_distance(a.split(':').next_back().unwrap(), last_path)
                    .cmp(&edit_distance(
                        b.split(':').next_back().unwrap(),
                        last_path,
                    ))
            });

            for f in table.iter() {
                if edit_distance(f.split(':').next_back().unwrap(), last_path)
                    > 4
                {
                    break;
                }
                msg.push_str(&format!("  {}\n", f));
            }

            msg.push_str("\nOtherwise make sure that the function is present in the final binary (e.g. if it's a generic function, make sure that it is actually monomorphized) or try to do a --clean build (sometimes changes are not picked up).\n"
            );

            display::write_error(&msg);
            ::std::process::exit(1);
        }
    }
}
