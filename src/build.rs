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
    if let Type::Release = opts.build_type() {
        cargo_build.arg("--release");
    }
    cargo_build.arg("--verbose");

    if !opts.features().is_empty() {
        cargo_build.arg(&format!("--features={}", opts.features().join(",")));
    }

    if let Some(example) = opts.example() {
        cargo_build.arg(&format!("--example={}", example));
    }

    if opts.no_default_features() {
        cargo_build.arg("--no-default-features");
    }

    if opts.lib() {
        cargo_build.arg("--lib");
    }

    if let Some(triple) = opts.TRIPLE() {
        cargo_build.arg(&format!("--target={}", triple));
    }

    let t = target::target();

    match *opts.read() {
        crate::options::Options::Asm(ref o) => {
            let asm_syntax = match o.asm_style {
                crate::asm::Style::Intel if t.contains("86") => {
                    "-C llvm-args=-x86-asm-syntax=intel"
                }
                _ => "",
            };

            let debug_info = if o.rust || o.debug_info {
                "-C debuginfo=2"
            } else {
                ""
            };

            cargo_build.env(
                "RUSTFLAGS",
                format!(
                    "{} --emit asm {} {}",
                    rustflags, debug_info, asm_syntax
                ),
            );
        }
        crate::options::Options::LlvmIr(ref _o) => {
            // TODO: the debug info really clutters the llvm-ir (-g)
            cargo_build.env(
                "RUSTFLAGS",
                format!("{} -C debuginfo=0 --emit=llvm-ir", rustflags),
            );
        }
    }

    debug!("starting cargo build... {:?}", cargo_build);
    let error_msg = "cargo build failed";
    process::exec(&mut cargo_build, error_msg, opts.debug_mode())
        .expect(error_msg);
    debug!("cargo build finished...");

    let ext = match *opts.read() {
        crate::options::Options::Asm(_) => "s",
        crate::options::Options::LlvmIr(_) => "ll",
    };

    let deps_directory = crate::target::directory("deps");

    let mut output_files = vec![];

    // Scan files in "deps" target dir:
    output_files.append(&mut scan_directory(
        deps_directory.as_path(),
        |_, extension| extension == Some(ext),
    ));

    if let Some(example) = opts.example() {
        let example_directory = crate::target::directory("examples");
        let prefix = format!("{}-", example);

        // Scan files in "examples" target dir, while making sure
        // to only scanning those files belonging to the compiled example:
        output_files.append(&mut scan_directory(
            example_directory.as_path(),
            |stem, extension| {
                let has_prefix =
                    stem.map_or(false, |stem| stem.starts_with(&prefix));
                let has_extension = extension == Some(ext);
                has_prefix && has_extension
            },
        ));
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

/// Scan a given output directory for files matching the predicate:
fn scan_directory<P>(
    target_directory: &::std::path::Path,
    predicate: P,
) -> Vec<::std::path::PathBuf>
where
    P: Fn(Option<&str>, Option<&str>) -> bool,
{
    let mut output_files = Vec::new();
    for entry in ::walkdir::WalkDir::new(target_directory.clone()) {
        let e = entry.unwrap_or_else(|_| {
            panic!(
                "failed to iterate over the directory: {}",
                target_directory.display()
            )
        });
        let p = e.path();

        let stem = p.file_stem().and_then(|v| v.to_str());
        let extension = p.extension().and_then(|v| v.to_str());

        if predicate(stem, extension) {
            let p = p.to_path_buf();
            debug!("found file matching predicate: {}", p.display());
            output_files.push(p);
        }
    }
    output_files
}
