//! Handles how to build the project.

use super::*;

/// Type of the build.
#[derive(Copy, Clone, Debug)]
pub enum Type {
    /// Debug build.
    Debug,
    /// Release build.
    Release,
}

impl ::std::str::FromStr for Type {
    type Err = String;
    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        match s {
            "debug" => Ok(Type::Debug),
            "release" => Ok(Type::Release),
            v => Err(format!("\"{}\" is not a valid build type. Try \"debug\" or \"releaes\"", v))
        }
    }
}

/// Builds the project according to the CLI options and returns a list of
/// assembly files generated.
pub fn project() -> Vec<::std::path::PathBuf> {
    use std::process::Command;
    debug!("Building project...");

    // Read the RUSTFLAGS environment variable
    let rustflags = ::std::env::var_os("RUSTFLAGS")
        .unwrap_or_default()
        .into_string()
        .expect("RUSTFLAGS are not valid UTF-8");

    debug!("RUSTFLAGS={}", rustflags);

    // Compile project generating assembly output:
    let mut cargo_build = Command::new("cargo");
    // TODO: unclear if `cargo build` + `RUSTFLAGS` should be used,
    // or instead one should use `cargo rustc -- --emit asm`
    cargo_build.arg("build");
    if !opts.no_color() {
        cargo_build.arg("--color=always");
        cargo_build.env("LS_COLORS", "rs=0:di=38;5;27:mh=44;38;5;15");
    }
    if let Ok(v) = ::std::env::var("RUSTC") {
        cargo_build.env("RUSTC", v);
    }
    match opts.build_type() {
        Type::Release => cargo_build.arg("--release"),
        Type::Debug => cargo_build.arg("--debug"),
    };
    cargo_build.arg("--verbose");

    if let Some(triple) = opts.TRIPLE() {
        cargo_build.arg(&format!("--target={}", triple));
    }

    let t = target::target();

    match *opts.read().unwrap() {
        ::options::Options::Asm(ref o) => {
            let asm_syntax = match o.asm_style {
                ::asm::Style::Intel if t.contains("86") => {
                    "-C llvm-args=-x86-asm-syntax=intel"
                }
                _ => "",
            };

            cargo_build.env(
                "RUSTFLAGS",
                format!("{} --emit asm -g {}", rustflags, asm_syntax),
            );
        }
        ::options::Options::LlvmIr(ref _o) => {
            cargo_build
                .env("RUSTFLAGS", format!("{} --emit=llvm-ir", rustflags));
        }
    }

    debug!("starting cargo build...");
    let error_msg = "cargo build failed";
    process::exec(&mut cargo_build, error_msg, opts.debug_mode())
        .expect(error_msg);
    debug!("cargo build finished...");

    let target_directory = ::target::directory();

    let ext = match *opts.read().unwrap() {
        ::options::Options::Asm(_) => "s",
        ::options::Options::LlvmIr(_) => "ll",
    };

    // Scan the output directories for files matching the extension:
    let mut output_files = Vec::new();
    for entry in ::walkdir::WalkDir::new(target_directory.clone()) {
        let e = entry.expect(&format!(
            "failed to iterate over the directory: {}",
            target_directory.display()
        ));
        let p = e.path();
        let is_assembly_file =
            p.extension().map_or("", |v| v.to_str().unwrap_or("")) == ext;
        if is_assembly_file {
            let p = p.to_path_buf();
            debug!(
                "found file matching extension \"{}\": {}",
                ext,
                p.display()
            );
            output_files.push(p);
        }
    }

    // Canonicalize, sort the files, remove duplicates, and done:
    if !cfg!(target_os = "windows") {
        // FIXME: On windows canonicalizing makes the path use UNC, but the
        // paths in the assembly emitted by rustc do not use UNC and they are
        // not currently canonicalized.
        for f in &mut output_files {
            let c = f.canonicalize().unwrap();
            debug!("canonicalize path {} into {}", f.display(), c.display());
            *f = c;
        }
    }
    output_files.sort_unstable();
    output_files.dedup();
    output_files
}
