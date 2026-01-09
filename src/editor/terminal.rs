use std::io::{stdout, Write, Error};
use crossterm::queue;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use crossterm::cursor::{MoveTo, Hide, Show};
use crossterm::style::Print;

pub struct Terminal;

#[derive(Copy, Clone)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

#[derive(Copy, Clone)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Terminal {
    pub fn flush() -> Result<(), Error> {
        stdout().flush()
    }
    pub fn terminate() -> Result<(), Error> {
        disable_raw_mode()?;
        Ok(())
    }
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position {x: 0, y: 0})?;
        Self::execute()?;
        Ok(())
    }
    pub fn clear_line() -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::CurrentLine))?;
        Ok(())
    }
    pub fn hide_cursor() -> Result<(), Error> {
        queue!(stdout(), Hide)
    }
    pub fn show_cursor() -> Result<(), Error> {
        queue!(stdout(), Show)
    }
    pub fn clear_screen() -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::All))
    }
    pub fn move_cursor_to(position: Position) -> Result<(), Error> {
        queue!(stdout(), MoveTo(position.x, position.y))
    }
    pub fn print(string: &str) -> Result<(), Error> {
        queue!(stdout(), Print(string))
    }
    pub fn size() -> Result<Size, Error> {
        let size = size()?;
        Ok(Size { width: size.0, height: size.1 })
    }
    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }
}
