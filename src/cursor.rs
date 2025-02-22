use std::io::Error;

use crossterm::{cursor::MoveToRow, execute};

use crate::console::Console;

pub struct Cursor<'a> {
    con: &'a mut Console,
    pos: (u16, u16),
}

impl<'a> Cursor<'a> {
    pub fn new(console: &'a mut Console) -> Self {
        Cursor {
            con: console,
            pos: (0, 0),
        }
    }

    pub fn move_up(&mut self, by: u16) -> Result<(), Error> {
        let (actual, delta) = if self.pos.1 <= by {
            (0, by - self.pos.1)
        } else {
            (self.pos.1 - by, 0)
        };

        execute!(std::io::stdout(), MoveToRow(actual))?;
        self.con.scroll_up(delta);
        self.pos.1 = actual;

        Ok(())
    }

    pub fn move_down(&mut self, by: u16) -> Result<(), Error> {
        let calc = self.pos.1 + by;
        let actual = if calc >= self.con.height {
            self.con.height - 1
        } else {
            calc
        };
        let delta = calc - actual;

        execute!(std::io::stdout(), MoveToRow(actual))?;
        self.con.scroll_down(delta);
        self.pos.1 = actual;

        Ok(())
    }
}
