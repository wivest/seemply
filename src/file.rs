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

    pub fn backspace(&mut self, row: usize, idx: usize) -> bool {
        let empty = &mut String::from("");
        let line = self.lines.get_mut(row).unwrap_or(empty);

        if idx != 0 {
            line.remove(idx - 1);
            false
        } else if row != 0 {
            let line = line.to_owned();
            let line = line.as_str();

            let empty = &mut String::from("");
            let previous = self.lines.get_mut(row - 1).unwrap_or(empty);

            previous.push_str(line);
            self.lines.remove(row);

            true
        } else {
            false
        }
    }

    pub fn insert_newline(&mut self, row: usize, idx: usize) {
        let empty = &mut String::from("");
        let line = self.lines.get_mut(row).unwrap_or(empty);

        let newline = line.split_off(idx);
        self.lines.insert(row + 1, newline);
    }
}
