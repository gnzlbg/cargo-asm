use crate::options::*;
use log::{debug, error};

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

pub fn directory<P: AsRef<::std::path::Path>>(
    sub_path: P,
) -> ::std::path::PathBuf {
    debug!("obtaining the target directory...");
    // Run cargo metadata to get the target directory
    let mut target_directory = {
        let mut cargo = ::std::process::Command::new("cargo");
        cargo.arg("metadata");
        cargo.arg("--format-version");
        cargo.arg("1");
        let error_msg = "cargo metadata failed";
        let (stdout, _stderr) =
            crate::process::exec(&mut cargo, error_msg, opts.debug_mode())
                .expect(error_msg);

        // Parse the metadata format
        let v: ::serde_json::Value = ::serde_json::from_str(&stdout)
            .expect("failed to parse cargo metadata's output as json");
        ::std::path::PathBuf::from(v["target_directory"].as_str().expect("could not find key \"target_directory\" in the output of `cargo metadata`"))
    };

    // Generate build type path component:
    let build_type = match opts.build_type() {
        crate::build::Type::Release => "release",
        crate::build::Type::Debug => "debug",
    };

    let t = target();

    // Is the target the "native" target?
    let is_native = match t.as_str() {
        "x86_64-apple-darwin" if cfg!(target_os = "macos") => true,
        "x86_64-unknown-linux-gnu" if cfg!(target_os = "linux") => true,
        "x86_64-pc-windows-msvc" if cfg!(target_os = "windows") => true,
        _ => false,
    };

    if !is_native {
        target_directory.push(t);
    }
    target_directory.push(build_type);
    target_directory.push(sub_path);

    if !target_directory.exists() {
        error!(
            "The target directory for the build does not exist: {}",
            target_directory.display()
        )
    }

    debug!("the target directory is {}", target_directory.display());

    target_directory
}
