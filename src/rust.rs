//! Parses Rust code

use asm;

#[derive(Debug)]
pub struct File {
    pub ast: asm::ast::File,
    pub lines: ::std::collections::BTreeMap<usize, Option<String>>,
}

impl File {
    pub fn line(&self, line_idx: usize) -> Option<String> {
        if let Some(ref l) = self.lines.get(&line_idx) {
            if let &&Some(ref l) = l {
                return Some(l.clone());
            }
        }
        None
    }
}

#[derive(Debug)]
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
    pub fn file_path(&self, loc: asm::ast::Loc) -> Option<String> {
        if let Some(file) = self.files.get(&loc.file_index) {
            return Some(file.ast.path.clone());
        }
        None
    }

}

/// Returns the files used by the function.
pub fn parse(
    function: &asm::ast::Function,
    file_table: ::std::collections::HashMap<usize, asm::ast::File>,
) -> Files {
    use asm::ast::Statement;
    use asm::ast::Directive;
    let mut files = ::std::collections::HashMap::new();

    // Go through all locations in the function and build a map of file indices
    // to files. The files contain a map of line indices to lines, the map is
    // initialized here to contain the lines pointed to by the locations.
    for s in function.statements.iter() {
        match s {
            &Statement::Directive(Directive::Loc(ref l)) => {
                if !files.contains_key(&l.file_index) {
                    let ast = file_table.get(&l.file_index).expect(
                            &format!("[ERROR]: incomplete file table. Location {:?} 's file is not in the file table:\n{:?}",
                                    l, file_table));
                    files.insert(
                        l.file_index,
                        File {
                            ast: ast.clone(),
                            lines: ::std::collections::BTreeMap::new(),
                        },
                    );
                }
                files
                    .get_mut(&l.file_index)
                    .unwrap()
                    .lines
                    .insert(l.file_line, None);
            }
            _ => {}
        }
    }

    // Go through the line map of each file and fill in holes smaller than N
    // lines:
    let N = 5;
    for (_k, f) in &mut files {
        let mut prev = 0;
        let mut to_add = Vec::new();
        for (&k, _l) in &f.lines {
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
    correct_rust_paths(&mut files);

    // Read the required lines from each Rust file:
    for (_k, f) in &mut files {
        use std::io::BufRead;
        let fh = ::std::fs::File::open(&f.ast.path)
            .expect(&format!("[ERROR]: failed to open file: {}", f.ast.path));
        let file_buf = ::std::io::BufReader::new(&fh);

        for (line_idx, line) in file_buf.lines().enumerate() {
            let line_idx = line_idx + 1;
            if f.lines.contains_key(&line_idx) {
                let line = line.unwrap().trim().to_string();
                *f.lines.get_mut(&line_idx).unwrap() = Some(line);
            }
        }
    }

    for (_k, f) in &mut files {
        for (l_idx, line) in &f.lines {
            if line.is_none() && *l_idx != 0 {
                panic!(
                    "[ERROR]: could not read line {} of file {} ",
                    l_idx, f.ast.path
                );
            }
        }
    }

    Files { files }
}

fn correct_rust_paths(files: &mut ::std::collections::HashMap<usize, File>) {
    let rust = ::std::env::var("RUSTC").unwrap_or("rustc".to_string());

    let mut sysroot = ::std::process::Command::new(&rust);
    sysroot.arg("--print").arg("sysroot");

    let r = ::process::exec(
        &mut sysroot,
        "failed to call rustc --print sysroot",
        false,
    );

    let mut sysroot = match r {
        Ok((stdout, _stderr)) => stdout,
        Err(()) => panic!(),
    };
    sysroot.pop();
    sysroot.push_str("/lib/rustlib/src/rust/");

    for (_k, f) in files {
        if f.ast.path.contains("travis/build/rust-lang/rust/") {
            let path = {
                let tail = f.ast
                    .path
                    .split("travis/build/rust-lang/rust/")
                    .nth(1)
                    .unwrap();
                let mut path = sysroot.clone();
                path.push_str(tail);
                path
            };
            f.ast.path = path;
        }
    }
}
