use std::io::{stdout, Write, Error};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, size, Clear, ClearType, 
    EnterAlternateScreen, LeaveAlternateScreen, 
    EnableLineWrap, DisableLineWrap, SetTitle
};
use crossterm::cursor::{MoveTo, Hide, Show};
use crossterm::style::{Attribute, Print};
use crossterm::{queue, Command};
use super::{Position, Size, AnnotatedString};

pub struct Terminal {
}

impl Terminal {
    pub fn terminate() -> Result<(), Error> {
        Self::leave_alternate_screen()?;
        Self::enable_line_wrap()?;
        Self::show_caret()?;
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }
    pub fn initialize() -> Result<(), Error> {
        // Move to the alternate screen first.
        enable_raw_mode()?;
        Self::enter_alternate_screen()?;
        Self::disable_line_wrap()?;
        Self::clear_screen()?;
        Self::move_caret_to(Position {col: 0, row: 0})?;
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
    pub fn set_title(title: &str) -> Result<(), Error> {
        Self::queue_command(SetTitle(title))?;
        Ok(())
    }
    pub fn enable_line_wrap() -> Result<(), Error> {
        Self::queue_command(EnableLineWrap)?;
        Ok(())
    }
    pub fn disable_line_wrap() -> Result<(), Error> {
        Self::queue_command(DisableLineWrap)?;
        Ok(())
    }
    pub fn enter_alternate_screen() -> Result<(), Error> {
        Self::queue_command(EnterAlternateScreen)?;
        Ok(())
    }
    pub fn leave_alternate_screen() -> Result<(), Error> {
        Self::queue_command(LeaveAlternateScreen)?;
        Ok(())
    }
    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }
    pub fn move_caret_to(position: Position) -> Result<(), Error> {
        #[allow(clippy::as_conversions,clippy::cast_possible_truncation)]
        Self::queue_command(MoveTo(position.col as u16, position.row as u16))
    }
    pub fn print(string: &str) -> Result<(), Error> {
        Self::queue_command(Print(string))
    }
    pub fn print_row(row: usize, line_text: &str) -> Result<(), Error> {
        Self::move_caret_to(Position { col: 0, row: row, })?;
        Self::clear_line()?;
        Self::print(line_text)?;
        Ok(())
    }
    pub fn print_annotated_row(row: usize, annotated_string: &AnnotatedString) -> Result<(), Error> {
        log::debug!("Print Annotated Row: {}", row);
        Self::move_caret_to(Position { col: 0, row: row, })?;
        Self::clear_line()?;

        let annotated_fragments = annotated_string.get_annotated_fragments();

        for fragment in annotated_fragments {
            Self::print(&fragment.string)?;
        }
        //Self::print(annotated_string.get_display_string())?;
        Ok(())
    }
    pub fn print_inverted_row(row: usize, line_text: &str) -> Result<(), Error> {
        let width = Self::size()?.width;
        Self::print_row(
            row,
            &format!(
                "{}{:width$.width$}{}",
                Attribute::Reverse,
                line_text,
                Attribute::Reset
            ),
        )
    }
    pub fn size() -> Result<Size, Error> {
        let (width_u16, height_u16) = size()?;

        #[allow(clippy::as_conversions)]
        let height = height_u16 as usize;

        #[allow(clippy::as_conversions)]
        let width = width_u16 as usize;

        Ok(Size { width, height })
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
