pub fn init() {
    print!("\x1B[?1049h");
    print!("\x1B[H");
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

fn get_height() -> i32 {
    print!("\x1B[18t");
    0
}

pub fn drop() {
    print!("\x1B[?1049l");
}
