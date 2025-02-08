use console::Console;
use std::fs;

mod args;
mod console;

fn main() {
    let filename = args::get_filename();
    let content = get_content(&filename);

    let con = Console::new();
    con.print(&content);

    loop {
        let command = con.ask_command();
        if command == "q" {
            break;
        }
        con.print(&content);
    }
}

fn get_content(filename: &String) -> String {
    fs::read_to_string(filename).expect("File at specified path doesn't exist!")
}
