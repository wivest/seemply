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
}
