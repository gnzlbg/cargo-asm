use super::asm;
use super::options;

fn format_function_name(function: &asm::ast::Function) -> String {
    if function.file.is_some() && function.loc.is_some() {
        if let &Some(ref file) = &function.file {
            if let &Some(ref loc) = &function.loc {
                return format!(
                    "{} ({}:{})",
                    function.id, file.path, loc.offset
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

pub fn print(function: asm::ast::Function, mut rust: Vec<(usize, String)>, opts: &options::Options) {
    println!("{}:", format_function_name(&function));

    if !opts.rust {
        for stmt in function.statements {
            if stmt.should_print(&opts) {
                println!("{}", stmt.format(&opts));
            }
        }
    } else {
        rust.pop();
        rust.reverse();
        let mut rust_line = 1;
        for (stmt_idx, stmt) in function.statements.iter().enumerate() {
            if stmt.should_print(&opts) {
                let off = stmt.rust_loc(&function.file.as_ref().map(|v| v.clone()).unwrap());
                if let Some(off) = off {
                    while let Some(r) = rust.last().map(|v| v.clone()) {
                        if r.0 <= off {
                            if opts.verbose {
                                println!("{} | rloc: {}", r.1, r.0);
                            } else {
                                println!("{}", r.1);
                            }
                            rust.pop();
                            rust_line += 1;
                        }
                        break;
                    }
                } else {
                    // Do a look ahead and check if the next location
                    // corresponds to the next Rust line. If not, print now the
                    // Rust lines until the next location:
                    let (_, tail) = function.statements.split_at(stmt_idx);

                    let n = tail.iter().find(|v| {
                        v.rust_loc(
                            &function.file.as_ref().map(|v| v.clone()).unwrap())
                            .is_some()
                    });

                    if let Some(n) = n {
                        println!("HERE {:?}", n);
                        use asm::ast::{Statement, Directive};
                        match n {
                            &Statement::Directive(Directive::Loc(ref n)) => {
                        if n.offset > rust_line {
                            while rust_line != n.offset && !rust.is_empty() {
                                while let Some(r) = rust.last().map(|v| v.clone()) {
                                if opts.verbose {
                                    println!("{} | rloc: {}", r.1, r.0);
                                } else {
                                    println!("{}", r.1);
                                }
                                rust.pop();
                                    rust_line += 1;
                                }
                            }
                        }
                            }
                            _ => {},
                        }
                    } else {
                        while !rust.is_empty() {
                            while let Some(r) = rust.last().map(|v| v.clone()) {
                            if opts.verbose {
                                println!("{} | rloc: {}", r.1, r.0);
                            } else {
                                println!("{}", r.1);
                            }
                            rust.pop();
                                rust_line += 1;
                            }
                        }
                    }
                }
                println!("{}", stmt.format(&opts));
            }
        }
        rust.reverse();
        for r in rust {
            println!("{}", r.1);
        }
        println!("}}");
    }
}
