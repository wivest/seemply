use std::env;
use std::fs;
use std::path::Path;

mod console;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = validate(&args);
    let content = get_content(filename);
    console::init();

    println!("{}", content);
    let mut buffer: String = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Input error!");
    console::drop();
}

fn validate(args: &Vec<String>) -> &String {
    if args.len() != 2 {
        panic!("Argument length must be 2!");
    }
    return &args[1];
}

fn get_content(filename: &String) -> String {
    let exists = Path::new(&filename).is_file();
    if !exists {
        panic!("File at specified path doesn't exist!");
    }
    return fs::read_to_string(filename).unwrap();
}
