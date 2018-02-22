use super::*;

/// Formatting of Rust source code:
#[derive(Clone, Serialize)]
struct Rust {
    line: String,
    path: ::std::path::PathBuf,
    loc: asm::ast::Loc,
}

impl Rust {
    fn new(
        line: String, path: ::std::path::PathBuf, loc: asm::ast::Loc
    ) -> Self {
        Self { line, path, loc }
    }
}

/// Type of node to display
#[derive(Serialize)]
enum Kind {
    Asm(asm::ast::Statement),
    Rust(Rust),
}

/// Prints `kind` using `opts`.
#[cfg_attr(feature = "cargo-clippy", allow(items_after_statements))]
fn write_output(kind: &Kind, function: &asm::ast::Function) {
    // Filter out what to print:
    match kind {
        Kind::Asm(ref a) => {
            use asm::ast::Statement::*;
            match a {
                Comment(_) if !opts.print_comments() => return,
                Directive(_) if !opts.print_directives() => return,
                Label(ref l) => {
                    if cfg!(target_os = "windows") || cfg!(target_os = "linux")
                    {
                        if l.id.starts_with(".Lcfi")
                            || l.id.starts_with(".Ltmp")
                            || l.id.starts_with(".Lfunc_end")
                        {
                            return;
                        }
                    } else {
                        if l.id.starts_with("Lcfi") || l.id.starts_with("Ltmp")
                            || l.id.starts_with("Lfunc_end")
                        {
                            return;
                        }
                    }
                }
                _ => {}
            }
        }
        Kind::Rust(_) => {
            if !opts.rust() {
                return;
            }
        }
    }

    // Is the current code part of the main function?
    let part_of_main_function = match kind {
        Kind::Asm(ref a) => is_stmt_in_function(function, a),
        Kind::Rust(ref r) => is_rust_in_function(function, r),
    };

    let indent = match kind {
        Kind::Asm(ref a) => {
            use asm::ast::Statement::*;
            match *a {
                Comment(_) | Directive(_) | Instruction(_) => {
                    if !opts.rust() || part_of_main_function {
                        1
                    } else {
                        5
                    }
                }
                Label(_) => 0,
            }
        }
        Kind::Rust(_) => {
            if part_of_main_function {
                1
            } else {
                5
            }
        }
    };
    let indent = (0..indent).map(|_| " ").collect::<String>();

    use termcolor::{Buffer, BufferWriter, Color, ColorChoice, ColorSpec,
                    WriteColor};
    use std::io::Write;

    let bufwtr = if opts.use_colors() {
        BufferWriter::stdout(ColorChoice::Auto)
    } else {
        BufferWriter::stdout(ColorChoice::Never)
    };
    let mut buffer = bufwtr.buffer();
    buffer.set_color(&ColorSpec::new()).unwrap();

    // Write the indentation:
    write!(&mut buffer, "{}", indent).unwrap();

    let mut instr_color = ColorSpec::new();
    instr_color
        .set_intense(true)
        .set_fg(Some(Color::Blue))
        .set_bold(true);
    let mut label_color = ColorSpec::new();
    label_color
        .set_intense(true)
        .set_fg(Some(Color::Green))
        .set_bold(true);
    let comment_color = ColorSpec::new();
    let instr_arg_color = ColorSpec::new();
    let mut rust_color = ColorSpec::new();
    rust_color
        .set_intense(true)
        .set_fg(Some(Color::Red))
        .set_bold(true);
    let instr_call_arg_color = rust_color.clone();

    fn debug_mode_format(mut buffer: &mut Buffer, loc: Option<asm::ast::Loc>) {
        if let Some(loc) = loc {
            write!(&mut buffer, "   [{}:{}]", loc.file_index, loc.file_line)
                .unwrap();
        } else {
            write!(&mut buffer, "   [{0}:{0}]", "-").unwrap();
        }
    }

    match kind {
        Kind::Asm(a) => {
            use asm::ast::Statement::*;
            match a {
                Label(l) => {
                    buffer.set_color(&label_color).unwrap();
                    write!(&mut buffer, "{}", l.id).unwrap();
                    write!(&mut buffer, ":").unwrap();
                    if opts.debug_mode() {
                        debug_mode_format(&mut buffer, l.rust_loc());
                    }
                }
                Directive(d) => match d {
                    asm::ast::Directive::File(f) => {
                        write!(
                            &mut buffer,
                            ".file {} \"{}\"",
                            f.index,
                            f.path.display(),
                        ).unwrap();
                    }
                    asm::ast::Directive::Loc(l) => {
                        write!(
                            &mut buffer,
                            ".loc {} {} {}",
                            l.file_index, l.file_line, l.file_column
                        ).unwrap();
                    }
                    asm::ast::Directive::Generic(g) => {
                        write!(&mut buffer, "{}", g.string).unwrap();
                    }
                },
                Comment(c) => {
                    buffer.set_color(&comment_color).unwrap();
                    write!(&mut buffer, "{}", c.string).unwrap();
                    if opts.debug_mode() {
                        debug_mode_format(&mut buffer, c.rust_loc());
                    }
                }
                Instruction(i) => {
                    buffer.set_color(&instr_color).unwrap();
                    if i.args.is_empty() {
                        write!(&mut buffer, "{}", i.instr).unwrap();
                    } else {
                        write!(&mut buffer, "{: <7}", i.instr).unwrap();
                    }
                    if i.instr.starts_with('j') && i.args.len() == 1 {
                        // jump instructions
                        buffer.set_color(&label_color).unwrap();
                    } else if i.instr.starts_with("call") {
                        buffer.set_color(&instr_call_arg_color).unwrap();
                    } else {
                        buffer.set_color(&instr_arg_color).unwrap();
                    }
                    if !i.args.is_empty() {
                        write!(&mut buffer, " {}", i.args.join(", ")).unwrap();
                    }
                    if opts.debug_mode() {
                        debug_mode_format(&mut buffer, i.rust_loc());
                    }
                }
            }
        }
        Kind::Rust(r) => {
            buffer.set_color(&rust_color).unwrap();
            if part_of_main_function {
                write!(&mut buffer, "{}", r.line).unwrap();
                if opts.debug_mode() {
                    debug_mode_format(&mut buffer, Some(r.loc));
                }
            } else {
                write!(
                    &mut buffer,
                    "{} ({}:{})",
                    r.line,
                    r.path.display(),
                    r.loc.file_line
                ).unwrap();
            }
        }
    }

    //println!("{}{}", indent, output);
    write!(&mut buffer, "\n").unwrap();
    bufwtr.print(&buffer).unwrap();
}

fn format_function_name(function: &asm::ast::Function) -> String {
    if function.file.is_some() && function.loc.is_some() {
        if let Some(ref file) = &function.file {
            if let Some(ref loc) = &function.loc {
                return format!(
                    "{} ({}:{})",
                    function.id,
                    file.path.display(),
                    loc.file_line
                );
            }
        }
        unreachable!()
    } else {
        assert!(function.file.is_none());
        assert!(function.loc.is_none());
        format!("{}", function.id)
    }
}

/// Returns true if the statement is in the function. It returns true if the
/// question cannot be answered.
fn is_stmt_in_function(
    f: &asm::ast::Function, stmt: &asm::ast::Statement
) -> bool {
    let function_file_index = if let Some(loc) = f.loc {
        Some(loc.file_index)
    } else {
        None
    };

    if let Some(function_file_index) = function_file_index {
        if let Some(loc) = stmt.rust_loc() {
            return loc.file_index == function_file_index;
        }
    }

    true
}

/// Returns true if the rust code belongs to the function `f`. It returns true
/// if the question cannot be answered.
fn is_rust_in_function(f: &asm::ast::Function, rust: &Rust) -> bool {
    let function_file_index = if let Some(loc) = f.loc {
        Some(loc.file_index)
    } else {
        None
    };

    if let Some(function_file_index) = function_file_index {
        return rust.loc.file_index == function_file_index;
    }
    true
}

/// Standard library paths are formatted relatively to the component name:
///
/// For example: libcore/... instead of /path/to/libcore/...
///
/// This functions trims their path.
fn make_paths_relative(rust: &mut rust::Files) {
    // Trim std lib paths:
    let rust_src_path =
        if cfg!(target_os = "macos") || cfg!(target_os = "linux") {
            ::std::path::PathBuf::from("lib/rustlib/src/rust/src/")
        } else if cfg!(target_os = "windows") {
            ::std::path::PathBuf::from(r#"lib\rustlib\src\rust\src\"#)
        } else {
            unimplemented!()
        };
    let current_dir_path =
        ::std::env::current_dir().expect("cannot read the current dir");
    debug!("making std lib paths relative");
    for f in rust.files.values_mut() {
        if !f.ast.path.is_absolute() {
            continue;
        }
        debug!("  * path: {}", f.ast.path.display());
        let ast = f.ast.clone();
        if ::path::contains(&ast.path, &rust_src_path) {
            let new_path = ::path::after(&ast.path, &rust_src_path);
            debug!("  * rel path: {}", new_path.display());
            f.ast.path = new_path;
        } else if ::path::contains(&ast.path, &current_dir_path) {
            let new_path = ::path::after(&ast.path, &current_dir_path);
            debug!("  * rel path: {}", new_path.display());
            f.ast.path = new_path;
            continue;
        }
    }
}

pub fn print(function: &asm::ast::Function, mut rust: rust::Files) {
    if !opts.rust() {
        // When emitting assembly without Rust code, print the requested
        // function path (the first function line will not be emitted):
        use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec,
                        WriteColor};
        use std::io::Write;

        let mut rust_color = ColorSpec::new();
        rust_color
            .set_intense(true)
            .set_fg(Some(Color::Red))
            .set_bold(true);

        let bufwtr = if opts.use_colors() {
            BufferWriter::stdout(ColorChoice::Auto)
        } else {
            BufferWriter::stdout(ColorChoice::Never)
        };
        let mut buffer = bufwtr.buffer();
        buffer.set_color(&rust_color).unwrap();
        writeln!(&mut buffer, "{}:", format_function_name(function)).unwrap();
        bufwtr.print(&buffer).unwrap();
    }

    make_paths_relative(&mut rust);

    let output = merge_rust_and_asm(function, &rust);

    for o in &output {
        write_output(o, function);
    }
    return;
}

fn merge_rust_and_asm(
    function: &asm::ast::Function, rust_files: &rust::Files
) -> Vec<Kind> {
    let mut output = Vec::<Kind>::new();
    for stmt in &function.statements {
        if let Some(rust_loc) = stmt.rust_loc() {
            if let Some(rust_line) = rust_files.line(rust_loc).map(|line| {
                let path = rust_files.file_path(rust_loc).unwrap();
                Rust::new(line, path, rust_loc)
            }) {
                output.push(Kind::Rust(rust_line))
            } else {
                // TODO: debug mode
                // println!("cannot find loc {:?} for line {:?}", rust_loc,
                // line); println!("{:?}", rust_files);
            }
        }

        let asm = Kind::Asm(stmt.clone());
        output.push(asm);
    }

    // Remove duplicates:
    {
        let mut last_rust: Option<Rust> = None;
        output.retain(|v| {
            let r = match v {
                Kind::Rust(r) => r,
                _ => return true,
            };

            if let Some(ref last_rust) = &last_rust {
                if last_rust.loc == r.loc {
                    return false;
                }
            }
            last_rust = Some(r.clone());
            true
        });
    }

    output
}

pub fn write_error(msg: &str) {
    use std::io::Write;
    use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};
    let mut error_color = ColorSpec::new();
    error_color
        .set_intense(true)
        .set_fg(Some(Color::Red))
        .set_bold(true);

    let bufwtr = if opts.use_colors() {
        BufferWriter::stderr(ColorChoice::Auto)
    } else {
        BufferWriter::stderr(ColorChoice::Never)
    };
    let mut buffer = bufwtr.buffer();
    buffer.set_color(&error_color).unwrap();
    write!(&mut buffer, "[ERROR]: ").unwrap();
    buffer.set_color(&ColorSpec::new()).unwrap();
    write!(&mut buffer, "{}", msg).unwrap();
    bufwtr.print(&buffer).unwrap();
}

pub fn to_json(
    function: &asm::ast::Function, rust_files: &rust::Files
) -> Option<String> {
    let r = merge_rust_and_asm(&function, rust_files);
    match ::serde_json::to_string_pretty(&r) {
        Ok(s) => Some(s),
        Err(e) => {
            error!("{}", e);
            None
        }
    }
}
