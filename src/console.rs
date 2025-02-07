pub struct Console {
    height: u16,
}

impl Console {
    pub fn new() -> Self {
        println!("\x1B[?1049h");
        print!("\x1B[H");

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

fn get_height() -> u16 {
    let size = termsize::get().expect("Expected to get size!");
    size.rows
}
