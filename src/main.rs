use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = validate(&args);

    let exists = Path::new(filename).is_file();
    dbg!(exists);
}

fn validate(args: &Vec<String>) -> &String {
    if args.len() != 2 {
        panic!();
    }
    return &args[1];
}
