use console::Console;
use std::fs;

mod args;
mod console;

fn main() {
    let filename = args::get_filename();
    let content = get_content(&filename);

    let mut con = Console::new(content).expect("Failed to initialize console!");
    con.print().expect("Failed to print content!");

    loop {
        let command = Console::ask_command().expect("Failed to ask command!");
        if command == b'q' {
            break;
        }
        if command == b'w' {
            let delta = con.cursor.up(1).expect("Failed to move cursor!");
            if delta != 0 {
                con.scroll_up(delta);
            }
            let line = con.get_line_width();
            con.cursor.right(0, line).expect("Failed to move cursor!");
        }
        if command == b's' {
            let delta = con.cursor.down(1).expect("Failed to move cursor!");
            if delta != 0 {
                con.scroll_down(delta);
            }
            let line = con.get_line_width();
            con.cursor.right(0, line).expect("Failed to move cursor!");
        }
        if command == b'a' {
            con.cursor.left(1).expect("Failed to move cursor!");
        }
        if command == b'd' {
            let line = con.get_line_width();
            con.cursor.right(1, line).expect("Failed to move cursor!");
        }
        con.print().expect("Failed to print content!");
    }
}

fn get_content(filename: &String) -> String {
    fs::read_to_string(filename).expect("File at specified path doesn't exist!")
}
