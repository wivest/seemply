use console::{Console, State};
use crossterm::event::{Event, KeyCode};
use std::fs;

mod args;
mod console;

fn main() {
    let filename = args::get_filename();
    let content = get_content(&filename);

    let mut con = Console::new(content).expect("Failed to initialize console!");
    con.print().expect("Failed to print content!");

    loop {
        let event = Console::ask_command().expect("Failed to ask command!");
        if let Event::Key(key) = event {
            if key.code == KeyCode::Char('q') {
                break;
            }
            if key.code == KeyCode::Char('w') {
                let delta = con.cursor.up(1);
                if delta != 0 {
                    con.scroll_up(delta);
                }
                let line = con.get_line_width();
                con.cursor.right(0, line);
            }
            if key.code == KeyCode::Char('s') {
                let delta = con.cursor.down(1);
                if delta != 0 {
                    con.scroll_down(delta);
                }
                let line = con.get_line_width();
                con.cursor.right(0, line);
            }
            if key.code == KeyCode::Char('a') {
                con.cursor.left(1);
            }
            if key.code == KeyCode::Char('d') {
                let line = con.get_line_width();
                con.cursor.right(1, line);
            }
            if key.code == KeyCode::Char('i') {
                con.state = State::Input;
            }
            if key.code == KeyCode::Esc {
                con.state = State::Control;
            }
        }
        con.print().expect("Failed to print content!");
    }
}

fn get_content(filename: &String) -> String {
    fs::read_to_string(filename).expect("File at specified path doesn't exist!")
}
