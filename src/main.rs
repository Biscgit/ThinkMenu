use std::env;
use std::process::ExitCode;


fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(String::as_str) {
        // help menu
        Some("-h" | "--help") => println!("Help menu!"),
        Some(_) => println!("Unknown argument, use `--help` to see the help page"),

        // normal program run
        None => {

        }
    }

    return ExitCode::from(0);
}
