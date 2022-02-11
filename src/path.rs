//! Path utilities

/// Does the path contain the sub path?
pub fn contains(
    path: &::std::path::Path,
    sub_path: &::std::path::Path,
) -> bool {
    let mut sub_path_iter = sub_path.components();
    let mut next_sub_path = sub_path_iter.next();

    // If the sub-path is empty, we are done:
    if next_sub_path.is_none() {
        return true;
    }

    let mut matching = false;
    for c in path.components() {
        if let Some(next_sp) = next_sub_path {
            if let ::std::path::Component::RootDir = next_sp {
                if !cfg!(target_os = "windows") {
                    next_sub_path = sub_path_iter.next();
                }
            }
        }
        let next_sub_path_val = next_sub_path.unwrap();
        if c == next_sub_path_val {
            matching = true;
            next_sub_path = sub_path_iter.next();
            // If we exhaust the sub-path, we are done:
            if next_sub_path.is_none() {
                return true;
            }
        } else if matching {
            // We have found at least one match, but this component does
            // not match, so we restart the search:
            sub_path_iter = sub_path.components();
            next_sub_path = sub_path_iter.next();
            matching = false;
        }
    }
    false
}

/// Path after sub-path:
pub fn after(
    path: &::std::path::Path,
    sub_path: &::std::path::Path,
) -> ::std::path::PathBuf {
    assert!(contains(path, sub_path));

    let mut buf = ::std::path::PathBuf::new();

    let mut sub_path_iter = sub_path.components();
    let mut next_sub_path = sub_path_iter.next();

    let mut appending = next_sub_path.is_none();
    let mut matching = false;
    for c in path.components() {
        if appending {
            buf.push(c.as_os_str());
        } else {
            if let Some(next_sp) = next_sub_path {
                if let ::std::path::Component::RootDir = next_sp {
                    if !cfg!(target_os = "windows") {
                        next_sub_path = sub_path_iter.next();
                    }
                }
            }

            let next_sub_path_val = next_sub_path.unwrap();
            if c == next_sub_path_val {
                matching = true;
                next_sub_path = sub_path_iter.next();
                // If we exhaust the sub-path, we are done:
                if next_sub_path.is_none() {
                    appending = true;
                }
            } else if matching {
                // We have found at least one match, but this component does
                // not match, so we restart the search:
                sub_path_iter = sub_path.components();
                next_sub_path = sub_path_iter.next();
                matching = false;
            }
        }
    }
    buf
}

/// Appends the `tail` to the path:
pub fn push(path: &mut ::std::path::PathBuf, tail: &::std::path::Path) {
    assert!(!tail.is_absolute());
    path.push(tail);
}

#[cfg(test)]
mod tests {
    #[test]
    fn contains() {
        {
            let sub_path =
                ::std::path::PathBuf::from("lib/rustlib/src/rust/src/");

            let macosx_path = ::std::path::PathBuf::from("/Users/foo/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/src/liballoc");
            let macosx_path_typo = ::std::path::PathBuf::from("/Users/foo/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib2/src/rust/src/liballoc");

            assert!(super::contains(&macosx_path, &sub_path));
            assert!(!super::contains(&macosx_path_typo, &sub_path));
            assert_eq!(
                super::after(&macosx_path, &sub_path),
                ::std::path::PathBuf::from("liballoc")
            );
        }
        if cfg!(target_os = "windows") {
            let sub_path = ::std::path::PathBuf::from(
                r#"C:\projects\cargo-asm\cargo-asm-test\lib_crate"#,
            );
            let windows_path = ::std::path::PathBuf::from(
                r#"C:\projects\cargo-asm\cargo-asm-test\lib_crate\src\lib.rs"#,
            );
            assert!(super::contains(&windows_path, &sub_path));

            assert_eq!(
                super::after(&windows_path, &sub_path),
                ::std::path::PathBuf::from(r#"src\lib.rs"#)
            );
        }
    }
}
