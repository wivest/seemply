pub fn init() {
    println!("\x1B[?1049h");
    println!("\x1B[H");
}

pub fn print(content: &String, height: i32) {
    let lines = content.lines();
    let mut i = 0;
    for line in lines {
        if i >= height {
            break;
        }
        println!("{line}");
        i += 1;
    }
}

pub fn get_height() -> i32 {
    println!("\x1B[18t");

    let mut buffer = String::new();
    let size = std::io::stdin()
        .read_line(&mut buffer)
        .expect("Init input error!");
    println!("{} | size: {}", &buffer[2..], size);

    0
}

pub fn drop() {
    println!("\x1B[?1049l");
}
