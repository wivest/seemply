use std::io::Write;

use crossterm::{
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};

pub struct Console {
    height: u16,
}

impl Console {
    pub fn new() -> Self {
        execute!(std::io::stdout(), EnterAlternateScreen)
            .expect("Failed to enter alternate screen!");

        Console {
            height: get_height(),
        }
    }

    pub fn print(&self, content: &String) {
        println!("\x1B[?1049h");
        print!("\x1B[H");
        std::io::stdout().flush().expect("Failed to move cursor!");

        let lines = content.lines();
        let mut i = 1;
        for line in lines {
            if i >= self.height {
                break;
            }
            println!("{line}");
            i += 1;
        }
    }

    pub fn ask_command(&self) -> String {
        print!("\x1B[{};1H", self.height);
        std::io::stdout()
            .flush()
            .expect("Failed to put cursor to bottom!");

        let mut command = String::new();
        std::io::stdin()
            .read_line(&mut command)
            .expect("Unable to read command!");
        command.trim().to_owned()
    }
}

impl Drop for Console {
    fn drop(&mut self) {
        execute!(std::io::stdout(), LeaveAlternateScreen)
            .expect("Failed to exit alternate screen!");
    }
}

fn get_height() -> u16 {
    let size = terminal::size().expect("Failed to get terminal size!");
    size.1
}
