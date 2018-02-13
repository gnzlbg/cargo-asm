//! Process utilities

use std::process::Command;

/// Executes the command printing `error_msg` and forwarding `stdout` and
/// `stderr` on failure to `stderr`.
///
/// If `verbose` is `true` `stdout` and `stderr` are forwarded to `stderr` on
/// success as well.
pub fn exec(
    cmd: &mut Command, error_msg: &str, verbose: bool
) -> Result<(String, String), ()> {
    let r = cmd.output().expect(error_msg);
    let success = r.status.success();
    let stdout = String::from_utf8(r.stdout).expect("Not UTF-8");
    let stderr = String::from_utf8(r.stderr).expect("Not UTF-8");
    if !success || verbose {
        if !success {
            eprintln!("\n[ERROR]: {}!\n", error_msg);
        }
        if !stderr.is_empty() {
            eprintln!("stderr:\n\n{}\n\n", stderr);
        }
        if !stdout.is_empty() {
            eprintln!("stdout:\n\n{}\n\n", stdout);
        }
        if !success {
            return Err(());
        }
    }
    Ok((stdout, stderr))
}
