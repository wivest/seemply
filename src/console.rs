use std::{
    io::{Read, Write},
    u8,
};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute, queue,
    style::Print,
    terminal::{self, Clear, EnterAlternateScreen, LeaveAlternateScreen},
};

pub struct Console {
    height: u16,
    scroll: u16,
}

impl Console {
    pub fn new() -> Self {
        terminal::enable_raw_mode().expect("Failed to enable raw mode!");
        execute!(std::io::stdout(), EnterAlternateScreen, Hide)
            .expect("Failed to enter alternate screen!");

        Console {
            height: get_height(),
            scroll: 0,
        }
    }

    pub fn print(&self, content: &String) -> Result<(), std::io::Error> {
        queue!(std::io::stdout(), MoveTo(0, 0))?;

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
            queue!(
                std::io::stdout(),
                Clear(terminal::ClearType::CurrentLine),
                Print(line.to_owned() + "\n")
            )?;
        }
        std::io::stdout().flush()?;
        Ok(())
    }

    pub fn scroll_up(&mut self, by: u16) {
        if self.scroll <= by {
            self.scroll = 0;
        } else {
            self.scroll -= by;
        }
    }

    pub fn scroll_down(&mut self, by: u16) {
        self.scroll += by;
    }

    pub fn ask_command(&self) -> u8 {
        let mut buf: [u8; 1] = [0; 1];
        std::io::stdin()
            .read_exact(&mut buf)
            .expect("Unable to read command!");
        buf[0]
    }
}

impl Drop for Console {
    fn drop(&mut self) {
        execute!(std::io::stdout(), LeaveAlternateScreen, Show)
            .expect("Failed to exit alternate screen!");
        terminal::disable_raw_mode().expect("Failed to disable raw mode!");
    }
}

fn get_height() -> u16 {
    let size = terminal::size().expect("Failed to get terminal size!");
    size.1
}
