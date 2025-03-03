use console::Console;
use crossterm::event::{Event, KeyEventKind};
use std::fs;

mod args;
mod console;

fn main() {
    let filename = args::get_filename();
    let content = get_content(&filename);

    let mut con = Console::new(content).expect("Failed to initialize console!");
    con.print().expect("Failed to print content!");

    loop {
        let event = Console::ask_command().expect("Failed to read command!");
        if let Event::Key(key) = event {
            if key.kind != KeyEventKind::Release {
                if !con.state.handle_input(key.code, &mut con) {
                    break;
                }
            }
        }
        con.print().expect("Failed to print content!");
    }
}

fn get_content(filename: &String) -> String {
    fs::read_to_string(filename).expect("File at specified path doesn't exist!")
}
