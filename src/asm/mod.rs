pub mod ast;
pub mod parse;
use crate::options::*;
use crate::target::TargetInfo;
use log::{debug, error};

#[derive(Copy, Clone, Debug)]
#[allow(clippy::upper_case_acronyms)]
pub enum Style {
    Intel,
    ATT,
}

impl ::std::str::FromStr for Style {
    type Err = String;
    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        match s {
            "intel" => Ok(Style::Intel),
            "att" => Ok(Style::ATT),
            v => Err(format!("\"{}\" is not a valid assembly style. Try \"intel\" or \"att\"", v))
        }
    }
}

fn parse_files(
    files: &[::std::path::PathBuf],
    target: &TargetInfo,
) -> parse::Result {
    use self::parse::Result;
    use std::io::BufRead;
    if opts.debug_mode() {
        // In debug mode dump all the raw assembly that we could find.
        for f in files {
            debug!("raw file dump {}:", f.display());
            let fh = ::std::fs::File::open(f).unwrap();
            let file_buf = ::std::io::BufReader::new(&fh);
            for l in file_buf.lines() {
                debug!("{}", l.unwrap());
            }
        }
    }
    let mut function_table = Vec::<String>::new();
    for f in files {
        assert!(f.exists(), "path does not exist: {}", f.display());
        match self::parse::function(f.as_path(), target) {
            Result::Found(function, files) => {
                return Result::Found(function, files)
            }
            Result::NotFound(table) => {
                for f in table {
                    function_table.push(f);
                }
            }
        }
    }
    function_table.sort();
    function_table.dedup();
    Result::NotFound(function_table)
}

pub fn run(files: &[::std::path::PathBuf], target: &TargetInfo) {
    // Parse the files
    match parse_files(files, target) {
        self::parse::Result::Found(mut function, file_table) => {
            // If we found the assembly for the path, we parse the assembly:
            let rust = crate::rust::parse(&function, &file_table);

            if opts.json() || opts.debug_mode() {
                if let Some(s) = crate::display::to_json(&function, &rust) {
                    println!("{}", s);
                } else {
                    error!("failed to emit json output");
                }
            }

            if !opts.json() {
                crate::display::print(&mut function, rust, target);
            }
        }
        self::parse::Result::NotFound(mut table) => match opts.path() {
            None => {
                for f in table {
                    println!("{}", f);
                }
            }
            Some(path) => {
                use edit_distance::edit_distance;
                let mut msg = format!("could not find function at path \"{}\" in the generated assembly.\n", &path);

                let last_path = path;
                let last_path = last_path.split(':').next_back().unwrap();
                table.sort_by(|a, b| {
                    edit_distance(a.split(':').next_back().unwrap(), last_path)
                        .cmp(&edit_distance(
                            b.split(':').next_back().unwrap(),
                            last_path,
                        ))
                });

                for (i, f) in table
                    .iter()
                    .take_while(|f| {
                        edit_distance(
                            f.split(':').next_back().unwrap(),
                            last_path,
                        ) <= 4
                    })
                    .enumerate()
                {
                    if i == 0 {
                        msg.push_str(
                            "Is it one of the following functions?\n\n",
                        );
                    }
                    msg.push_str(&format!("  {}\n", f));
                }

                msg.push_str(r#"
Tips:
* make sure that the function is present in the final binary (e.g. if it's a generic function, make sure that it is actually monomorphized)
* try to do a --clean build (sometimes changes are not picked up)
"#
                    );

                crate::display::write_error(&msg);
                ::std::process::exit(1);
            }
        },
    }
}
