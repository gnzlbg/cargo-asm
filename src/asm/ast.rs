//! Abstract Syntax Tree
use options;

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
    line_off: usize,
    rust_loc_off: Option<Loc>,
}

impl Label {
    pub fn new(
        s: &str, line_off: usize, rust_loc_off: Option<Loc>
    ) -> Option<Self> {
        if s.ends_with(":") {
            return Some(Self {
                id: s.split_at(s.len() - 1).0.to_string(),
                line_off,
                rust_loc_off,
            });
        }
        None
    }
    pub fn line_offset(&self) -> usize {
        self.line_off
    }
    pub fn should_print(&self, opts: &options::Options) -> bool {
        true
    }
    pub fn format(&self, opts: &options::Options) -> String {
        format!("  {}:", self.id)
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
    pub path: String,
    pub index: usize,
    line_off: usize,
}

impl File {
    pub fn new(s: &str, line_off: usize) -> Option<Self> {
        if !s.starts_with(".file") {
            return None;
        }
        let path = s.split('"').nth(1).unwrap();
        let index = s.split_whitespace().nth(1).unwrap();
        Some(Self {
            path: path.to_string(),
            index: index.parse().unwrap(),
            line_off,
        })
    }
    pub fn line_offset(&self) -> usize {
        self.line_off
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Loc {
    pub file_idx: usize,
    pub offset: usize,
    line_off: usize,
}

impl Loc {
    pub fn new(s: &str, line_off: usize) -> Option<Self> {
        if !s.starts_with(".loc") {
            return None;
        }
        let mut it = s.split_whitespace();
        let file_idx = it.nth(1).unwrap();
        let offset = it.next().unwrap();
        Some(Self {
            file_idx: file_idx.parse().unwrap(),
            offset: offset.parse().unwrap(),
            line_off,
        })
    }
    pub fn line_offset(&self) -> usize {
        self.line_off
    }
}

#[derive(Clone, Debug)]
pub struct GenericDirective {
    string: String,
    line_off: usize,
}

impl GenericDirective {
    pub fn new(s: &str, line_off: usize) -> Option<Self> {
        if s.starts_with(".") {
            return Some(Self {
                string: s.to_string(),
                line_off,
            });
        }
        None
    }
    pub fn line_offset(&self) -> usize {
        self.line_off
    }
}

impl Directive {
    pub fn new(s: &str, line_off: usize) -> Option<Self> {
        if s.starts_with(".") {
            if let Some(file) = File::new(s, line_off) {
                return Some(Directive::File(file));
            }
            if let Some(loc) = Loc::new(s, line_off) {
                return Some(Directive::Loc(loc));
            }

            return Some(Directive::Generic(
                GenericDirective::new(s, line_off).unwrap(),
            ));
        }
        None
    }
    pub fn line_offset(&self) -> usize {
        match *self {
            Directive::File(ref f) => f.line_offset(),
            Directive::Loc(ref f) => f.line_offset(),
            Directive::Generic(ref f) => f.line_offset(),
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
    pub fn should_print(&self, opts: &options::Options) -> bool {
        opts.directives
    }
    pub fn format(&self, opts: &options::Options) -> String {
        format!("{:?}", self)
    }
}

/// Asm comments, e.g, ;; this is a comment.
#[derive(Debug, Clone)]
pub struct Comment {
    string: String,
    line_off: usize,
}

impl Comment {
    pub fn new(s: &str, line_off: usize) -> Option<Self> {
        if s.starts_with(";") {
            return Some(Self {
                string: s.to_string(),
                line_off,
            });
        }
        None
    }
    pub fn line_offset(&self) -> usize {
        self.line_off
    }
    pub fn should_print(&self, opts: &options::Options) -> bool {
        opts.comments
    }
    pub fn format(&self, opts: &options::Options) -> String {
        format!("  {}", self.string)
    }
}

/// Asm instructions: everything else (not a Comment, Directive, or Label).
#[derive(Debug, Clone)]
pub struct Instruction {
    instr: String,
    args: Vec<String>,
    line_off: usize,
    rust_loc_off: Option<Loc>,
}

impl Instruction {
    pub fn new(
        s: &str, line_off: usize, rust_loc_off: Option<Loc>
    ) -> Option<Self> {
        let mut iter = s.split_whitespace();
        let instr = iter.next().unwrap().to_string();
        let mut args = Vec::new();
        for arg in iter {
            args.push(arg.to_string());
        }
        if &instr == "call" {
            let demangled_function = ::demangle::demangle(&args[0]);
            args[0] = demangled_function;
        }
        return Some(Self {
            instr,
            args,
            line_off,
            rust_loc_off,
        });
    }
    pub fn line_offset(&self) -> usize {
        self.line_off
    }
    pub fn should_print(&self, opts: &options::Options) -> bool {
        true
    }
    pub fn format(&self, opts: &options::Options) -> String {
        format!("    {} {}", self.instr, self.args.join(" "))
    }
}

impl Statement {
    pub fn should_print(&self, opts: &options::Options) -> bool {
        match self {
            &Statement::Label(ref l) => l.should_print(opts),
            &Statement::Directive(ref l) => l.should_print(opts),
            &Statement::Instruction(ref l) => l.should_print(opts),
            &Statement::Comment(ref l) => l.should_print(opts),
        }
    }
    pub fn format(&self, opts: &options::Options) -> String {
        match self {
            &Statement::Label(ref l) => l.format(opts),
            &Statement::Directive(ref l) => l.format(opts),
            &Statement::Instruction(ref l) => l.format(opts),
            &Statement::Comment(ref l) => l.format(opts),
        }
    }
}
