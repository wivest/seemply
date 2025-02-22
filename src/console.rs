use std::{
    io::{Error, Read, Write},
    u8,
};

use crossterm::{
    cursor::{Hide, MoveTo, MoveToRow, Show},
    execute, queue,
    style::Print,
    terminal::{self, Clear, EnterAlternateScreen, LeaveAlternateScreen},
};

pub struct Console {
    height: u16,
    cursor: (u16, u16),
    content: String,
    scroll: u16,
}

impl Console {
    pub fn new(content: String) -> Result<Self, Error> {
        terminal::enable_raw_mode()?;
        execute!(std::io::stdout(), EnterAlternateScreen, Hide)?;
        let size = terminal::size()?;

        Ok(Console {
            height: size.1,
            cursor: (0, 0),
            content,
            scroll: 0,
        })
    }

    pub fn print(&self) -> Result<(), Error> {
        queue!(std::io::stdout(), MoveTo(0, 0))?;

        let lines = self.content.lines();
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

    pub fn cursor_up(&mut self, by: u16) -> Result<(), Error> {
        let (actual, delta) = if self.cursor.1 <= by {
            (0, by - self.cursor.1)
        } else {
            (self.cursor.1 - by, 0)
        };

        execute!(std::io::stdout(), MoveToRow(actual))?;
        self.scroll_up(delta);
        self.cursor.1 = actual;

        Ok(())
    }

    pub fn cursor_down(&mut self, by: u16) -> Result<(), Error> {
        let calc = self.cursor.1 + by;
        let actual = if calc >= self.height {
            self.height - 1
        } else {
            calc
        };
        let delta = calc - actual;

        execute!(std::io::stdout(), MoveToRow(actual))?;
        self.scroll_down(delta);
        self.cursor.1 = actual;

        Ok(())
    }

    fn scroll_up(&mut self, by: u16) {
        if self.scroll <= by {
            self.scroll = 0;
        } else {
            self.scroll -= by;
        }
    }

    fn scroll_down(&mut self, by: u16) {
        self.scroll += by;
    }

    pub fn ask_command() -> Result<u8, Error> {
        let mut buf: [u8; 1] = [0; 1];
        std::io::stdin().read_exact(&mut buf)?;
        Ok(buf[0])
    }
}

impl Drop for Console {
    fn drop(&mut self) {
        execute!(std::io::stdout(), LeaveAlternateScreen, Show)
            .expect("Failed to exit alternate screen!");
        terminal::disable_raw_mode().expect("Failed to disable raw mode!");
    }
}
