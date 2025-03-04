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

        true
    }
}

impl State for Input {
    fn handle_input(&self, code: KeyCode, con: &mut Console) -> bool {
        match code {
            KeyCode::Esc => con.state = &Control,
            KeyCode::Enter => con.insert_newline(),
            KeyCode::Backspace => con.backspace(),
            KeyCode::Char(ch) => con.insert_char(ch),
            _ => (),
        }
        true
    }
}
