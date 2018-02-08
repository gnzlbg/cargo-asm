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

fn parse_files(files: &Vec<std::path::PathBuf>, path: &String) {
    for f in files {
        asm::parse(f.as_path(), &path);
    }
}

fn main() {
    let opts = options::get();

    println!("path: {}", opts.path);
    let asm_files = build::project(&opts);

    if asm_files.is_empty() {
        panic!("[ERROR] cargo build did not emit any assembly or cargo asm could not find it!")
    }
    println!("files: {:?}", asm_files);
    parse_files(&asm_files, &opts.path);
}
