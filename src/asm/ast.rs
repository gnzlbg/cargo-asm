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
#[derive(Debug, Clone)]
pub enum Statement {
    Label(Label),
    Directive(Directive),
    Instruction(Instruction),
    Comment(Comment),
}

/// Asm labels, e.g., LBB0:
#[derive(Debug, Clone)]
pub struct Label {
    pub id: String,
    rust_loc: Option<Loc>,
}

impl Label {
    pub fn new(s: &str, rust_loc: Option<Loc>) -> Option<Self> {
        if s.ends_with(':') {
            return Some(Self {
                id: s.split_at(s.len() - 1).0.to_string(),
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
#[derive(Debug, Clone)]
pub enum Directive {
    File(File),
    Loc(Loc),
    Generic(GenericDirective),
}

#[derive(PartialEq, Debug, Clone)]
pub struct File {
    pub path: ::std::path::PathBuf,
    pub index: usize,
}

impl File {
    pub fn new(s: &str) -> Option<Self> {
        if !s.starts_with(".file") {
            return None;
        }
        let path = s.split('"').nth(1).unwrap();
        let index = s.split_whitespace().nth(1).unwrap().parse().unwrap_or(0);

        Some(Self {
            path: ::std::path::PathBuf::from(path),
            index,
        })
    }
    pub fn rust_loc(&self) -> Option<Loc> {
        None
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Loc {
    pub file_index: usize,
    pub file_line: usize,
    pub file_column: usize,
}

impl Loc {
    pub fn new(s: &str) -> Option<Self> {
        if !s.starts_with(".loc") {
            return None;
        }
        let mut it = s.split_whitespace();
        let file_index = it.nth(1).unwrap();
        let file_line = it.next().unwrap();
        let file_column = it.next().unwrap_or("0");
        Some(Self {
            file_index: file_index.parse().unwrap(),
            file_line: file_line.parse().unwrap(),
            file_column: file_column.parse().unwrap(),
        })
    }
    pub fn rust_loc(&self) -> Option<Self> {
        None
    }
}

#[derive(Clone, Debug)]
pub struct GenericDirective {
    pub string: String,
}

impl GenericDirective {
    pub fn new(s: &str) -> Option<Self> {
        if s.starts_with('.') {
            return Some(Self {
                string: s.to_string(),
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
#[derive(Debug, Clone)]
pub struct Comment {
    pub string: String,
}

impl Comment {
    pub fn new(s: &str) -> Option<Self> {
        if s.starts_with(';') {
            return Some(Self {
                string: s.to_string(),
            });
        }
        None
    }
    pub fn rust_loc(&self) -> Option<Loc> {
        None
    }
}

/// Asm instructions: everything else (not a Comment, Directive, or Label).
#[derive(Debug, Clone)]
pub struct Instruction {
    pub instr: String,
    pub args: Vec<String>,
    rust_loc: Option<Loc>,
}

impl Instruction {
    pub fn new(s: &str, rust_loc: Option<Loc>) -> Option<Self> {
        let mut iter = s.split_whitespace();
        let instr = iter.next().unwrap().to_string();
        let mut args = Vec::new();
        for arg in iter {
            args.push(arg.to_string());
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
