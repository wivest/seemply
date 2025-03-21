use std::fs::{File, OpenOptions};
use std::io::{Error, Read, Seek, Write};

pub struct Content {
    pub lines: Vec<String>,
    file: File,
}

pub enum Backspace {
    Single,
    Chomp(usize),
}

impl Content {
    pub fn new(path: &String) -> Result<Self, Error> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;
        let mut content = String::from("");
        file.read_to_string(&mut content)?;
        let lines: Vec<String> = (content + "\n").lines().map(|l| l.to_owned()).collect();
        Ok(Self { lines, file })
    }

    pub fn save(&mut self) -> Result<(), Error> {
        let content = self.lines.join("\r\n");
        self.file.set_len(0)?;
        self.file.rewind()?;
        self.file.write_all(content.as_bytes())?;
        Ok(())
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

    pub fn backspace(&mut self, row: usize, idx: usize) -> Backspace {
        let empty = &mut String::from("");
        let line = self.lines.get_mut(row).unwrap_or(empty);

        if idx != 0 {
            line.remove(idx - 1);
            Backspace::Single
        } else if row != 0 {
            let line = line.to_owned();
            let line = line.as_str();

            let empty = &mut String::from("");
            let previous = self.lines.get_mut(row - 1).unwrap_or(empty);
            let width = previous.len();

            previous.push_str(line);
            self.lines.remove(row);

            Backspace::Chomp(width)
        } else {
            Backspace::Single
        }
    }

    pub fn insert_newline(&mut self, row: usize, idx: usize) {
        let empty = &mut String::from("");
        let line = self.lines.get_mut(row).unwrap_or(empty);

        let newline = line.split_off(idx);
        self.lines.insert(row + 1, newline);
    }
}
