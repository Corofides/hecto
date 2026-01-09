use std::io::stdout;
use std::io::Write;
use crossterm::queue;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use crossterm::cursor::{MoveTo, Hide, Show};
use crossterm::style::Print;

pub struct Terminal {}

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Terminal {
    pub fn flush() -> Result<(), std::io::Error> {
        stdout().flush()
    }
    pub fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()?;
        Ok(())
    }
    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position {x: 0, y: 0})?;
        Ok(())
    }
    pub fn hide_cursor() -> Result<(), std::io::Error> {
        queue!(stdout(), Hide)
    }
    pub fn show_cursor() -> Result<(), std::io::Error> {
        queue!(stdout(), Show)
    }
    pub fn clear_screen() -> Result<(), std::io::Error> {
        queue!(stdout(), Clear(ClearType::All))
    }
    pub fn move_cursor_to(position: Position) -> Result<(), std::io::Error> {
        queue!(stdout(), MoveTo(position.x, position.y))
    }
    pub fn print(text: &str) -> Result<(), std::io::Error> {
        queue!(stdout(), Print(text))
    }
    pub fn size() -> Result<Size, std::io::Error> {
        let size = size()?;
        Ok(Size { width: size.0, height: size.1 })
    }
}
