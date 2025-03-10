use std::env::args;

pub const HELP: &str = "Basic terminal-based notepad editor

Usage:
    seemply <path>
    seemply [Options]

Options:
    -h, --help: Display this help page";

pub fn get_filename() -> Option<String> {
    let arg = args().nth(1)?;
    is_help(arg)
}

fn is_help(arg: String) -> Option<String> {
    match arg.as_str() {
        "-h" => None,
        "--help" => None,
        _ => Some(arg),
    }
}
