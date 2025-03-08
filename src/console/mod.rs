use std::io::{stdout, Error, Write};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event},
    execute, queue,
    style::Print,
    terminal::{self, Clear, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::file::File;
use cursor::Cursor;
use state::{Control, State};

mod cursor;
mod state;

pub struct Console<'a> {
    pub cursor: Cursor,
    pub state: &'a dyn State,
    height: u16,
    file: File,
    scroll: u16,
}

impl<'a> Console<'a> {
    pub fn new(path: &String) -> Result<Self, Error> {
        terminal::enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen)?;
        let size = terminal::size()?;

        Ok(Self {
            cursor: Cursor {
                display: 0,
                x: 0,
                y: 0,
            },
            state: &Control,
            height: size.1,
            file: File::new(path),
            scroll: 0,
        })
    }

    pub fn print(&self) -> Result<(), Error> {
        queue!(stdout(), Hide, MoveTo(0, 0))?;

        let mut i = 0;
        for line in &self.file.lines {
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

        queue!(stdout(), MoveTo(self.cursor.display, self.cursor.y), Show)?;
        stdout().flush()?;
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
        let calc = self.scroll + by;
        let count = self.file.lines.len() as u16;
        self.scroll = if count < self.height {
            0
        } else if calc + self.height <= count {
            calc
        } else {
            count - self.height
        };
    }

    pub fn get_bound(&self) -> u16 {
        let lines = self.file.lines.len() as u16;
        if lines > self.height {
            self.height
        } else {
            lines
        }
    }

    pub fn get_line_width(&self) -> u16 {
        self.file
            .lines
            .get((self.scroll + self.cursor.y) as usize)
            .unwrap_or(&String::from(""))
            .len() as u16
    }

    pub fn ask_command() -> Result<Event, Error> {
        let event = event::read()?;
        Ok(event)
    }
}

impl<'a> Drop for Console<'a> {
    fn drop(&mut self) {
        execute!(stdout(), LeaveAlternateScreen).expect("Failed to exit alternate screen!");
        terminal::disable_raw_mode().expect("Failed to disable raw mode!");
    }
}
