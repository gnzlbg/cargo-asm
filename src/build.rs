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
    // Read the RUSTFLAGS environment variable
    let rustflags = ::std::env::var_os("RUSTFLAGS")
        .unwrap_or_default()
        .into_string()
        .expect("RUSTFLAGS are not valid UTF-8");

    // Runs `cargo clean` before generating assembly code.
    // TODO: figure out if this is really necessary.
    // UPDATE: seems to be necessary in some cases with incremental
    // compilation.
    if opts.clean() {
        let mut cargo_clean = Command::new("cargo");
        cargo_clean.arg("clean");
        let error_msg = "cargo clean failed";
        process::exec(&mut cargo_clean, error_msg, opts.debug_mode())
            .expect(error_msg);
    }

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
    let asm_syntax = match opts.asm_style() {
        ::asm::Style::Intel => {
            "-C llvm-args=-x86-asm-syntax=intel"
        }
        ::asm::Style::ATT => "",
    };
    if let Some(triple) = opts.TRIPLE() {
        cargo_build.arg(&format!("--target={}", triple));
    }
    cargo_build.env(
        "RUSTFLAGS",
        format!("{} --emit asm -g {}", rustflags, asm_syntax),
    );

    // let build_start = ::std::time::SystemTime::now();
    let error_msg = "cargo build failed";
    let (_stdout, stderr) =
        process::exec(&mut cargo_build, error_msg, opts.debug_mode())
            .expect(error_msg);

    // Find output directories:
    // TODO: is this really necessary? Assembly output "should" be in
    // ${working_dir}/targets/{release,debug}/build/*.s
    let mut output_directories = Vec::<::std::path::PathBuf>::new();
    for l in stderr.lines() {
        // This goes through the Running "rustc ... " invokations printed to
        // stderr looking for --out-dir and collects the directories into a
        // Vec:
        l.trim()
            .split_whitespace()
            .skip_while(|&s| s != "--out-dir")
            .skip(1)
            .take(1)
            .for_each(|v| {
                let path = ::std::path::PathBuf::from(v.to_string());
                debug!("parsed --out-dir: {}", path.display());
                output_directories.push(path)
            });
    }

    // Append the typical output directory:
    {
        let build_dir = match opts.build_type() {
            Type::Release => "release",
            Type::Debug => "debug",
        };

        let mut typical_path = ::std::env::current_dir()
            .expect("cannot read the current working directory");
        typical_path.push("target");
        typical_path.push(build_dir);
        typical_path.push("deps");
        debug!("typical path: {}", typical_path.display());

        output_directories.push(typical_path);
    }

    // Scan the output directories for assembly files ".s" that have been
    // generated after the build start.
    let mut output_files = Vec::new();
    for dir in output_directories {
        debug!("scanning directory for assembly files: {}", dir.display());
        for entry in ::walkdir::WalkDir::new(dir.clone()) {
            let e = entry.expect(&format!(
                "failed to iterate over the directory: {}",
                dir.display()
            ));
            let p = e.path();
            let is_assembly_file =
                p.extension().map_or("", |v| v.to_str().unwrap_or("")) == "s";
            if is_assembly_file {
                let p = p.to_path_buf();
                debug!("found assembly file: {}", p.display());
                output_files.push(p);
            }
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
