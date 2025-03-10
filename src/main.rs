use console::Console;
use crossterm::event::{Event, KeyEventKind};

mod args;
mod console;
mod content;

fn main() {
    let path = args::get_filename();

    if let Ok(mut con) = Console::new(&path) {
        con.print().expect("Failed to print content!");
        while handle_input(&mut con) {
            con.print().expect("Failed to print content!");
        }
    } else {
        panic!("Failed to initialize console!");
    }
}

fn handle_input(con: &mut Console) -> bool {
    let event = Console::get_event().expect("Failed to read event!");
    if let Event::Key(key) = event {
        if key.kind != KeyEventKind::Release {
            return con.state.handle_input(key.code, con);
        }
    }
    true
}
