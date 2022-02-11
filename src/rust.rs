//! Parses Rust code

use crate::asm;
use crate::options::*;

use log::{debug, info};

#[derive(Debug, Clone)]
pub struct File {
    pub ast: asm::ast::File,
    pub lines: ::std::collections::BTreeMap<usize, Option<String>>,
}

impl File {
    pub fn line(&self, line_idx: usize) -> Option<String> {
        if let Some(l) = self.lines.get(&line_idx) {
            if let Some(ref l) = l {
                return Some(l.clone());
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct Files {
    pub files: ::std::collections::HashMap<usize, File>,
}

impl Files {
    pub fn line_at(
        &self,
        file_index: usize,
        line_idx: usize,
    ) -> Option<String> {
        if let Some(file) = self.files.get(&file_index) {
            return file.line(line_idx);
        }
        None
    }
    pub fn line(&self, loc: asm::ast::Loc) -> Option<String> {
        self.line_at(loc.file_index, loc.file_line)
    }
    pub fn file_path(
        &self,
        loc: asm::ast::Loc,
    ) -> Option<::std::path::PathBuf> {
        if let Some(file) = self.files.get(&loc.file_index) {
            return Some(file.ast.path.clone());
        }
        None
    }
}

/// Returns the files used by the function.
#[allow(clippy::use_debug)]
pub fn parse(
    function: &asm::ast::Function,
    file_table: &::std::collections::HashMap<usize, asm::ast::File>,
) -> Files {
    use crate::asm::ast::Directive;
    use crate::asm::ast::Statement;
    let mut files = ::std::collections::HashMap::<usize, File>::new();

    // Go through all locations in the function and build a map of file indices
    // to files. The files contain a map of line indices to lines, the map is
    // initialized here to contain the lines pointed to by the locations.
    for s in &function.statements {
        if let Statement::Directive(Directive::Loc(ref l)) = s {
            debug!("inserting locs: {:?}", l);
            files.entry(l.file_index).or_insert_with(|| {
                let ast = file_table.get(&l.file_index).unwrap_or_else(||
                    panic!("[ERROR]: incomplete file table. Location {:?} 's file is not in the file table:\n{:?}",
                             l, file_table));
                File {
                    ast: ast.clone(),
                    lines: ::std::collections::BTreeMap::new(),
                }
            });
            files
                .get_mut(&l.file_index)
                .unwrap()
                .lines
                .insert(l.file_line, None);
            debug!("files: {:?}", files);
        }
    }

    debug!("Done inserting files: {:?}", files);

    // Go through the line map of each file and fill in holes smaller than N
    // lines:
    let N = 5;
    for f in files.values_mut() {
        let prev = 0;
        let mut to_add = Vec::new();
        for &k in f.lines.keys() {
            if k > prev + 1 && k < prev + N {
                for i in prev + 1..k {
                    to_add.push(i);
                }
            }
        }
        for l in to_add {
            f.lines.insert(l, None);
        }
    }

    debug!("Done filing holes in files: {:?}", files);

    // Corrects paths to Rust std library components:
    correct_rust_paths(&mut files);

    debug!("Done correcting paths in files: {:?}", files);

    // Read the required lines from each Rust file:
    for f in files.values_mut() {
        use std::{
            fs::File,
            io::{BufRead, BufReader},
        };
        let fh = File::open(&f.ast.path).unwrap_or_else(|e| {
            panic!(
                "[ERROR]: failed to open file: {}\ncause: {:?}",
                f.ast.path.display(),
                e
            )
        });
        let file_buf = BufReader::new(&fh);

        for (line_idx, line) in file_buf.lines().enumerate() {
            let line_idx = line_idx + 1;
            if f.lines.contains_key(&line_idx) {
                let line = line.unwrap().trim().to_string();
                *f.lines.get_mut(&line_idx).unwrap() = Some(line);
            }
        }
    }

    debug!("Done reading lines in files: {:?}", files);

    for f in files.values_mut() {
        for (l_idx, line) in &f.lines {
            if line.is_none() && *l_idx != 0 {
                panic!(
                    "[ERROR]: could not read line {} of file {} ",
                    l_idx,
                    f.ast.path.display()
                );
            }
        }
    }

    Files { files }
}

fn correct_rust_paths(files: &mut ::std::collections::HashMap<usize, File>) {
    let rust =
        ::std::env::var("RUSTC").unwrap_or_else(|_| "rustc".to_string());

    let mut sysroot = ::std::process::Command::new(&rust);
    sysroot.arg("--print").arg("sysroot");

    let r = crate::process::exec(
        &mut sysroot,
        "failed to call rustc --print sysroot",
        false,
    );

    let mut sysroot = match r {
        Ok((stdout, _stderr)) => ::std::path::PathBuf::from(stdout.trim()),
        Err(()) => panic!(),
    };

    debug!("sysroot: {}", sysroot.display());
    let rust_src_path = crate::target::rust_src_path_component();

    crate::path::push(&mut sysroot, &rust_src_path);
    debug!(
        "merging {} with sysroot results in {}",
        rust_src_path.display(),
        sysroot.display()
    );

    // Identify core source components by their name in the referenced path
    let is_core_src_component = |path: std::path::Component| {
        path.as_os_str() == "libstd"
            || path.as_os_str() == "libcore"
            || path.as_os_str() == "liballoc"
    };

    let mut missing_path_warning = false;
    for f in files.values_mut() {
        debug!("correcting path: {}", f.ast.path.display());
        // Strip the build specific prefix and replace by the determined
        // sysroot
        if f.ast.path.components().any(is_core_src_component) {
            debug!("prepending {}", sysroot.display());
            let mut path = sysroot.clone();
            path.push(
                f.ast
                    .path
                    .components()
                    .skip_while(|p| !is_core_src_component(*p))
                    .collect::<std::path::PathBuf>()
                    .as_path(),
            );

            debug!("  merge result: {}", path.display());

            f.ast.path = path;
            if !f.ast.path.exists() {
                if !missing_path_warning {
                    info!("path does not exist: {}. Maybe the rust-src component is not installed? Use `rustup component add rust-src to install it!`", f.ast.path.display());
                    missing_path_warning = true;
                }
                opts.set_rust(false);
            }
        } else {
            debug!("couldn't correct {}", &f.ast.path.display());
        }
    }
    files.retain(|_k: &usize, f: &mut File| {
        if f.ast.path.exists() {
            true
        } else {
            println!("file {} does not exist!", f.ast.path.display());
            false
        }
    });
}
