//! Wrapper for demangling functions correctly.

use rustc_demangle;

fn has_hash(name: &str) -> bool {
    let mut bytes = name.bytes().rev();
    for _ in 0..16 {
        if !bytes.next().map_or(false, is_ascii_hexdigit) {
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

pub fn demangle(n: &str, target: &str) -> String {
    let n = if target.contains("linux") {
        n.split("@PLT").nth(0).unwrap().to_string()
    } else {
        n.to_string()
    };
    let mut name = rustc_demangle::demangle(&n).to_string();
    if has_hash(&name) {
        let len = name.len() - 19;
        name.truncate(len);
    }
    name
}
