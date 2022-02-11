use super::*;
use crate::target::TargetInfo;

use serde_derive::Serialize;

/// Formatting of Rust source code:
#[derive(Clone, Serialize)]
struct Rust {
    line: String,
    path: ::std::path::PathBuf,
    loc: asm::ast::Loc,
}

impl Rust {
    fn new(
        line: String,
        path: ::std::path::PathBuf,
        loc: asm::ast::Loc,
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
#[allow(clippy::items_after_statements)]
fn write_output(
    kind: &Kind,
    function: &asm::ast::Function,
    target: &TargetInfo,
) {
    // Filter out what to print:
    match kind {
        Kind::Asm(ref a) => {
            use crate::asm::ast::Statement::*;
            match a {
                Comment(_) if !opts.print_comments() => return,
                Directive(_) if !opts.print_directives() => return,
                Label(ref l) => {
                    if l.id.contains("Lcfi")
                        || l.id.contains("Ltmp")
                        || l.id.contains("Lfunc_end")
                    {
                        return;
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
            use crate::asm::ast::Statement::*;
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

    use std::io::Write;
    use termcolor::{
        Buffer, BufferWriter, Color, ColorChoice, ColorSpec, WriteColor,
    };

    let bufwtr = if opts.use_colors() {
        BufferWriter::stdout(ColorChoice::Auto)
    } else {
        BufferWriter::stdout(ColorChoice::Never)
    };
    let mut buffer = bufwtr.buffer();
    buffer.set_color(&ColorSpec::new()).unwrap();

    // Write the indentation:
    write!(buffer, "{}", indent).unwrap();

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

    fn debug_mode_format(buffer: &mut Buffer, loc: Option<asm::ast::Loc>) {
        if let Some(loc) = loc {
            write!(buffer, "   [{}:{}]", loc.file_index, loc.file_line)
                .unwrap();
        } else {
            write!(buffer, "   [-:-]").unwrap();
        }
    }

    match kind {
        Kind::Asm(ref a) => {
            use crate::asm::ast::Statement::*;
            match a {
                Label(ref l) => {
                    buffer.set_color(&label_color).unwrap();
                    write!(buffer, "{}", l.id).unwrap();
                    write!(buffer, ":").unwrap();
                    if opts.debug_mode() {
                        debug_mode_format(&mut buffer, l.rust_loc());
                    }
                }
                Directive(ref d) => match d {
                    asm::ast::Directive::File(ref f) => {
                        write!(
                            buffer,
                            ".file {} \"{}\"",
                            f.index,
                            f.path.display(),
                        )
                        .unwrap();
                    }
                    asm::ast::Directive::Loc(ref l) => {
                        write!(
                            buffer,
                            ".loc {} {} {}",
                            l.file_index, l.file_line, l.file_column
                        )
                        .unwrap();
                    }
                    asm::ast::Directive::Generic(ref g) => {
                        write!(buffer, "{}", g.string).unwrap();
                    }
                },
                Comment(ref c) => {
                    buffer.set_color(&comment_color).unwrap();
                    write!(buffer, "{}", c.string).unwrap();
                    if opts.debug_mode() {
                        debug_mode_format(&mut buffer, c.rust_loc());
                    }
                }
                Instruction(ref i) => {
                    buffer.set_color(&instr_color).unwrap();
                    if i.args.is_empty() {
                        write!(buffer, "{}", i.instr).unwrap();
                    } else {
                        write!(buffer, "{: <7}", i.instr).unwrap();
                    }
                    if i.is_jump(target) {
                        // jump instructions
                        buffer.set_color(&label_color).unwrap();
                    } else if i.is_call(target) {
                        buffer.set_color(&instr_call_arg_color).unwrap();
                    } else {
                        buffer.set_color(&instr_arg_color).unwrap();
                    }
                    if !i.args.is_empty() {
                        write!(buffer, " {}", i.args.join(", ")).unwrap();
                    }
                    if opts.debug_mode() {
                        debug_mode_format(&mut buffer, i.rust_loc());
                    }
                }
            }
        }
        Kind::Rust(ref r) => {
            buffer.set_color(&rust_color).unwrap();
            if part_of_main_function {
                write!(buffer, "{}", r.line).unwrap();
                if opts.debug_mode() {
                    debug_mode_format(&mut buffer, Some(r.loc));
                }
            } else {
                write!(
                    buffer,
                    "{} ({}:{})",
                    r.line,
                    r.path.display(),
                    r.loc.file_line
                )
                .unwrap();
            }
        }
    }

    writeln!(buffer).unwrap();
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
        function.id.to_string()
    }
}

/// Returns true if the statement is in the function. It returns true if the
/// question cannot be answered.
fn is_stmt_in_function(
    f: &asm::ast::Function,
    stmt: &asm::ast::Statement,
) -> bool {
    let function_file_index = f.loc.map(|loc| loc.file_index);

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
    let function_file_index = f.loc.map(|loc| loc.file_index);

    if let Some(function_file_index) = function_file_index {
        return rust.loc.file_index == function_file_index;
    }
    true
}

fn make_path_relative(path: &mut ::std::path::PathBuf) {
    // The path might already be relative:
    if !path.is_absolute() {
        return;
    }

    // Trim std lib paths:
    let rust_src_path = crate::target::rust_src_path_component();
    let current_dir_path =
        ::std::env::current_dir().expect("cannot read the current dir");
    debug!("making paths relative: {}", path.display());
    debug!(" * std lib paths contain: {}", rust_src_path.display());
    debug!(" * local paths contain: {}", current_dir_path.display());

    if crate::path::contains(path, &rust_src_path) {
        let new_path = crate::path::after(path, &rust_src_path);
        debug!("  * rel path std: {}", new_path.display());
        *path = new_path;
    } else if crate::path::contains(path, &current_dir_path) {
        let new_path = crate::path::after(path, &current_dir_path);
        debug!("  * rel path loc: {}", new_path.display());
        *path = new_path;
    } else {
        debug!("  * path is neither local nor to std lib");
    }
}

/// Standard library paths are formatted relatively to the component name:
///
/// For example: libcore/... instead of /path/to/libcore/...
///
/// This functions trims their path.
///
/// The path of the current crate are also displayed as relative paths.
fn make_paths_relative(
    function: &mut asm::ast::Function,
    rust: &mut rust::Files,
) {
    if let Some(ref mut file) = &mut function.file {
        make_path_relative(&mut file.path)
    }
    for f in rust.files.values_mut() {
        make_path_relative(&mut f.ast.path)
    }
}

pub fn print(
    function: &mut asm::ast::Function,
    mut rust: rust::Files,
    target: &TargetInfo,
) {
    make_paths_relative(function, &mut rust);

    if !opts.rust() {
        // When emitting assembly without Rust code, print the requested
        // function path (the first function line will not be emitted):
        use std::io::Write;
        use termcolor::{
            BufferWriter, Color, ColorChoice, ColorSpec, WriteColor,
        };

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
        writeln!(buffer, "{}:", format_function_name(function)).unwrap();
        bufwtr.print(&buffer).unwrap();
    }

    let output = merge_rust_and_asm(function, &rust);

    for o in &output {
        write_output(o, function, target);
    }
}

fn merge_rust_and_asm(
    function: &asm::ast::Function,
    rust_files: &rust::Files,
) -> Vec<Kind> {
    let mut output = Vec::<Kind>::new();
    for stmt in &function.statements {
        if let Some(rust_loc) = stmt.rust_loc() {
            if let Some(rust_line) = rust_files.line(rust_loc).map(|line| {
                let path = rust_files.file_path(rust_loc).unwrap();
                Rust::new(line, path, rust_loc)
            }) {
                let rl = rust_line.line.trim().to_string();
                if !rl.starts_with("//") {
                    output.push(Kind::Rust(rust_line))
                }
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
                Kind::Rust(ref r) => r,
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
    write!(buffer, "[ERROR]: ").unwrap();
    buffer.set_color(&ColorSpec::new()).unwrap();
    write!(buffer, "{}", msg).unwrap();
    bufwtr.print(&buffer).unwrap();
}

pub fn to_json(
    function: &asm::ast::Function,
    rust_files: &rust::Files,
) -> Option<String> {
    let r = merge_rust_and_asm(function, rust_files);
    match ::serde_json::to_string_pretty(&r) {
        Ok(s) => Some(s),
        Err(e) => {
            error!("{}", e);
            None
        }
    }
}
