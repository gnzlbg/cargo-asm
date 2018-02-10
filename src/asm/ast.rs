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
    pub fn rust_loc(&self) -> Option<Loc> {
        self.rust_loc_off
    }
    pub fn should_print(&self, _opts: &options::Options) -> bool {
        !self.id.starts_with("Lcfi") && !self.id.starts_with("Ltmp")
            && !self.id.starts_with("Lfunc_end")
    }
    pub fn format(&self, _opts: &options::Options) -> String {
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
    pub fn rust_loc(&self) -> Option<Loc> {
        None
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
    pub fn rust_loc(&self) -> Option<Loc> {
        None
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
    pub fn rust_loc(&self) -> Option<Loc> {
        None
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
    pub fn should_print(&self, opts: &options::Options) -> bool {
        opts.directives
    }
    pub fn format(&self, _opts: &options::Options) -> String {
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
    pub fn rust_loc(&self) -> Option<Loc> {
        None
    }
    pub fn should_print(&self, opts: &options::Options) -> bool {
        opts.comments
    }
    pub fn format(&self, _opts: &options::Options) -> String {
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
    pub fn rust_loc(&self) -> Option<Loc> {
        self.rust_loc_off
    }
    pub fn should_print(&self, _opts: &options::Options) -> bool {
        true
    }
    pub fn format(&self, opts: &options::Options) -> String {
        if opts.verbose {
            format!("    {} {} | rloc: {:?}", self.instr, self.args.join(" "), self.rust_loc().as_ref().map(|v| (v.file_idx, v.offset)))
        } else {
            format!("    {} {}", self.instr, self.args.join(" "))
        }
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
    pub fn rust_loc(&self, file: &File) -> Option<usize> {
        let loc = match self {
            &Statement::Label(ref l) => l.rust_loc(),
            &Statement::Directive(ref l) => l.rust_loc(),
            &Statement::Instruction(ref l) => l.rust_loc(),
            &Statement::Comment(ref l) => l.rust_loc(),
        };
        if loc.is_none() { return None; }
        let loc = loc.unwrap();
        if loc.file_idx != file.index { return None; }
        Some(loc.offset)
    }

}
