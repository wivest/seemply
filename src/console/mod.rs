use std::io::{stdout, Error, Write};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event},
    execute, queue,
    style::Print,
    terminal::{self, Clear, EnterAlternateScreen, LeaveAlternateScreen},
};

use cursor::Cursor;
use state::{Control, State};

mod cursor;
mod state;

pub struct Console<'a> {
    pub cursor: Cursor,
    pub state: &'a dyn State,
    height: u16,
    content: Vec<String>,
    scroll: u16,
}

impl<'a> Console<'a> {
    pub fn new(content: String) -> Result<Self, Error> {
        terminal::enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen)?;
        let size = terminal::size()?;
        let content: Vec<String> = content.lines().map(|l| l.to_owned()).collect();

        Ok(Console {
            cursor: Cursor {
                display: 0,
                x: 0,
                y: 0,
            },
            state: &Control,
            height: size.1,
            content,
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
        let count = self.content.len() as u16;
        self.scroll = if count < self.height {
            0
        } else if calc + self.height <= count {
            calc
        } else {
            count - self.height
        };
    }

    pub fn get_bound(&self) -> u16 {
        let lines = self.content.len() as u16;
        if lines > self.height {
            self.height
        } else {
            lines
        }
    }

    pub fn get_line_width(&self) -> u16 {
        self.content
            .get((self.scroll + self.cursor.y) as usize)
            .unwrap_or(&String::from(""))
            .len() as u16
    }

    pub fn insert_char(&mut self, ch: char) {
        let empty = &mut String::from("");
        let line = self
            .content
            .get_mut((self.scroll + self.cursor.y) as usize)
            .unwrap_or(empty);

        line.insert(self.cursor.display as usize, ch);
        self.cursor.right(1, line.len() as u16);
    }

    pub fn backspace(&mut self) {
        let empty = &mut String::from("");
        let line = self
            .content
            .get_mut((self.scroll + self.cursor.y) as usize)
            .unwrap_or(empty);

        if self.cursor.display != 0 {
            line.remove(self.cursor.display as usize - 1);
            self.cursor.left(1);
        } else if self.scroll + self.cursor.y != 0 {
            let line = line.to_owned();
            let line = line.as_str();

            let empty = &mut String::from("");
            let previous = self
                .content
                .get_mut((self.scroll + self.cursor.y - 1) as usize)
                .unwrap_or(empty);

            let old = previous.len() as u16;
            previous.push_str(line);
            let updated = previous.len() as u16;
            self.content.remove((self.scroll + self.cursor.y) as usize);

            self.cursor.up(1);
            self.cursor.right(old, updated);
        }
    }

    pub fn insert_newline(&mut self) {
        let empty = &mut String::from("");
        let line = self
            .content
            .get_mut((self.scroll + self.cursor.y) as usize)
            .unwrap_or(empty);

        let newline = line.split_off(self.cursor.display as usize);
        self.content
            .insert((self.scroll + self.cursor.y) as usize + 1, newline);
        let delta = self.cursor.down(1, self.get_bound());
        if delta != 0 {
            self.scroll_down(delta);
        }
        self.cursor.left(self.cursor.x);
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
