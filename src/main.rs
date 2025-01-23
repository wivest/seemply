use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = validate(&args);

    let content = get_content(filename);
    print!("\x1B[?1049h");
    print!("\x1B[H");
    println!("{}", content);
    let mut buffer: String = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Input error!");
    print!("\x1B[?1049l");
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
