use std::io::stdout;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};
use crossterm::cursor::MoveTo;

pub struct Terminal {}

impl Terminal {
    pub fn clear_screen() -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All))
    }
    pub fn move_to(x: u16, y: u16) -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        execute!(stdout, MoveTo(x, y))
    }
}
