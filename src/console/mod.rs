use std::io::{stdout, Error, Write};

use crossterm::{
    cursor::{Hide, MoveTo, SetCursorStyle, Show},
    event::{self, Event},
    execute, queue,
    style::Print,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::content::Content;
use cursor::Cursor;
use state::{Control, State};

mod cursor;
mod state;

pub struct Console<'a> {
    pub cursor: Cursor,
    pub state: &'a dyn State,
    file: Content,
    scroll: u16,
}

impl<'a> Console<'a> {
    pub fn new(path: &String) -> Result<Self, Error> {
        let con = Self {
            cursor: Cursor {
                display: 0,
                x: 0,
                y: 0,
            },
            state: &Control,
            file: Content::new(path)?,
            scroll: 0,
        };

        terminal::enable_raw_mode()?;
        execute!(
            stdout(),
            EnterAlternateScreen,
            SetCursorStyle::BlinkingBlock
        )?;
        Ok(con)
    }

    pub fn get_height() -> u16 {
        terminal::size().unwrap_or((1, 1)).1
    }

    pub fn get_event() -> Result<Event, Error> {
        let event = event::read()?;
        Ok(event)
    }

    pub fn print(&self) -> Result<(), Error> {
        queue!(stdout(), Hide, MoveTo(0, 0))?;

        for i in 0..Self::get_height() - 1 {
            self.print_line((self.scroll + i) as usize, "\n")?;
        }
        self.print_line((self.scroll + Self::get_height() - 1) as usize, "")?;

        queue!(stdout(), MoveTo(self.cursor.display, self.cursor.y), Show)?;
        stdout().flush()?;
        Ok(())
    }

    fn print_line(&self, idx: usize, end: &str) -> Result<(), Error> {
        let line = if idx >= self.file.lines.len() {
            Print(end.to_owned())
        } else {
            Print(self.file.lines[idx].to_owned() + end)
        };
        queue!(stdout(), Clear(ClearType::CurrentLine), line)?;
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
        self.scroll = if count < Self::get_height() {
            0
        } else if calc + Self::get_height() <= count {
            calc
        } else {
            count - Self::get_height()
        };
    }

    pub fn get_line_width(&self) -> u16 {
        self.file
            .lines
            .get((self.scroll + self.cursor.y) as usize)
            .unwrap_or(&String::from(""))
            .len() as u16
    }
}

impl<'a> Drop for Console<'a> {
    fn drop(&mut self) {
        execute!(
            stdout(),
            LeaveAlternateScreen,
            SetCursorStyle::DefaultUserShape
        )
        .expect("Failed to exit alternate screen!");
        terminal::disable_raw_mode().expect("Failed to disable raw mode!");
    }
}
