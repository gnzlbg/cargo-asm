use super::asm;
use super::options;
use super::rust;

fn format_function_name(function: &asm::ast::Function) -> String {
    if function.file.is_some() && function.loc.is_some() {
        if let &Some(ref file) = &function.file {
            if let &Some(ref loc) = &function.loc {
                return format!(
                    "{} ({}:{})",
                    function.id, file.path, loc.file_line
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

pub fn print_asm(function: asm::ast::Function, opts: &options::Options) {
    for stmt in function.statements {
        if stmt.should_print(&opts) {
            println!("{}", stmt.format(&opts));
        }
    }
}

pub fn print_rust(
    function: asm::ast::Function, mut rust: rust::Files, opts: &options::Options
) {
    println!("{}:", format_function_name(&function));

    let mut last_locs =
        ::std::collections::HashMap::<usize, asm::ast::Loc>::new();

    // Find the last location of the function.
    let mut last_loc: Option<asm::ast::Loc> = None;
    for stmt in function.statements.iter() {
        if let Some(new_loc) = stmt.rust_loc() {
            if let Some(old_loc) = last_loc {
                if old_loc.file_index == new_loc.file_index
                    && old_loc.file_line < new_loc.file_line
                {
                    last_loc = Some(new_loc);
                }
            } else {
                last_loc = Some(new_loc);
            }
        }
    }

    // Trim std lib paths:
    for (_k, f) in &mut rust.files {
        let ast = f.ast.clone();
        if !ast.path.contains("/lib/rustlib/src/rust/src/") { continue; }
        let new_path = ast.path.split("/lib/rustlib/src/rust/src/").nth(1).unwrap();
        f.ast.path = new_path.to_string();
    }

    for (stmt_idx, stmt) in function.statements.iter().enumerate() {
        if let Some(new_loc) = stmt.rust_loc() {
            if let Some(old_loc) = last_locs.get(&new_loc.file_index) {
                for line_idx in old_loc.file_line + 1..new_loc.file_line + 1 {
                    if let Some(l) = rust.line_at(new_loc.file_index, line_idx)
                    {
                        if last_loc.is_none()
                            || !(last_loc.unwrap().file_index
                                == new_loc.file_index
                                && last_loc.unwrap().file_line == line_idx)
                        {
                            if new_loc.file_index != function.loc.unwrap().file_index {
                                println!("{} ({}:{})", l, rust.file_path(new_loc).unwrap(), new_loc.file_line);
                            } else {
                                println!("{}", l);
                            }
                        }
                    }
                }
            } else {
                if let Some(l) = rust.line(new_loc) {
                    if last_loc.is_none()
                        || !(last_loc.unwrap().file_index == new_loc.file_index
                            && last_loc.unwrap().file_line
                                == new_loc.file_line)
                    {
                        if new_loc.file_index != function.loc.unwrap().file_index {
                            println!("{} ({}:{})", l, rust.file_path(new_loc).unwrap(), new_loc.file_line);
                        } else {
                            println!("{}", l);
                        }
                    }
                }
            }
            if new_loc.file_line > 0 {
                last_locs.insert(new_loc.file_index, new_loc);
            }

            if let Some(func_loc) = function.loc {
                if func_loc.file_index == new_loc.file_index {
                    let stmt_tail =
                        function.statements.split_at(stmt_idx+1).1;
                    for s in stmt_tail {
                        if let Some(s) = s.rust_loc() {
                            if s.file_index != new_loc.file_index {
                                continue;
                            }
                            if s.file_line == new_loc.file_line + 1 {
                                break;
                            }
                            if let Some(old_loc) = last_locs.get(&new_loc.file_index) {
                                if old_loc.file_line > new_loc.file_line {
                                    break;
                                }
                            }

                            for l in new_loc.file_line + 1..s.file_line + 1 {
                                if let Some(line) =
                                    rust.line_at(s.file_index, l)
                                {
                                    if last_loc.is_none()
                                        || !(last_loc.unwrap().file_index
                                            == new_loc.file_index
                                            && last_loc.unwrap().file_line
                                                == l)
                                    {
                                        println!("{}", line);
                                    }
                                    last_locs.insert(
                                        new_loc.file_index,
                                        asm::ast::Loc {
                                            file_index: new_loc.file_index,
                                            file_line: l,
                                            file_column: 0,
                                        },
                                    );
                                }
                            }
                            break;
                        }
                    }
                }
            }
        }

        if stmt.should_print(&opts) {
            println!("{}", stmt.format(&opts));
        }
    }

    // At the end prin the Rust code of the last location.
    if let Some(last_loc) = last_loc {
        if let Some(l) = rust.line(last_loc) {
            println!("{}", l);
        }
    }
}
