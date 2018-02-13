//! Parses Rust code

use asm;

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
        &self, file_index: usize, line_idx: usize
    ) -> Option<String> {
        if let Some(file) = self.files.get(&file_index) {
            return file.line(line_idx);
        }
        None
    }
    pub fn line(&self, loc: asm::ast::Loc) -> Option<String> {
        self.line_at(loc.file_index, loc.file_line)
    }
    pub fn file_path(&self, loc: asm::ast::Loc) -> Option<::std::path::PathBuf> {
        if let Some(file) = self.files.get(&loc.file_index) {
            return Some(file.ast.path.clone());
        }
        None
    }
}

/// Returns the files used by the function.
#[cfg_attr(feature = "cargo-clippy", allow(use_debug))]
pub fn parse(
    function: &asm::ast::Function,
    file_table: &::std::collections::HashMap<usize, asm::ast::File>,
    mut opts: &mut ::options::Options,
) -> Files {
    use asm::ast::Statement;
    use asm::ast::Directive;
    let mut files = ::std::collections::HashMap::<usize, File>::new();

    // Go through all locations in the function and build a map of file indices
    // to files. The files contain a map of line indices to lines, the map is
    // initialized here to contain the lines pointed to by the locations.
    for s in &function.statements {
        if let Statement::Directive(Directive::Loc(ref l)) = s {
            files.entry(l.file_index).or_insert_with(|| {
                let ast = file_table.get(&l.file_index).expect(
                    &format!("[ERROR]: incomplete file table. Location {:?} 's file is not in the file table:\n{:?}",
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
        }
    }

    // Go through the line map of each file and fill in holes smaller than N
    // lines:
    let N = 5;
    for f in files.values_mut() {
        let mut prev = 0;
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

    // Corrects paths to Rust std library components:
    correct_rust_paths(&mut files, &mut opts);

    // Read the required lines from each Rust file:
    for f in files.values_mut() {
        use std::io::BufRead;
        let fh = ::std::fs::File::open(&f.ast.path)
            .expect(&format!("[ERROR]: failed to open file: {}", f.ast.path.display()));
        let file_buf = ::std::io::BufReader::new(&fh);

        for (line_idx, line) in file_buf.lines().enumerate() {
            let line_idx = line_idx + 1;
            if f.lines.contains_key(&line_idx) {
                let line = line.unwrap().trim().to_string();
                *f.lines.get_mut(&line_idx).unwrap() = Some(line);
            }
        }
    }

    for f in files.values_mut() {
        for (l_idx, line) in &f.lines {
            if line.is_none() && *l_idx != 0 {
                panic!(
                    "[ERROR]: could not read line {} of file {} ",
                    l_idx, f.ast.path.display()
                );
            }
        }
    }

    Files { files }
}

fn correct_rust_paths(files: &mut ::std::collections::HashMap<usize, File>, opts: &mut ::options::Options) {
    let rust =
        ::std::env::var("RUSTC").unwrap_or_else(|_| "rustc".to_string());

    let mut sysroot = ::std::process::Command::new(&rust);
    sysroot.arg("--print").arg("sysroot");

    let r = ::process::exec(
        &mut sysroot,
        "failed to call rustc --print sysroot",
        false,
    );

    let mut sysroot = match r {
        Ok((stdout, _stderr)) => ::std::path::PathBuf::from(stdout.trim()),
        Err(()) => panic!(),
    };
    if opts.verbose {
        println!("sysroot: {}", sysroot.display());
    }
    sysroot.parent();
    let rust_src_path = ::std::path::PathBuf::from("lib/rustlib/src/rust/src");
   
    ::path::push(&mut sysroot, &rust_src_path);
    if opts.verbose {
        eprintln!("merging {} with sysroot results in {}", rust_src_path.display(), sysroot.display());
    }

    let travis_rust_src_path = if cfg!(target_os = "macosx") {
     ::std::path::PathBuf::from("travis/build/rust-lang/rust/")
    } else {
        ::std::path::PathBuf::from("checkout/src/")
    };
    let mut missing_path_warning = false;
    for f in files.values_mut() {
        if ::path::contains(&f.ast.path, &travis_rust_src_path) {
            let path = {
                let tail = ::path::after(&f.ast.path, &travis_rust_src_path);
                let mut path = sysroot.clone();
                if opts.verbose {
                    eprintln!("merging {} with {}", path.display(), tail.display());
                }
                path.push(&tail);
                if opts.verbose {
                    eprintln!("  merge result: {}", path.display());
                }

                path
            };
            f.ast.path = path;
            if !f.ast.path.exists() {
                if !missing_path_warning && !opts.verbose {
                    eprintln!("[WARNING]: path does not exist: {}. Maybe the rust-src component is not installed?", f.ast.path.display());
                    missing_path_warning = true;
                    }
                opts.rust = false;
            }
        }
    }
    files.retain(|_k: &usize, f: &mut File| f.ast.path.exists());
}
