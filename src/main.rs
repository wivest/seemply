use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    if args.len() != 2 {
        panic!();
    }

    let exists = Path::new(filename).is_file();
    dbg!(exists);
}
