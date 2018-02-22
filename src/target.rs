use options::*;

/// Returns the target that is being compiled.
pub fn target() -> String {
    if let Some(triple) = opts.TRIPLE() {
        // If the user specified it, we know it:
        triple
    } else {
        // The user did not specify it, so the default target is chosen.
        // This is very brittle, and is just a best effort:
        let r = if cfg!(target_os = "macos") {
            "x86_64-apple-darwin"
        } else if cfg!(target_os = "linux") {
            "x86_64-unknown-linux-gnu"
        } else if cfg!(target_os = "windows") {
            "x86_64-pc-windows-msvc"
        } else {
            error!("unknown target");
            ::std::process::exit(1);
        };
        r.to_string()
    }
}

/// Returns the base path in which the rust std library was built in the build
/// bots. This path needs to be replaced by the path of the rust-src component
/// in the user's computer.
pub fn rust_src_build_path() -> ::std::path::PathBuf {
    let t = target();
    let p = if t.contains("apple") {
        "travis/build/rust-lang/rust/src"
    } else if t.contains("linux") {
        "checkout/src/"
    } else if t.contains("windows") {
        r#"projects\rust\src\"#
    } else {
        error!("unknown target");
        ::std::process::exit(1);
    };
    ::std::path::PathBuf::from(p)
}

/// Returns a path component of the rust-src path (like rust std) that can be
/// used to identify whether a path points to a file within the
/// rust-src component.
///
/// This is a bit brittle since it needs to know even if the rust-src component
/// is not installed, so we cannot query rustup for its path nor walk the
/// sysroot to find it.
pub fn rust_src_path_component() -> ::std::path::PathBuf {
    let t = target();
    let p = if t.contains("windows") {
        r#"lib\rustlib\src\rust\src"#
    } else {
        "lib/rustlib/src/rust/src"
    };

    ::std::path::PathBuf::from(p)
}
