use std::io::{stdout, Write, Error};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use crossterm::cursor::{MoveTo, Hide, Show};
use crossterm::style::Print;
use crossterm::{queue, Command};

pub struct Terminal;

#[derive(Copy, Clone)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

#[derive(Copy, Clone, Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
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
        Self::move_caret_to(Position {x: 0, y: 0})?;
        Self::execute()?;
        Ok(())
    }
    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }
    pub fn hide_caret() -> Result<(), Error> {
        Self::queue_command(Hide)
    }
    pub fn show_caret() -> Result<(), Error> {
        Self::queue_command(Show)
    }
    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }
    pub fn move_caret_to(position: Position) -> Result<(), Error> {
        #[allow(clippy::as_conversions,clippy::cast_possible_truncation)]
        Self::queue_command(MoveTo(position.x as u16, position.y as u16))
    }
    pub fn print(string: &str) -> Result<(), Error> {
        Self::queue_command(Print(string))
    }
    pub fn size() -> Result<Size, Error> {
        let (width_u16, height_u16) = size()?;

        #[allow(clippy::as_conversion)]
        let height = height_u16 as usize;

        #[allow(clippy::as_conversion)]
        let width = width_u16 as usize;

        Ok(Size { width: width, height: height })
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
