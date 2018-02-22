//! Abstract Syntax Tree

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
    pub fn new(s: &str) -> Option<Self> {
        let file_label = if cfg!(target_os = "windows") {
            ".cv_file"
        } else {
            // linux and macosx
            ".file"
        };
        if !s.starts_with(file_label) {
            return None;
        }

        let file_path_index_index = 1;
        let ws_tokens = s.split_whitespace().collect::<Vec<_>>();

        let file_path_index = 1;
        let colon_tokens = s.split('"').collect::<Vec<_>>();

        let path = colon_tokens.get(file_path_index).unwrap();
        // On Linux some files miss the file index:
        let index = ws_tokens
            .get(file_path_index_index)
            .unwrap()
            .parse()
            .unwrap_or(0);
        let path_str = path.trim();
        if cfg!(target_os = "windows") {
            // Replace \\ with \ on windows
            path_str.replace("\\\\", "\\");
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
    pub fn new(s: &str) -> Option<Self> {
        let loc_label = if cfg!(target_os = "windows") {
            ".cv_loc"
        } else {
            // linux and macosx
            ".loc"
        };
        if !s.starts_with(loc_label) {
            return None;
        }

        let file_index_index = if cfg!(target_os = "windows") {
            // on windows index 1 is the cv_func_id
            2
        } else {
            // linux and macosx
            1
        };

        let file_line_index = if cfg!(target_os = "windows") {
            3
        } else {
            // linux and macosx
            2
        };

        let file_column_index = if cfg!(target_os = "windows") {
            4
        } else {
            // linux and macosx
            3
        };

        let tokens = s.split_whitespace().collect::<Vec<_>>();
        let file_index = tokens.get(file_index_index).unwrap();
        let file_line = tokens.get(file_line_index).unwrap();
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
        Some(self.clone())
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct GenericDirective {
    pub string: String,
}

impl GenericDirective {
    pub fn new(s: &str) -> Option<Self> {
        if s.starts_with('.') {
            if (cfg!(target_os = "windows") || cfg!(target_os = "linux"))
                && s.ends_with(":")
            {
                // On Windows and Linux .{pattern}: is a label
                return None;
            }
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

impl Directive {
    pub fn new(s: &str) -> Option<Self> {
        if s.starts_with('.') {
            if (cfg!(target_os = "windows") || cfg!(target_os = "linux"))
                && s.ends_with(":")
            {
                // On Windows and Linux .{pattern}: is a label
                return None;
            }
            if let Some(file) = File::new(s) {
                return Some(Directive::File(file));
            }
            if let Some(loc) = Loc::new(s) {
                return Some(Directive::Loc(loc));
            }
            return Some(Directive::Generic(GenericDirective::new(s).unwrap()));
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
    pub fn new(s: &str, rust_loc: Option<Loc>) -> Option<Self> {
        let mut iter = s.split(|c: char| c.is_whitespace() || c == ',');
        let instr = iter.next().unwrap().trim().to_string();
        let mut args = Vec::new();
        for arg in iter {
            let arg_s = arg.trim().to_string();
            if !arg_s.is_empty() {
                args.push(arg_s);
            }
        }
        if instr == "call" {
            let demangled_function = ::demangle::demangle(&args[0]);
            args[0] = demangled_function;
        }
        Some(Self {
            instr,
            args,
            rust_loc,
        })
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
