mod terminal;
mod filesystem;

use tokio::io::Result;

#[tokio::main]
async fn main() -> Result<()> {
    Ok(())
}

fn old_main() {
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

            run_application(is_root)
                .expect("Unhandled critical error encountered. Exiting...");
        }
    }
}
