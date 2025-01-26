use std::env;

pub fn get_filename() -> String {
    let args: Vec<String> = env::args().collect();
    validate(&args).clone()
}

fn validate(args: &Vec<String>) -> &String {
    if args.len() != 2 {
        panic!("Argument length must be 2!");
    }
    return &args[1];
}
