#![allow(non_snake_case)]

extern crate rustc_demangle;
#[macro_use]
extern crate structopt;
extern crate walkdir;

mod options;
mod process;
mod build;
mod asm;
mod demangle;
mod display;
mod rust;

fn parse_files(
    files: &Vec<std::path::PathBuf>, path: &String
) -> Option<
    (
        asm::ast::Function,
        ::std::collections::HashMap<usize, asm::ast::File>,
    ),
> {
    for f in files {
        if let Some(f) = asm::parse::function(f.as_path(), &path) {
            return Some(f);
        }
    }
    None
}

fn main() {
    let opts = options::get();
    if opts.verbose {
        println!("Options: {:?}", opts);
        println!("Input path: {}", opts.path);
    }

    let asm_files = build::project(&opts);

    if asm_files.is_empty() {
        panic!("[ERROR] cargo build did not emit any assembly or cargo asm could not find it!")
    }
    if opts.verbose {
        println!("Assembly files found: {:?}", asm_files);
    }
    let (function, file_table) = parse_files(&asm_files, &opts.path)
        .expect(
            &format!("[ERROR]: could not find function at path \"{}\" in the generated assembly",
                     &opts.path));

    if opts.rust {
        let rust = rust::parse(&function, file_table);
        display::print_rust(function, rust, &opts);
    } else {
        display::print_asm(function, &opts);
    };
}
