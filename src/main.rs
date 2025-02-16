use console::Console;
use std::fs;

mod args;
mod console;

fn main() {
    let filename = args::get_filename();
    let content = get_content(&filename);

    let mut con = Console::new();
    con.print(&content).expect("Failed to print content!");

    loop {
        let command = con.ask_command();
        if command == b'q' {
            break;
        }
        if command == b'w' {
            con.scroll_up(1);
        }
        if command == b's' {
            con.scroll_down(1);
        }
        con.print(&content).expect("Failed to print content!");
    }
}

fn get_content(filename: &String) -> String {
    fs::read_to_string(filename).expect("File at specified path doesn't exist!")
}
