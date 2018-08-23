//! cargo-asm driver
#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(
    feature = "cargo-clippy",
    allow(
        missing_docs_in_private_items,
        option_unwrap_used,
        result_unwrap_used
    )
)]

extern crate edit_distance;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate rustc_demangle;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate structopt;
extern crate parking_lot;
extern crate termcolor;
extern crate walkdir;

mod asm;
mod build;
mod demangle;
mod display;
mod llvmir;
mod logger;
mod options;
mod path;
mod process;
mod rust;
mod target;

use options::*;

#[cfg_attr(feature = "cargo-clippy", allow(print_stdout, use_debug))]
fn main() {
    #[cfg(feature = "deadlock_detection")]
    {
        // Create a background thread which checks for deadlocks every 10s
        std::thread::spawn(move || loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
            let deadlocks = parking_lot::deadlock::check_deadlock();
            if deadlocks.is_empty() {
                continue;
            }
            println!("{} deadlocks detected", deadlocks.len());
            for (i, threads) in deadlocks.iter().enumerate() {
                println!("Deadlock #{}", i);
                for t in threads {
                    println!("Thread Id {:#?}", t.thread_id());
                    println!("{:#?}", t.backtrace());
                }
            }
            ::std::process::abort();
        });
    }

    // Initialize logger and options:
    if let Err(err) = logger::Logger::init() {
        eprintln!("failed to initialize logger: {}", err);
        ::std::process::exit(1);
    }

    if opts.debug_mode() {
        log::set_max_level(log::LevelFilter::Debug);
    } else {
        log::set_max_level(log::LevelFilter::Error);
    }

    debug!("Options: {:?}", *opts);

    // Executing cargo asm into a different path via --manifest-path=... is
    // done by changing the current working directory.
    if let Some(ref new_path) = opts.manifest_path() {
        if !new_path.exists() {
            error!("The manifest-path {} does not exist!", new_path.display());
            ::std::process::exit(1);
        }
        let result = ::std::env::set_current_dir(&new_path);
        if !result.is_ok() {
            error!(
                "failed to change the working path to {}",
                new_path.display()
            );
            ::std::process::exit(1);
        }
        debug!("manifest path changed to {}", new_path.display());
    }

    // Builds the project and returns a list of all relevant assembly files:
    let files = build::project();

    if files.is_empty() {
        display::write_error("cargo asm could not find any output files!");
        ::std::process::exit(1);
    }

    debug!("Found following files:");
    for f in &files {
        debug!("  {}", f.display());
    }
    let o = { (*opts.read()).clone() };
    match o {
        ::options::Options::Asm(_) => asm::run(&files),
        ::options::Options::LlvmIr(_) => llvmir::run(&files),
    }
}
