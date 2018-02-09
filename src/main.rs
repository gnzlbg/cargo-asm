#![allow(non_snake_case)]

extern crate rustc_demangle;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
extern crate walkdir;

mod options;
mod process;
mod build;
mod asm;
mod demangle;
mod display;

fn parse_files(
    files: &Vec<std::path::PathBuf>, path: &String
) -> Option<asm::ast::Function> {
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
    let function = parse_files(&asm_files, &opts.path)
        .expect(
            &format!("[ERROR]: could not find function at path \"{}\" in the generated assembly",
                     &opts.path));

    display::print(function, &opts);
}
