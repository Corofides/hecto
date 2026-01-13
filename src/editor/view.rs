use std::io::Error;
use super::terminal::{Terminal, Size, Position};

mod buffer;
use buffer::Buffer;

#[derive(Default)]
pub struct View {
    buffer: Buffer,
    pub needs_redraw: bool
}

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

impl View {
    fn draw_empty_row() -> Result<(), Error> {
        Terminal::print("~")?;
        Ok(())
    }
    /*fn draw_row_from_buffer(&self, row: usize) -> Result<(), Error> {
        Terminal::print(self.buffer.get_row(row));
        Ok(())
    }*/
    pub fn draw_welcome_message() -> Result<(), Error> {
        let mut welcome_message = format!("{NAME} editor -- version {VERSION}");

        let width = Terminal::size()?.width;
        let len = welcome_message.len();

        #[allow(clippy::integer_division)]
        let padding = (width.saturating_sub(len)) / 2;

        let spaces = " ".repeat(padding.saturating_sub(1));

        welcome_message = format!("~{spaces}{welcome_message}");
        welcome_message.truncate(width);

        Terminal::print(&welcome_message)?;
        Ok(())
    }
    pub fn render_welcome_screen(&self) -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;
       
        for current_row in 0..height {
            Terminal::move_caret_to(Position {x: 0, y: current_row})?;
            Terminal::clear_line()?;

            #[allow(clippy::integer_division)]
            if current_row == height / 3 {
                Self::draw_welcome_message()?;
            } else {
                Self::draw_empty_row()?;
            }
        }
        Terminal::flush()?;
        Ok(())
    }
    pub fn render_buffer(&self) -> Result<(), Error> {
        let Size {height, width} = Terminal::size()?;

        for current_row in 0..height {
            Terminal::move_caret_to(Position {x: 0, y: current_row})?;
            Terminal::clear_line()?;

            if let Some(line) = self.buffer.lines.get(current_row) {
                let mut display_line = String::from(line);
                display_line.truncate(width);
                Terminal::print(&display_line)?;
            } else {
                Self::draw_empty_row()?;
            }
        }
        Terminal::flush()?;
        Ok(())
    }
    pub fn render(&mut self) -> Result<(), Error> {
        if !self.needs_redraw {
            return Ok(());
        }
        if self.buffer.is_empty() {
            self.render_welcome_screen()?;
        } else {
            self.render_buffer()?;
        }
        self.needs_redraw = true;
        Ok(())
    }
    pub fn load(&mut self, filename: &String) {
        if let Ok(buffer) = Buffer::load(filename) {
            self.buffer = buffer;
        }
    }
}
