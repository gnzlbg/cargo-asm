#![allow(non_snake_case)]

#[macro_use]
extern crate structopt_derive;
extern crate structopt;
extern crate walkdir;
extern crate rustc_demangle;

mod options;
use options::Options;


fn build_project(opt: &Options) -> (Vec<String>, std::time::SystemTime) {
    use std::process::Command;

    let rustflags = std::env::var_os("RUSTFLAGS")
        .unwrap_or_default().into_string()
        .expect("RUSTFLAGS are not valid UTF-8");

    Command::new("cargo").arg("clean").output().expect("failed to run cargo clean");

    let build_start = std::time::SystemTime::now();

    let mut b = Command::new("cargo");
    b.arg("build")
        .arg("--color=always")
        .arg("--release")
        .arg("--verbose")
        .env("LS_COLORS", "rs=0:di=38;5;27:mh=44;38;5;15")
        .env("RUSTFLAGS", format!("{} --emit asm -g -Z asm-comments -C llvm-args=-x86-asm-syntax=intel", rustflags));

    if let Some(ref triple) = opt.TRIPLE {
        b.arg(format!("--target={}", triple));
    }

    println!("{:?}", b);

    let output = b.output().expect("failed to build project");

    if !output.status.success() {
        let stdout = String::from_utf8(output.stdout).expect("Not UTF-8");
        let stderr = String::from_utf8(output.stderr).expect("Not UTF-8");
        eprintln!("\n[ERROR]: The project failed to build!\n");
        if !stderr.is_empty() {
            eprintln!("Build errors:\n\n{}\n\n", stderr);
        }
        if !stdout.is_empty() {
            eprintln!("Build output:\n\n{}\n\n", stdout);
        }
        std::process::exit(1)
    }
    let stdout = String::from_utf8(output.stdout).expect("Not UTF-8");
    let stderr = String::from_utf8(output.stderr).expect("Not UTF-8");
    println!("stdout: {}", stdout);
    println!("stderr: {}", stderr);

    let mut outdirs = Vec::<String>::new();

    for l in stderr.lines() {
        l.trim().split_whitespace().skip_while(|s| s != &"--out-dir").skip(1).take(1)
         .for_each(|v| outdirs.push(v.to_string()));
    }

    (outdirs, build_start)
}

fn find_files(dirs: Vec<String>, build_time: std::time::SystemTime)
  -> Vec<std::path::PathBuf> {
    let mut files = Vec::new();
    for dir in dirs {
        for entry in walkdir::WalkDir::new(dir) {
            let e = entry.unwrap();
            let p = e.path();
            let m = std::fs::metadata(p).unwrap();
            let tm = m.modified().unwrap();
            if tm >= build_time &&
                p.extension().map(|v| v.to_str().unwrap_or("")).unwrap_or("") == "s" {
                files.push(p.to_path_buf());
            }
        }
    }
    files.sort();
    files.dedup();
    files
}

fn parse_files(files: &Vec<std::path::PathBuf>, path: &String) {
    for f in files {
        parse_file(f.as_path(), &path);
    }
}

fn has_hash(name: &str) -> bool {
    let mut bytes = name.bytes().rev();
    for _ in 0..16 {
        if !bytes.next().map(is_ascii_hexdigit).unwrap_or(false) {
            return false;
        }
    }
    bytes.next() == Some(b'h')
        && bytes.next() == Some(b':')
        && bytes.next() == Some(b':')
}

fn is_ascii_hexdigit(byte: u8) -> bool {
    byte >= b'0' && byte <= b'9' || byte >= b'a' && byte <= b'f'
}

fn demangle(n: &str) -> String {
    let mut name = rustc_demangle::demangle(&n).to_string();
    if has_hash(&name) {
        let len = name.len() - 19;
        name.truncate(len);
    }
    name
}

fn parse_file(file: &std::path::Path, path: &String) {
    use std::io::BufRead;
    let fh = std::fs::File::open(file).unwrap();
    let file_buf = std::io::BufReader::new(&fh);
    let mut state = 0;
    for line in file_buf.lines() {
        let l = line.unwrap();
        if state == 0 {
            if l.starts_with("_") {
                let l = l.split(":").take(1).collect::<Vec<_>>()[0];
                println!("{}", l);
                let name = demangle(&l);
                if name == *path {
                    state = 1;
                    let current_dir = std::env::current_dir().unwrap();
                    let rel_path = file.strip_prefix(&current_dir).unwrap();
                    println!("{}({}:{}):", name, rel_path.display(), 0);
                    continue;
                }
            }
        } else if state == 1 {
            let l = l.trim();
            if l == ".cfi_endproc" {
                return;
            }
            if l.starts_with(".file") {
                println!("HERE");
                let fname = l.split(|c: char| c.is_whitespace()).skip(2).next().unwrap();
                println!("fname: {}", fname);
            }
            if l.starts_with(".") && !l.starts_with(".loc") {
                continue;
            }
            if l.ends_with(":") {
                if l.starts_with("Ltmp") | l.starts_with("Lcf") | l.starts_with("Lfunc") {
                    continue;
                }
                println!("  {}", l);
            } else {
                if l.starts_with("call") {
                    let r = l.split_whitespace().skip(1).next().unwrap();
                    println!("    call {}", demangle(r));

                } else {
                    println!("    {}", l.replace("\t", " "));
                }
            }
        }
    }
}

fn main() {
    let opts = options::get();

    println!("path: {}", opts.path);
    let (outdirs, build_time) = build_project(&opts);

    if outdirs.is_empty() {
        eprintln!("sorry! cargo build did not emit any assembly!")
    }

    let files = find_files(outdirs, build_time);
    parse_files(&files, &opts.path);
}
