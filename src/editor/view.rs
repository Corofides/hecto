use std::io::Error;
use super::terminal::{Terminal, Size, Position};

use super::buffer::Buffer;

pub struct View {
    buffer: Buffer
}

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

impl View {
    pub const fn default() -> Self {
        Self {
            buffer: Buffer::default(),
        }
    }
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
    pub fn render(&self) -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;
        Terminal::move_caret_to(Position {x: 0, y: 0})?;
       
        for current_row in 0..height {
            Terminal::clear_line()?;

            #[allow(clippy::integer_division)]
            if current_row == height / 3 {
                Self::draw_welcome_message()?;
            } else {
                let current_row = self.buffer.get_row(current_row);

                match current_row {
                    Some(text) => {
                        Terminal::print(text)?;
                    }
                    None => {
                        Self::draw_empty_row()?;
                    }
                }
                /*if current_row < rows_in_buffer {
                    Self::draw_row_from_buffer(current_row)?;
                } else {
                    Self::draw_empty_row()?;
                }*/
            }

            if current_row == 0 {
                Terminal::print("Hello, world!")?;
            }
            
            if current_row.saturating_add(1) < height {
                Terminal::print("\r\n")?; //print!("\r\n")?;
            }
        }
        Terminal::flush()?;
        Ok(())
    }
}
