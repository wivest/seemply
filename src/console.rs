use std::io::Read;

use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{self, Clear, EnterAlternateScreen, LeaveAlternateScreen},
};

pub struct Console {
    height: u16,
    pub scroll: u16,
}

impl Console {
    pub fn new() -> Self {
        terminal::enable_raw_mode().expect("Failed to enable raw mode!");
        execute!(std::io::stdout(), EnterAlternateScreen)
            .expect("Failed to enter alternate screen!");

        Console {
            height: get_height(),
            scroll: 0,
        }
    }

    pub fn print(&self, content: &String) {
        execute!(
            std::io::stdout(),
            Clear(terminal::ClearType::All),
            MoveTo(0, 0)
        )
        .expect("Failed to move cursor!");

        let lines = content.lines();
        let mut i = 0;
        for line in lines {
            i += 1;
            if i <= self.scroll {
                continue;
            }
            if i >= self.height + self.scroll {
                break;
            }
            println!("{line}");
        }
    }

    pub fn ask_command(&self) -> String {
        execute!(std::io::stdout(), MoveTo(0, self.height - 1)).expect("Failed to move cursor!");

        let mut command = String::new();
        std::io::stdin()
            .read_to_string(&mut command)
            .expect("Unable to read command!");
        command.trim().to_owned()
    }
}

impl Drop for Console {
    fn drop(&mut self) {
        execute!(std::io::stdout(), LeaveAlternateScreen)
            .expect("Failed to exit alternate screen!");
        terminal::disable_raw_mode().expect("Failed to disable raw mode!");
    }
}

fn get_height() -> u16 {
    let size = terminal::size().expect("Failed to get terminal size!");
    size.1
}
