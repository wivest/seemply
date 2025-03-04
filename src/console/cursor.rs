pub struct Cursor {
    pub display: u16,
    pub x: u16,
    pub y: u16,
}

impl Cursor {
    pub fn up(&mut self, by: u16) -> u16 {
        let delta;
        (self.y, delta) = if self.y <= by {
            (0, by - self.y)
        } else {
            (self.y - by, 0)
        };

        delta
    }

    pub fn down(&mut self, by: u16, bound: u16) -> u16 {
        let calc = self.y + by;
        self.y = if calc >= bound { bound - 1 } else { calc };

        calc - self.y
    }

    pub fn left(&mut self, by: u16) {
        self.display = if self.x <= by { 0 } else { self.display - by };

        if by != 0 {
            self.x = self.display;
        }
    }

    pub fn right(&mut self, by: u16, line: u16) {
        let calc = self.x + by;
        self.display = if calc >= line { line } else { calc };

        if by != 0 {
            self.x = self.display;
        }
    }
}
