mod platform;

pub struct Console {
    height: i32,
}

impl Console {
    pub fn new() -> Self {
        println!("\x1B[?1049h");
        println!("\x1B[H");

        Console {
            height: get_height(),
        }
    }

    pub fn print(&self, content: &String) {
        let lines = content.lines();
        let mut i = 0;
        for line in lines {
            if i >= self.height {
                break;
            }
            println!("{line}");
            i += 1;
        }
    }
}

impl Drop for Console {
    fn drop(&mut self) {
        println!("\x1B[?1049l");
    }
}

fn get_height() -> i32 {
    let output = platform::get_command()
        .arg("\x1B[18t")
        .output()
        .expect("Output failed!");

    let answer = String::from_utf8(output.stdout).unwrap();
    println!("{}", answer);

    3
}
