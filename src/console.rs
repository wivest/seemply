use std::{io::Write, process::Command};

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
    let output = Command::new("cmd")
        .args(["/C", "echo \x1B[18t"])
        .output()
        .expect("Output failed!");

    let answer = String::from_utf8(output.stdout).unwrap();
    println!("{}", answer);

    0
}

pub fn drop() {
    println!("\x1B[?1049l");
}
