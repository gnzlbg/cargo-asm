use crate::options::*;
use log::{debug, error};

use serde_derive::Deserialize;
use std::io::prelude::*;

pub struct TargetInfo {
    triple: String,
}

impl Default for TargetInfo {
    fn default() -> Self {
        TargetInfo {
            triple: "none-none-none".to_owned(),
        }
    }
}

impl TargetInfo {
    pub fn new_from_target() -> Self {
        TargetInfo::new_from_triple(target())
    }

    pub fn new_from_triple(triple: String) -> Self {
        TargetInfo { triple }
    }

    pub fn is_intel(&self) -> bool {
        self.triple.contains("86")
    }

    pub fn is_linux(&self) -> bool {
        self.triple.contains("linux")
    }

    pub fn is_windows(&self) -> bool {
        self.triple.contains("windows")
    }

    pub fn is_apple(&self) -> bool {
        self.triple.contains("apple")
    }

    pub fn is_x86(&self) -> bool {
        self.triple.contains("x86")
    }

    pub fn is_i386(&self) -> bool {
        self.triple.contains("i386")
    }

    pub fn is_i586(&self) -> bool {
        self.triple.contains("i586")
    }

    pub fn is_i686(&self) -> bool {
        self.triple.contains("i686")
    }

    pub fn is_aarch64(&self) -> bool {
        self.triple.contains("aarch64")
    }

    pub fn is_arm(&self) -> bool {
        self.triple.contains("arm")
    }

    pub fn is_sparc(&self) -> bool {
        self.triple.contains("sparc")
    }

    pub fn is_power(&self) -> bool {
        self.triple.contains("power")
    }

    pub fn is_mips(&self) -> bool {
        self.triple.contains("mips")
    }
}

/// Returns the target that is being compiled.
fn target() -> String {
    if let Some(triple) = opts.TRIPLE() {
        // If the user specified it, we know it:
        triple
    } else {
        // The user did not specify explicitly specify it, so the let's see
        // whether we can find a TARGET variable in the environment
        if let Ok(target) = ::std::env::var("TARGET") {
            return target;
        }

        // Try reading the build target from the cargo config file
        if let Ok(mut file) =
            std::fs::File::open(std::path::Path::new("./.cargo/config"))
        {
            #[derive(Deserialize, Debug)]
            struct Config {
                build: Option<Build>,
            }

            #[derive(Deserialize, Debug)]
            struct Build {
                target: Option<String>,
            }

            let mut contents = String::new();
            if file.read_to_string(&mut contents).is_ok() {
                if let Ok(config) = toml::from_str::<Config>(&contents) {
                    if let Some(build) = config.build {
                        if let Some(target) = build.target {
                            return target;
                        }
                    }
                }
            }
        }

        // If everything else fails use a best effort guesstimate for the
        // current platform
        if let Some(target) = platforms::Platform::guess_current() {
            return target.target_triple.to_owned();
        }

        // Let's give up
        error!("unknown target");
        ::std::process::exit(1);
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
