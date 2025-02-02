mod platform;

pub struct Console {
    width: i32,
    height: i32,
}

impl Console {
    pub fn new() -> Self {
        println!("\x1B[?1049h");
        println!("\x1B[H");

        Console {
            width: 0,
            height: 0,
        }
    }
}

impl Drop for Console {
    fn drop(&mut self) {
        println!("\x1B[?1049l");
    }
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
    let output = platform::get_command()
        .arg("\x1B[18t")
        .output()
        .expect("Output failed!");

    let answer = String::from_utf8(output.stdout).unwrap();
    println!("{}", answer);

    0
}
