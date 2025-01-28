use std::fs;
use std::path::Path;

mod args;
mod console;

fn main() {
    let filename = args::get_filename();
    let content = get_content(&filename);
    console::init();

    console::print(&content, 3);
    let mut buffer: String = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Input error!");
    console::drop();
}

fn get_content(filename: &String) -> String {
    let exists = Path::new(&filename).is_file();
    if !exists {
        panic!("File at specified path doesn't exist!");
    }
    return fs::read_to_string(filename).unwrap();
}
