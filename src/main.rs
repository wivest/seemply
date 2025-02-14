use console::Console;
use std::fs;

mod args;
mod console;

fn main() {
    let filename = args::get_filename();
    let content = get_content(&filename);

    let mut con = Console::new();
    con.print(&content);

    loop {
        let command = con.ask_command();
        if command == "q" {
            break;
        }
        if command == "w" {
            con.scroll -= 1;
        }
        if command == "s" {
            con.scroll += 1;
        }
        con.print(&content);
    }
}

fn get_content(filename: &String) -> String {
    fs::read_to_string(filename).expect("File at specified path doesn't exist!")
}
