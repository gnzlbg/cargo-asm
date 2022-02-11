use crate::options::*;
use crate::target::TargetInfo;

use log::debug;

pub fn run(files: &[::std::path::PathBuf], target: &TargetInfo) {
    let mut function_table: Option<Vec<String>> = None;

    for f in files {
        debug!("Scanning file: {:?}", f);
        assert!(f.exists(), "path does not exist: {}", f.display());
        let r = print_function(f, target);

        if r.is_ok() {
            debug!("Function found, we are done!");
            function_table = None;
            break;
        }
        debug!("Function not found, appending all function names in the file to the table...");
        let mut r = r.unwrap_err();
        if let Some(ref mut function_table) = function_table {
            function_table.append(&mut r);
        } else {
            function_table = Some(r);
        }
    }

    if function_table.is_none() {
        debug!("Function found!");
        return;
    }

    debug!("Function not found. Showing functions in the table...");
    let mut function_table = function_table.unwrap();

    match opts.path() {
        None => {
            for f in function_table {
                println!("{}", f);
            }
        }
        Some(path) => {
            use edit_distance::edit_distance;
            let mut msg = format!(
                "could not find function at path \"{}\" in the generated LLVM IR.\n",
                &path
            );

            let last_path = path;
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
                    msg.push_str("Is it one of the following functions?\n\n");
                }
                msg.push_str(&format!("  {}\n", f));
            }

            msg.push_str(r#"
Tips:
* make sure that the function is present in the final binary (e.g. if it's a generic function, make sure that it is actually monomorphized)

"#
                    );

            crate::display::write_error(&msg);
            ::std::process::exit(1);
        }
    }
}

fn print_function(
    file_name: &::std::path::PathBuf,
    target: &TargetInfo,
) -> Result<(), Vec<String>> {
    use std::io::BufRead;

    let path = if let Some(path) = opts.path() {
        path
    } else {
        "".to_owned()
    };
    let fh = ::std::fs::File::open(file_name).unwrap();
    let file_buf = ::std::io::BufReader::new(&fh);

    let line_iter = file_buf.lines();
    let mut function_names: Vec<String> = Vec::new();
    let mut function_lines: Option<Vec<String>> = None;
    for line in line_iter {
        let line = line.unwrap().trim().to_string();

        if let Some(ref mut function_lines) = function_lines {
            if line.starts_with("define") {
                debug!("End of function");
                break;
            }

            debug!("    {:?}", line);
            function_lines.push(line);
        } else {
            if !line.starts_with("define") {
                continue;
            }

            let first = line.find('@').unwrap();
            let last = line[first..].find('(').unwrap() + first;
            assert!(
                first < last,
                "first: {:?}, last: {:?}, line:\n{:?}",
                first,
                last,
                line
            );
            let mangled_name = &line[first + 1..last];
            let demangled_name =
                crate::demangle::demangle(mangled_name, target);
            if demangled_name != path {
                function_names.push(demangled_name);
                continue;
            }
            function_lines =
                Some(vec![line.replace(mangled_name, &demangled_name)]);
            debug!("Found function with path: {:?}", path);
            continue;
        }
    }

    if let Some(function_lines) = function_lines {
        debug!("Function found! Displaying function...");
        // Find last }
        let r = function_lines.iter().rposition(|s| s.trim() == "}");
        let r = r.unwrap_or(function_lines.len() - 1);
        for line in &function_lines[0..r + 1] {
            let mut demangled_line = String::new();
            let mut start = 0;
            while let Some(f) = &line[start..].find('"') {
                if start == 0 {
                    debug!("line to demangle: {}", line);
                }
                debug!("s: {}, f: {}, dl: {}", start, f, demangled_line);
                let f = f + start + 1;
                let l = line[f..].find('"').unwrap() + f;
                let mangled_name = &line[f..l];
                let demangled_name = if mangled_name.ends_with(".exit") {
                    let mut v = crate::demangle::demangle(
                        &mangled_name[0..mangled_name.len() - 5],
                        target,
                    );
                    v += ".exit";
                    v
                } else {
                    crate::demangle::demangle(mangled_name, target)
                };
                debug!(
                    "  f: {}, l: {}, mn: {}, dm: {}",
                    f, l, mangled_name, demangled_name
                );
                demangled_line += &line[start..f];
                demangled_line += &demangled_name;
                demangled_line.push('"');
                start = l + 1;
                debug!("  ns: {}, ndl: {}", start, demangled_line);
            }
            if demangled_line.is_empty() {
                println!("{}", line);
            } else {
                println!("{}", demangled_line);
            }
        }
        Ok(())
    } else {
        function_names.sort();
        function_names.dedup();
        Err(function_names)
    }
}
