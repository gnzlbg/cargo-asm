//! Wrapper for demangling functions correctly.

use crate::target::TargetInfo;

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
    (b'0'..=b'9').contains(&byte) || (b'a'..=b'f').contains(&byte)
}

pub fn demangle(n: &str, target: &TargetInfo) -> String {
    let n = if target.is_linux() {
        n.split("@PLT").next().unwrap().to_string()
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
