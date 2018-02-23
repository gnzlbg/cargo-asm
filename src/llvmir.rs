use ::options::*;

pub fn run(files: &[::std::path::PathBuf]) {
    let mut function_table: Option<Vec<String>> = None;
    for f in files {
        assert!(f.exists(), "path does not exist: {}", f.display());
        let r = print_function(f);
        if r.is_none() {
            function_table = None;
            break;
        }
        let mut r = r.unwrap();
        if let Some(ref mut function_table) = function_table {
            function_table.append(&mut r);
        } else {
            function_table = Some(r);
        }
    }

    if function_table.is_none() { return; }
    let mut function_table = function_table.unwrap();

    use edit_distance::edit_distance;
    let mut msg = format!("could not find function at path \"{}\" in the generated LLVM IR.\n", &opts.path());

    let last_path = opts.path();
    let last_path = last_path.split(':').next_back().unwrap();
    function_table.sort_by(|a, b| {
        edit_distance(a.split(':').next_back().unwrap(), last_path)
            .cmp(&edit_distance(
                b.split(':').next_back().unwrap(),
                last_path,
            ))
    });
    
    for (i, f) in function_table
        .iter()
        .take_while(|f| {
            edit_distance(f.split(':').next_back().unwrap(), last_path)
                        <= 4
                })
                .enumerate()
            {
                if i == 0 {
                    msg.push_str(&format!(
                        "Is it one of the following functions?\n\n"
                    ));
                }
                msg.push_str(&format!("  {}\n", f));
            }

            msg.push_str(r#"
Tips:
  * make sure that the function is present in the final binary (e.g. if it's a generic function, make sure that it is actually monomorphized)

"#
            );

    ::display::write_error(&msg);
    ::std::process::exit(1);
}

fn print_function(file_name: &::std::path::PathBuf) -> Option<Vec<String>> {
    use std::io::BufRead;

    let path = opts.path();
    let fh = ::std::fs::File::open(file_name).unwrap();
    let file_buf = ::std::io::BufReader::new(&fh);

    let mut line_iter = file_buf.lines();
    let mut function_names: Vec<String> = Vec::new();
    let mut function_lines: Option<Vec<String>> = None;
    while let Some(line) = line_iter.next() {
        let line = line.unwrap().trim().to_string();

        if let Some(ref mut function_lines) = function_lines {
            if line.starts_with("define") {
                break;
            }

            function_lines.push(line);
        } else {
            if !line.starts_with("define") {
                continue;
            }

            let first = line.find("@").unwrap();
            let last = line.find("(").unwrap();
            let mangled_name = &line[first+1..last];
            let demangled_name = ::demangle::demangle(&mangled_name);
            if demangled_name != path {
                function_names.push(demangled_name);
                continue;
            }
            function_lines = Some(vec![line.clone()]);
            continue;
        }
    }

    if let Some(function_lines) = function_lines {
        // Find last }
        let r = function_lines.iter().rposition(|s| s.trim() == "}");
        let r = r.unwrap_or(function_lines.len()-1);
        for l in &function_lines[0..r+1] {
            println!("{}", l);
        }
        None
    } else {
        function_names.sort();
        function_names.dedup();
        Some(function_names)
    }
}
