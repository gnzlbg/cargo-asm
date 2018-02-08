//! Assembly code parser

use rustc_demangle;

#[derive(Copy, Clone, Debug)]
pub enum Style {
    Intel,
    ATT,
}

impl ::std::str::FromStr for Style {
    type Err = String;
    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        match s {
            "intel" => Ok(Style::Intel),
            "at&t" => Ok(Style::ATT),
            v => Err(format!("\"{}\" is not a valid assembly style. Try \"intel\" or \"at&t\"", v))
        }
    }
}

fn has_hash(name: &str) -> bool {
    let mut bytes = name.bytes().rev();
    for _ in 0..16 {
        if !bytes.next().map(is_ascii_hexdigit).unwrap_or(false) {
            return false;
        }
    }
    bytes.next() == Some(b'h') && bytes.next() == Some(b':')
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

pub fn parse(file: &::std::path::Path, path: &String) {
    use std::io::BufRead;
    let fh = ::std::fs::File::open(file).unwrap();
    let file_buf = ::std::io::BufReader::new(&fh);
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
                    let current_dir = ::std::env::current_dir().unwrap();
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
                let fname = l.split(|c: char| c.is_whitespace())
                    .skip(2)
                    .next()
                    .unwrap();
                println!("fname: {}", fname);
            }
            if l.starts_with(".") && !l.starts_with(".loc") {
                continue;
            }
            if l.ends_with(":") {
                if l.starts_with("Ltmp") | l.starts_with("Lcf")
                    | l.starts_with("Lfunc")
                {
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
