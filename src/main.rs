use console::Console;
use cursor::Cursor;
use std::fs;

mod args;
mod console;
mod cursor;

fn main() {
    let filename = args::get_filename();
    let content = get_content(&filename);

    let mut con = Console::new(content).expect("Failed to initialize console!");
    con.print().expect("Failed to print content!");

    let cursor = Cursor::new(&mut con);

    loop {
        let command = con.ask_command().expect("Failed to ask command!");
        if command == b'q' {
            break;
        }
        if command == b'w' {
            con.scroll_up(1);
        }
        if command == b's' {
            con.scroll_down(1);
        }
        con.print().expect("Failed to print content!");
    }
}

fn get_content(filename: &String) -> String {
    fs::read_to_string(filename).expect("File at specified path doesn't exist!")
}
