//! Abstract Syntax Tree

use crate::target::TargetInfo;

use log::debug;
use serde_derive::Serialize;

/// AST of an asm function
#[derive(Debug, Clone)]
pub struct Function {
    pub id: String,
    pub file: Option<File>,
    pub loc: Option<Loc>,
    pub statements: Vec<Statement>,
}

/// Statemets
#[derive(Debug, Clone, Serialize)]
pub enum Statement {
    Label(Label),
    Directive(Directive),
    Instruction(Instruction),
    Comment(Comment),
}

/// Asm labels, e.g., LBB0:
#[derive(Debug, Clone, Serialize)]
pub struct Label {
    pub id: String,
    rust_loc: Option<Loc>,
}

impl Label {
    pub fn new(s: &str, rust_loc: Option<Loc>) -> Option<Self> {
        if s.ends_with(':') {
            return Some(Self {
                id: s.split_at(s.len() - 1).0.trim().to_string(),
                rust_loc,
            });
        }
        None
    }
    pub fn rust_loc(&self) -> Option<Loc> {
        self.rust_loc
    }
}

/// Asm directives, e.g, .static ...
#[derive(Debug, Clone, Serialize)]
pub enum Directive {
    File(File),
    Loc(Loc),
    Generic(GenericDirective),
}

#[derive(PartialEq, Debug, Clone, Serialize)]
pub struct File {
    pub path: ::std::path::PathBuf,
    pub index: usize,
}

impl File {
    pub fn new(s: &str, target: &TargetInfo) -> Option<Self> {
        fn contains_file_label(s: &str, target: &TargetInfo) -> bool {
            if target.is_windows() {
                s.starts_with(".cv_file") && !s.starts_with(".cv_filec")
            } else {
                s.starts_with(".file")
            }
        }

        if !contains_file_label(s, target) {
            return None;
        }
        debug!("parsing file directive: {}", s);

        let file_path_index_index = 1;
        let ws_tokens = s.split_whitespace().collect::<Vec<_>>();

        let file_path_index = 1;
        let colon_tokens = s.split('"').collect::<Vec<_>>();

        let path = colon_tokens
            .get(file_path_index)
            .unwrap_or_else(|| panic!("could not get file path of {} | file_path_index: {} | tokens: {:?}", s, file_path_index, &colon_tokens));

        if colon_tokens.is_empty() {
            return None;
        }
        // On Linux some files miss the file index:
        let index = ws_tokens
            .get(file_path_index_index)
            .unwrap_or_else(|| panic!("could not get file index of {} | file_path_index_index: {} | tokens: {:?}", s, file_path_index_index, &ws_tokens))
            .parse()
            .unwrap_or(0);
        if ws_tokens.is_empty() {
            return None;
        }

        let mut path_str = path.trim().to_string();
        if target.is_windows() {
            // Replace \\ with \ on windows
            replace_slashes(&mut path_str);
            // FIXME: on windows these paths do not follow the UNC, but we
            // can't canonicalize them here because they might not
            // exist (e.g. they might point into the std library
            // path of where the std library was built: this path
            // is not the same as the path of where the rust-src
            // component is installed in the user's machine, and the rust-src
            // component does not necessarily need to be installed.
        }
        let path = ::std::path::PathBuf::from(path_str);
        debug!("parsed file path: {}", path.display());

        Some(Self { path, index })
    }
    pub fn rust_loc(&self) -> Option<Loc> {
        None
    }
}

#[derive(PartialEq, Debug, Copy, Clone, Serialize)]
pub struct Loc {
    pub file_index: usize,
    pub file_line: usize,
    pub file_column: usize,
}

impl Loc {
    pub fn new(s: &str, target: &TargetInfo) -> Option<Self> {
        fn contains_loc_label(s: &str, target: &TargetInfo) -> bool {
            if target.is_windows() {
                s.contains(".cv_loc")
            } else {
                s.contains(".loc")
            }
        }

        if !contains_loc_label(s, target) {
            return None;
        }

        let file_index_index = if target.is_windows() {
            // on windows index 1 is the cv_func_id
            2
        } else {
            // linux and macosx
            1
        };

        let file_line_index = if target.is_windows() {
            3
        } else {
            // linux and macosx
            2
        };

        let file_column_index = if target.is_windows() {
            4
        } else {
            // linux and macosx
            3
        };

        let tokens = s.split_whitespace().collect::<Vec<_>>();
        let file_index = tokens[file_index_index];
        let file_line = tokens[file_line_index];
        // On Linux the file-column is not emitted so we just set it to zero
        // here.
        let file_column = tokens.get(file_column_index).unwrap_or(&"0");
        Some(Self {
            file_index: file_index.parse().unwrap(),
            file_line: file_line.parse().unwrap(),
            file_column: file_column.parse().unwrap(),
        })
    }
    pub fn rust_loc(&self) -> Option<Self> {
        Some(*self)
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct GenericDirective {
    pub string: String,
}

fn is_directive(s: &str) -> bool {
    // Directives start with .
    if !s.starts_with('.') {
        return false;
    }
    // And do not end with : (in this case they are probably labels)
    if s.ends_with(':') {
        return false;
    }
    true
}

impl GenericDirective {
    pub fn new(s: &str) -> Option<Self> {
        if is_directive(s) {
            Some(Self {
                string: s.trim().to_string(),
            })
        } else {
            None
        }
    }
    pub fn rust_loc(&self) -> Option<Loc> {
        None
    }
}

impl Directive {
    pub fn new(s: &str, target: &TargetInfo) -> Option<Self> {
        if is_directive(s) {
            if let Some(file) = File::new(s, target) {
                return Some(Directive::File(file));
            }
            if let Some(loc) = Loc::new(s, target) {
                return Some(Directive::Loc(loc));
            }
            return Some(Directive::Generic(
                GenericDirective::new(s).unwrap(),
            ));
        }
        None
    }
    pub fn rust_loc(&self) -> Option<Loc> {
        match *self {
            Directive::File(ref f) => f.rust_loc(),
            Directive::Loc(ref f) => f.rust_loc(),
            Directive::Generic(ref f) => f.rust_loc(),
        }
    }
    pub fn file(&self) -> Option<File> {
        match *self {
            Directive::File(ref f) => Some(f.clone()),
            _ => None,
        }
    }
    pub fn loc(&self) -> Option<Loc> {
        match *self {
            Directive::Loc(ref l) => Some(*l),
            _ => None,
        }
    }
}

/// Asm comments, e.g, ;; this is a comment.
#[derive(Debug, Clone, Serialize)]
pub struct Comment {
    pub string: String,
}

impl Comment {
    pub fn new(s: &str) -> Option<Self> {
        if s.starts_with(';') {
            return Some(Self {
                string: s.trim().to_string(),
            });
        }
        None
    }
    pub fn rust_loc(&self) -> Option<Loc> {
        None
    }
}

/// Asm instructions: everything else (not a Comment, Directive, or Label).
#[derive(Debug, Clone, Serialize)]
pub struct Instruction {
    pub instr: String,
    pub args: Vec<String>,
    rust_loc: Option<Loc>,
}

impl Instruction {
    pub fn new(
        s: &str,
        rust_loc: Option<Loc>,
        target: &TargetInfo,
    ) -> Option<Self> {
        let mut iter = s.split(|c: char| c.is_whitespace() || c == ',');
        let instr = iter.next().unwrap().trim().to_string();
        let mut args = Vec::new();
        for arg in iter {
            let arg = arg.trim().to_string();
            if !arg.is_empty() {
                args.push(arg);
            }
        }
        let mut v = Self {
            instr,
            args,
            rust_loc,
        };
        v.demangle_args(target);
        Some(v)
    }
    pub fn is_jump(&self, target: &TargetInfo) -> bool {
        if target.is_x86()
            || target.is_i386()
            || target.is_i586()
            || target.is_i686()
        {
            self.instr.starts_with('j') && self.args.len() == 1
        } else if target.is_aarch64() {
            self.instr == "b" || self.instr.starts_with("b.")
        } else if target.is_arm() || target.is_sparc() {
            self.args.iter().any(|x| x.starts_with(".L"))
        } else if target.is_power() {
            self.instr.starts_with('b')
                && self.instr != "bl"
                && self.args.len() == 2
        } else if target.is_mips() {
            self.instr.starts_with('b') && self.instr.len() > 1
        } else {
            debug!("unimplemented target");
            false
        }
    }
    pub fn is_call(&self, target: &TargetInfo) -> bool {
        if target.is_x86()
            || target.is_i386()
            || target.is_i586()
            || target.is_i686()
            || target.is_sparc()
        {
            self.instr.starts_with("call")
        } else if target.is_aarch64() || target.is_power() || target.is_arm() {
            self.instr == "bl"
        } else {
            debug!("unimplemented target");
            false
        }
    }

    fn demangle_args(&mut self, target: &TargetInfo) {
        if target.is_mips() {
            // On mips we need to inspect every argument of every instruction.
            for arg in &mut self.args {
                if !arg.contains("_Z") {
                    continue;
                }
                let f = arg.find("_Z").unwrap();
                let l = arg.find(')');
                if l.is_none() {
                    continue;
                }
                let l = l.unwrap();
                let name_to_demangle = &arg[f..l].to_string();
                let demangled_name =
                    crate::demangle::demangle(name_to_demangle, target);
                let new_arg = arg.replace(name_to_demangle, &demangled_name);
                *arg = new_arg;
            }
        } else if self.is_call(target) {
            // Typically, we just check if the instruction is a call
            // instruction, and the mangle the first argument.
            let demangled_function =
                crate::demangle::demangle(&self.args[0], target);
            self.args[0] = demangled_function;
        }
    }

    pub fn rust_loc(&self) -> Option<Loc> {
        self.rust_loc
    }
}

impl Statement {
    pub fn rust_loc(&self) -> Option<Loc> {
        match self {
            Statement::Label(ref l) => l.rust_loc(),
            Statement::Directive(ref l) => l.rust_loc(),
            Statement::Instruction(ref l) => l.rust_loc(),
            Statement::Comment(ref l) => l.rust_loc(),
        }
    }
}

fn replace_slashes(s: &mut String) {
    let n = s.replace(r#"\\"#, r#"\"#);
    *s = n;
}

#[cfg(test)]
mod tests {
    #[test]
    fn replace_slashes() {
        let mut windows_path = r#"C:\\projects\\cargo-asm\\cargo-asm-test\\lib_crate\\src\\bar.rs"#.to_string();
        let windows_path_norm =
            r#"C:\projects\cargo-asm\cargo-asm-test\lib_crate\src\bar.rs"#
                .to_string();
        super::replace_slashes(&mut windows_path);
        assert_eq!(windows_path_norm, windows_path);
    }
}
