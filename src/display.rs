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

pub fn print(function: asm::ast::Function, opts: &options::Options) {
    println!("{}:", format_function_name(&function));

    if !opts.rust {
        for stmt in function.statements {
            if stmt.should_print(&opts) {
                println!("{}", stmt.format(&opts));
            }
        }
    }
}
