use std::fs;

pub struct File {
    pub lines: Vec<String>,
}

impl File {
    pub fn new(path: &String) -> Self {
        let content = fs::read_to_string(path).expect("File at specified path doesn't exist!");
        let lines: Vec<String> = content.lines().map(|l| l.to_owned()).collect();
        Self { lines }
    }

    pub fn get_bound(&self, height: u16) -> u16 {
        let len = self.lines.len() as u16;
        if len > height {
            height
        } else {
            len
        }
    }

    pub fn get_line_width(&self, row: usize) -> u16 {
        self.lines.get(row).unwrap_or(&String::from("")).len() as u16
    }

    pub fn insert_char(&mut self, ch: char, row: usize, idx: usize) {
        let empty = &mut String::from("");
        let line = self.lines.get_mut(row).unwrap_or(empty);

        line.insert(idx, ch);
    }

    pub fn backspace(&mut self, row: usize) {
        let empty = &mut String::from("");
        let line = self.lines.get_mut(row).unwrap_or(empty);

        // if self.cursor.display != 0 {
        //     line.remove(self.cursor.display as usize - 1);
        //     self.cursor.left(1);
        // } else if self.scroll + self.cursor.y != 0 {
        //     let line = line.to_owned();
        //     let line = line.as_str();

        //     let empty = &mut String::from("");
        //     let previous = self
        //         .lines
        //         .get_mut((self.scroll + self.cursor.y - 1) as usize)
        //         .unwrap_or(empty);

        //     let old = previous.len() as u16;
        //     previous.push_str(line);
        //     let updated = previous.len() as u16;
        //     self.lines.remove((self.scroll + self.cursor.y) as usize);

        //     self.cursor.up(1);
        //     self.cursor.right(old, updated);
        // }
    }

    pub fn insert_newline(&mut self, row: usize, idx: usize) {
        let empty = &mut String::from("");
        let line = self.lines.get_mut(row).unwrap_or(empty);

        let newline = line.split_off(idx);
        self.lines.insert(row + 1, newline);
    }
}
