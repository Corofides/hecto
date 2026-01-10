use std::io::{stdout, Write, Error};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use crossterm::cursor::{MoveTo, Hide, Show};
use crossterm::style::Print;
use crossterm::{queue, Command};
use core::fmt::Display;

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
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }
    pub fn hide_cursor() -> Result<(), Error> {
        Self::queue_command(Hide)
    }
    pub fn show_cursor() -> Result<(), Error> {
        Self::queue_command(Show)
    }
    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }
    pub fn move_cursor_to(position: Position) -> Result<(), Error> {
        Self::queue_command(MoveTo(position.x, position.y))
    }
    pub fn print<T: Display>(string: T) -> Result<(), Error> {
        Self::queue_command(Print(string))
    }
    pub fn size() -> Result<Size, Error> {
        let size = size()?;
        Ok(Size { width: size.0, height: size.1 })
    }
    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }
    pub fn queue_command<T:Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command)?;
        Ok(())
    }
}
