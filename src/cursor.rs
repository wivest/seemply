use std::io::Error;

use crossterm::{cursor::MoveToRow, execute};

use crate::console::Console;

pub struct Cursor {
    width: u16,
    height: u16,
    pos: (u16, u16),
}

impl Cursor {
    pub fn new(console: &Console) -> Self {
        Cursor {
            width: console.width,
            height: console.height,
            pos: (0, 0),
        }
    }

    pub fn move_up(&mut self, by: u16) -> Result<(), Error> {
        let h = self.pos.1 + by;
        if h >= self.height {
            let delta = h - self.height + 1;
            execute!(std::io::stdout(), MoveToRow(self.height - 1))?;
        }
        Ok(())
    }
}
