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
}
