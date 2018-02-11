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

    let rust = if opts.rust {
        parse_rust_code(&function)
    } else {
        Vec::new()
    };

    display::print(function, rust, &opts);
}

fn parse_rust_code(function: &asm::ast::Function) -> Vec<(usize, String)> {
    use std::io::BufRead;
    use asm::ast::{Statement, Directive};

    if function.file.is_none() {
        panic!("Could not find Rust code for {}!",
               function.id
        );
    }

    if function.loc.is_none() {
        panic!("TODO {}!",
               function.id
        );
    }

    let fh = ::std::fs::File::open(function.file.as_ref().map(|v| v.path.clone()).unwrap()).unwrap();
    let file_buf = ::std::io::BufReader::new(&fh);

    let first_loc = function.loc.as_ref().map(|v| v.offset).unwrap();
    let last_loc = function.statements.iter().filter(|v| {
        match v {
            &&Statement::Directive(Directive::Loc(ref l)) => l.file_index == function.file.as_ref().map(|f| f.index).unwrap(),
            _ => false,
        }
    }).map(|v| {
        match v {
            &Statement::Directive(Directive::Loc(ref l)) => l.offset,
            _ => 0,
        }
    }).max().unwrap();

    let mut r = Vec::new();
    for (line_idx, line) in file_buf.lines().enumerate() {
        if line_idx >= first_loc - 1 && line_idx < last_loc {
            r.push((line_idx + 1, line.unwrap()));
        }
    }
    r
}
