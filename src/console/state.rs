use crossterm::event::KeyCode;

use super::Console;

pub trait State {
    fn handle_input(&self, code: KeyCode, con: &mut Console) -> bool;
}

pub struct Control;

pub struct Input;

impl State for Control {
    fn handle_input(&self, code: KeyCode, con: &mut Console) -> bool {
        if code == KeyCode::Char('q') {
            return false;
        }
        if code == KeyCode::Char('w') {
            let delta = con.cursor.up(1);
            if delta != 0 {
                con.scroll_up(delta);
            }
            let line = con.get_line_width();
            con.cursor.right(0, line);
        }
        if code == KeyCode::Char('s') {
            let delta = con.cursor.down(1, con.get_bound());
            if delta != 0 {
                con.scroll_down(delta);
            }
            let line = con.get_line_width();
            con.cursor.right(0, line);
        }
        if code == KeyCode::Char('a') {
            con.cursor.left(1);
        }
        if code == KeyCode::Char('d') {
            let line = con.get_line_width();
            con.cursor.right(1, line);
        }
        if code == KeyCode::Char('i') {
            con.state = &Input;
        }
        if code == KeyCode::Char('r') {
            con.file.save().expect("Failed to save file!");
        }

        true
    }
}

impl State for Input {
    fn handle_input(&self, code: KeyCode, con: &mut Console) -> bool {
        match code {
            KeyCode::Esc => con.state = &Control,
            KeyCode::Enter => {
                con.file.insert_newline(
                    (con.scroll + con.cursor.y) as usize,
                    con.cursor.display as usize,
                );
                let delta = con.cursor.down(1, con.file.get_bound(con.height));
                if delta != 0 {
                    con.scroll_down(delta);
                }
                con.cursor.left(con.cursor.x);
            }
            KeyCode::Backspace => {
                let row = (con.scroll + con.cursor.y) as usize;
                let width = con.file.get_line_width(row - 1);
                let newline = con.file.backspace(row, con.cursor.display as usize);
                if newline {
                    con.cursor.up(1);
                    con.cursor.right(width, width);
                } else {
                    con.cursor.left(1);
                }
            }
            KeyCode::Char(ch) => {
                let row = (con.scroll + con.cursor.y) as usize;
                con.file.insert_char(ch, row, con.cursor.display as usize);
                con.cursor.right(1, con.file.get_line_width(row));
            }
            _ => (),
        }
        true
    }
}
