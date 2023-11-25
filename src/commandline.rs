const HELP_TEXT: &str = "
ThinkMenu: A simple TUI programm to change common Settings on Lenovo Laptops.

Usage:
    thinkmenu                   Run Menu and asks to save changes when exiting
    sudo thinkmenu              Make runtime changes and apply them when exiting

    thinkmenu -h, --help        Show this menu
";

pub fn print_help() {
    println!("{}", HELP_TEXT)
}

pub fn print_unknown() {
    println!("Unknown argument, use `--help` to see the help page")
}