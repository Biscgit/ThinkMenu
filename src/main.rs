mod application;
mod commandline;

use std::env;
use std::process::ExitCode;

use commandline::{print_help, print_unknown};
use application::run_application;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(String::as_str) {
        // help menu
        Some("-h" | "--help") => print_help(),
        Some(_) => print_unknown(),

        // normal program run
        None => {
            // check for sudo privileges
            let mut is_root: bool = false;

            if let Some(user) = env::var_os("USER") {
                if user == "root" {
                    is_root = true;
                }
            }

            return run_application(is_root);
        }
    }

    return ExitCode::from(0);
}
