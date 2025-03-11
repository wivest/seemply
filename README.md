# seemply

A terminal-based editor tool with basic notepad functionality.
Open, edit and save text files.
This entire README was written using seemply.

## Installation

### Build from source

Firstly, you need to have [Rust](https://www.rust-lang.org/tools/install) installed on your machine.
Then clone this repository with `git clone` command.
Navigate to cloned repo and build project with `cargo build` command.
Your executable is now located under `./target` folder.
To access **seemply** from anywhere on your computer add its location to `PATH`.

## Guidelines

To display help page run `seemply` without parameters or with `-h`/`--help` option.

The program accepts one _\<path\>_ argument, which is a path to a text file you want to open.
File will be created if not present under specified path.

Editor operates in two modes: _Control_ and _Input_, with former on start up.
Use `I` key to enter _Input_ mode and `Esc` key to switch back to _Control_ mode.
Control mode is where you perform actions, such as navigate cursor with `W`/`A`/`S`/`D` keys,
save file with `R` key or exit editor with `Q` key.
Input mode allows you to modify file content: type characters to insert them at cursor position,
use `Backspace` to delete character to the left and `Enter` to create a newline.
