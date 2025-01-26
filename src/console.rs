pub fn init() {
    print!("\x1B[?1049h");
    print!("\x1B[H");
}

pub fn drop() {
    print!("\x1B[?1049l");
}
