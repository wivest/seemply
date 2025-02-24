use std::{
    io::{stdout, Error, Read, Write},
    u8,
};

use crossterm::{
    cursor::{Hide, MoveTo, MoveToColumn, MoveToRow, Show},
    execute, queue,
    style::Print,
    terminal::{self, Clear, EnterAlternateScreen, LeaveAlternateScreen},
};

pub struct Console {
    height: u16,
    cursor: (u16, u16),
    content: Vec<String>,
    scroll: u16,
}

impl Console {
    pub fn new(content: String) -> Result<Self, Error> {
        terminal::enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen)?;
        let size = terminal::size()?;

        Ok(Console {
            height: size.1,
            cursor: (0, 0),
            content: content.lines().map(|l| l.to_owned()).collect(),
            scroll: 0,
        })
    }

    pub fn print(&self) -> Result<(), Error> {
        queue!(stdout(), Hide, MoveTo(0, 0))?;

        let mut i = 0;
        for line in &self.content {
            i += 1;
            if i <= self.scroll {
                continue;
            }
            if i >= self.height + self.scroll {
                break;
            }
            queue!(
                stdout(),
                Clear(terminal::ClearType::CurrentLine),
                Print(line.to_owned() + "\n")
            )?;
        }
        while i < self.height + self.scroll - 1 {
            i += 1;
            queue!(
                stdout(),
                Clear(terminal::ClearType::CurrentLine),
                Print("\n")
            )?;
        }

        queue!(stdout(), MoveTo(self.cursor.0, self.cursor.1), Show)?;
        stdout().flush()?;
        Ok(())
    }

    pub fn cursor_up(&mut self, by: u16) -> Result<(), Error> {
        let (actual, delta) = if self.cursor.1 <= by {
            (0, by - self.cursor.1)
        } else {
            (self.cursor.1 - by, 0)
        };

        execute!(stdout(), MoveToRow(actual))?;
        self.scroll_up(delta);
        self.cursor.1 = actual;

        self.cursor_right(0)
    }

    pub fn cursor_down(&mut self, by: u16) -> Result<(), Error> {
        let calc = self.cursor.1 + by;
        let actual = if calc >= self.height {
            self.height - 1
        } else {
            calc
        };
        let delta = calc - actual;

        execute!(stdout(), MoveToRow(actual))?;
        self.scroll_down(delta);
        self.cursor.1 = actual;

        self.cursor_right(0)
    }

    pub fn cursor_left(&mut self, by: u16) -> Result<(), Error> {
        self.cursor.0 = if self.cursor.0 <= by {
            0
        } else {
            self.cursor.0 - by
        };
        execute!(stdout(), MoveToColumn(self.cursor.0))?;
        Ok(())
    }

    pub fn cursor_right(&mut self, by: u16) -> Result<(), Error> {
        let calc = self.cursor.0 + by;
        let line = self
            .content
            .get((self.scroll + self.cursor.1) as usize)
            .unwrap_or(&String::from(""))
            .len() as u16;
        self.cursor.0 = if calc >= line { line } else { calc };
        execute!(stdout(), MoveToColumn(self.cursor.0))?;
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
        let calc = self.scroll + by;
        let count = self.content.len() as u16;
        self.scroll = if calc >= count { count - 1 } else { calc };
    }

    pub fn ask_command() -> Result<u8, Error> {
        let mut buf: [u8; 1] = [0; 1];
        std::io::stdin().read_exact(&mut buf)?;
        Ok(buf[0])
    }
}

impl Drop for Console {
    fn drop(&mut self) {
        execute!(stdout(), LeaveAlternateScreen).expect("Failed to exit alternate screen!");
        terminal::disable_raw_mode().expect("Failed to disable raw mode!");
    }
}
