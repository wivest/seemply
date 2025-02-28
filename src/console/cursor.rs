use std::io::{stdout, Error};

use crossterm::{
    cursor::{MoveToColumn, MoveToRow},
    execute,
};

pub struct Cursor {
    pub display: u16,
    pub x: u16,
    pub y: u16,
    height: u16,
}

impl Cursor {
    pub fn new(size: (u16, u16)) -> Self {
        Cursor {
            display: 0,
            x: 0,
            y: 0,
            height: size.1,
        }
    }

    pub fn up(&mut self, by: u16) -> Result<u16, Error> {
        let (actual, delta) = if self.y <= by {
            (0, by - self.y)
        } else {
            (self.y - by, 0)
        };

        execute!(stdout(), MoveToRow(actual))?;
        self.y = actual;

        Ok(delta)
    }

    pub fn down(&mut self, by: u16) -> Result<u16, Error> {
        let calc = self.y + by;
        let actual = if calc >= self.height {
            self.height - 1
        } else {
            calc
        };
        let delta = calc - actual;

        execute!(stdout(), MoveToRow(actual))?;
        self.y = actual;

        Ok(delta)
    }

    pub fn left(&mut self, by: u16) -> Result<(), Error> {
        self.display = if self.x <= by { 0 } else { self.display - by };

        if by != 0 {
            self.x = self.display;
        }

        execute!(stdout(), MoveToColumn(self.display))?;
        Ok(())
    }

    pub fn right(&mut self, by: u16, line: u16) -> Result<(), Error> {
        let calc = self.x + by;
        self.display = if calc >= line { line } else { calc };

        if by != 0 {
            self.x = self.display;
        }

        execute!(stdout(), MoveToColumn(self.display))?;
        Ok(())
    }
}
