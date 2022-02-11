use self::ast::*;
use super::ast;
use crate::options::{opts, Ext};
use crate::target::TargetInfo;

use log::{debug, error};

/// Parses the body of a function `path` from the `function_line`
fn function_body(
    function_lines: Vec<String>,
    path: &str,
    target: &TargetInfo,
) -> ast::Function {
    let mut function = Function {
        id: path.to_string(),
        file: None,
        loc: None,
        statements: Vec::new(),
    };

    let mut current_loc: Option<Loc> = None;

    // Parse assembly lines of the function.
    //
    // The first line corresponds to the function path, we skip it since we
    // already know it.
    for (line_off, line) in function_lines
        .into_iter()
        .skip(1)
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .enumerate()
    {
        debug!("parsing line: {}", line);

        // If the line contains a comment, split the line at the comment.
        let (node_str, comment_str) =
            if let Some(comment_start) = line.find(';') {
                debug!(" * contains a comment at: {:?}", comment_start);
                let (node_str, comment_str) = line.split_at(comment_start);
                (node_str, comment_str)
            } else {
                (line.as_str(), "")
            };

        // If the line contains a comment, we parse that first:
        if let Some(comment) = Comment::new(comment_str) {
            debug!(" * parsing comment: {:?}", comment);
            function.statements.push(Statement::Comment(comment));
        }

        // Then we parse the AST statements.

        if let Some(directive) = Directive::new(node_str, target) {
            debug!(" * parsed directive: {:?}", directive);

            // We set the first .file directive we parse as the functions file
            // path:
            if function.file.is_none() {
                if let Some(file) = directive.file() {
                    debug!(" * file directive: {:?}", file);

                    // If there is a function location already set, we set the
                    // file only if the file index matches the location. Many
                    // functions don't have a .file directive set at the
                    // beginnin (only a location) but they contain .file
                    // directives in the body when code from other files gets
                    // inlined:
                    if let Some(ref loc) = &function.loc {
                        if loc.file_index == file.index {
                            function.file = Some(file);
                        }
                    } else {
                        // If no location is set, the .file directive likely
                        // belongs to the function: when a .file directive for
                        // the function is generated these come before the
                        // first .loc directive.
                        function.file = Some(file);
                    }
                }
            }

            // If we find a .loc directive we parse the loc offset and set its
            // value as the current one:
            if let Some(new_loc) = directive.loc() {
                debug!(" * loc directive: {:?}", new_loc);

                current_loc = Some(new_loc);

                // The function location is the first location that we find
                // while parsing the function body:
                if function.loc.is_none() {
                    // If there is a function file already set, we check
                    // that the new location matches the file idx.
                    if let Some(ref file) = &function.file {
                        assert_eq!(new_loc.file_index, file.index);
                    }
                    function.loc = Some(new_loc);
                }
            }
            let dir = Statement::Directive(directive);
            debug!(" * appending directive: {:?}", dir);

            function.statements.push(dir);
            continue;
        }

        if let Some(label) = Label::new(node_str, current_loc) {
            debug!(" * parsed label: {:?}", label);

            function.statements.push(Statement::Label(label));
            continue;
        }

        if let Some(instruction) =
            Instruction::new(node_str, current_loc, target)
        {
            debug!(" * parsed instruction: {:?}", instruction);

            function
                .statements
                .push(Statement::Instruction(instruction));
            continue;
        }

        panic!(
            "cannot parse function: {}\n  line off: {}\n{}",
            path, line_off, line
        );
    }
    function
}

/// Result of parsing a function, either a match, or a table of functions in
/// the file.
pub enum Result {
    Found(ast::Function, ::std::collections::HashMap<usize, ast::File>),
    NotFound(Vec<String>),
}

/// Parses the assembly function at `path` from the file `file`.
#[allow(clippy::use_debug, clippy::cognitive_complexity)]
pub fn function(file: &::std::path::Path, target: &TargetInfo) -> Result {
    use std::{
        collections::HashMap,
        fs::File,
        io::{BufRead, BufReader},
    };

    let path = if let Some(path) = opts.path() {
        path
    } else {
        "".to_owned()
    };

    let fh = File::open(file).unwrap();
    let file_buf = BufReader::new(&fh);

    // We keep here the file ids of the already parsed files:
    let mut file_directive_table = HashMap::<usize, ast::File>::new();

    // This is the AST of the function we are looking for:
    let mut function: Option<ast::Function> = None;

    let mut line_iter = file_buf.lines();

    let mut function_table = Vec::<String>::new();

    // This is the pattern at the beginning of an assembly label
    // that identifies the label as a function:
    let function_label_pattern = {
        if target.is_apple() {
            "__"
        } else {
            "_"
        }
    };

    // This is the pattern that we match to know that we have finished
    // searching the function
    let function_end_pattern = {
        if target.is_windows() {
            ".seh_endproc" // TODO: does this work with panic=abort ?
        } else {
            ".cfi_endproc"
        }
    };

    while let Some(line) = line_iter.next() {
        let line = line.unwrap().trim().to_string();

        if function.is_none() && line.starts_with(function_label_pattern) {
            // We haven't found the function yet:
            //
            // Assembly functions are labels that start with `_` or `__`
            // and have mangled names.
            if let Some(label) = ast::Label::new(&line, None) {
                let demangled_function_name =
                    crate::demangle::demangle(&label.id, target);
                function_table.push(demangled_function_name.clone());
                if demangled_function_name != path {
                    continue;
                }
                // We have found the function, collect its lines and build
                // an AST:
                let mut lines = Vec::<String>::new();
                while let Some(l) = line_iter.next() {
                    let l = l.unwrap().trim().to_string();
                    if l.starts_with(function_end_pattern) {
                        break;
                    }
                    lines.push(l);
                }
                debug!("Function found: {}", path);
                if opts.debug_mode() {
                    for l in &lines {
                        debug!("## {}", l);
                    }
                }

                function = Some(function_body(lines, &path, target));
                // If the function contained a .file directive, we are
                // done:
                if let Some(ref function) = &function {
                    if function.file.is_some() {
                        break;
                    }
                }

                // If the function did not contain a .loc directive
                // either, we can't finde its
                // corresponding Rust code so we are done:
                if let Some(ref function) = &function {
                    if function.loc.is_none() {
                        break;
                    }
                }

                // Otherwise we continue parsing the assembly file to try
                // to find a .file directive for the
                // function
                continue;
            }
            panic!(
                "line starts with _ but we failed to parse the label: {}",
                line
            );
        }

        // If the line does not begin an assembly function try to parse the
        // line as a .file directive.
        if let Some(file) = ast::File::new(&line, target) {
            debug!("found file directive: {:?}", file);
            let idx = file.index;

            // If the file directive is already in the table, check that
            // the paths match:
            if file_directive_table.contains_key(&idx) {
                let f = &file_directive_table[&idx];
                assert_eq!(f.path, file.path);
                continue;
            }

            // The file directive is not in the table: insert it:
            file_directive_table.insert(idx, file);
        }

        // If we have found the function but landed here, the function contains
        // at least one .loc directive but we haven't found its corresponding
        // file yet, so we see if its present in the HashMap:
        if let Some(ref mut function) = function {
            assert!(function.file.is_none());
            assert!(function.loc.is_some());
            let file_index = function.loc.unwrap().file_index;
            if let Some(file) = file_directive_table.remove(&file_index) {
                function.file = Some(file);
                break;
            }
        }
    }

    if function.is_none() {
        // If the function is not found we have visited the whole file so the
        // function table is complete.
        return Result::NotFound(function_table);
    }

    let function = function.unwrap();

    // Add all local .file directives in the body of the function to the table:
    if let Some(ref f) = &function.file {
        file_directive_table
            .entry(f.index)
            .or_insert_with(|| f.clone());
    }
    for s in &function.statements {
        if let Statement::Directive(Directive::File(ref f)) = s {
            file_directive_table
                .entry(f.index)
                .or_insert_with(|| f.clone());
        }
    }

    // Check that we have found all .file directives for all .loc statements
    // within the function:
    let mut done = true;
    for s in &function.statements {
        if let Statement::Directive(Directive::Loc(ref l)) = s {
            if !file_directive_table.contains_key(&l.file_index) {
                done = false;
                error!(
                    "File directive for location not found! Location: {:?}",
                    l
                );
            }
        }
    }

    if done {
        return Result::Found(function, file_directive_table);
    }

    unimplemented!(
        "TODO: need to continue scanning the file for file directives"
    )
}
