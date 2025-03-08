use console::Console;
use crossterm::event::{Event, KeyEventKind};

mod args;
mod console;
mod content;

fn main() {
    let path = args::get_filename();

    let mut con = Console::new(&path).expect("Failed to initialize console!");
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
