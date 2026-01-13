use std::io::Error;
use super::terminal::{Terminal, Size, Position};

mod buffer;
use buffer::Buffer;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
}

impl View {
    pub fn resize(&mut self, to: Size) {
        self.size = to;
        self.needs_redraw = true;
    }
    pub fn render_line(at: usize, line_text: &str) -> Result<(), Error> {
        Terminal::move_caret_to(Position {x: 0, y: at})?;
        Terminal::clear_line()?;
        Terminal::print(line_text)?;
        Ok(())
    }
    // pub fn draw_welcome_message() -> Result<(), Error> {
    //     let mut welcome_message = format!("{NAME} editor -- version {VERSION}");

    //     let width = Terminal::size()?.width;
    //     let len = welcome_message.len();

    //     #[allow(clippy::integer_division)]
    //     let padding = (width.saturating_sub(len)) / 2;

    //     let spaces = " ".repeat(padding.saturating_sub(1));

    //     welcome_message = format!("~{spaces}{welcome_message}");
    //     welcome_message.truncate(width);

    //     Terminal::print(&welcome_message)?;
    //     Ok(())
    // }
    // pub fn render_welcome_screen(&self) -> Result<(), Error> {
    //     let Size { height, .. } = Terminal::size()?;
    //    
    //     for current_row in 0..height {
    //         Terminal::move_caret_to(Position {x: 0, y: current_row})?;
    //         Terminal::clear_line()?;

    //         #[allow(clippy::integer_division)]
    //         if current_row == height / 3 {
    //             Self::draw_welcome_message()?;
    //         } else {
    //             Self::draw_empty_row()?;
    //         }
    //     }
    //     Terminal::flush()?;
    //     Ok(())
    // }
    // pub fn render_buffer(&self) -> Result<(), Error> {
    //     let Size {height, width} = Terminal::size()?;

    //     for current_row in 0..height {
    //         Terminal::move_caret_to(Position {x: 0, y: current_row})?;
    //         Terminal::clear_line()?;

    //         if let Some(line) = self.buffer.lines.get(current_row) {
    //             let mut display_line = String::from(line);
    //             display_line.truncate(width);
    //             Terminal::print(&display_line)?;
    //         } else {
    //             Self::draw_empty_row()?;
    //         }
    //     }
    //     Terminal::flush()?;
    //     Ok(())
    // }
    pub fn render(&mut self) -> Result<(), Error> {
        if !self.needs_redraw {
            return Ok(());
        }

        let Size { width, height } = self.size;

        if height == 0 || width == 0 {
            return Ok(());
        }

        #[allow(clippy::integer_division)]
        let vertical_center = height / 3;

        for current_row in 0..height {
            if let Some(line) = self.buffer.lines.get(current_row) {
                let truncated_line = if line.len() >= width {
                    &line[0..width]
                } else {
                    line
                };
                Self::render_line(current_row, truncated_line)?;
            } else if current_row == vertical_center && self.buffer.is_empty() {
                Self::render_line(current_row, &Self::build_welcome_message(width))?;
            } else {
                Self::render_line(current_row, "~")?;
            }
        }
        self.needs_redraw = false;
        Ok(())
    }
    pub fn build_welcome_message(width: usize) -> String {
        if width == 0 {
            return " ".to_string();
        }
        
        let welcome_message = format!("{NAME} editor -- version {VERSION}");

        let len = welcome_message.len();

        if width <= len {
            "~" .to_string();
        }

        #[allow(clippy::integer_division)]
        let padding = (width.saturating_sub(len).saturating_sub(1)) / 2;
        
        let mut full_message = format!("~{}{}", " ".repeat(padding), welcome_message);
        full_message.truncate(width);
        full_message

    }
    pub fn load(&mut self, filename: &String) {
        if let Ok(buffer) = Buffer::load(filename) {
            self.buffer = buffer;
        }
    }
}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(),
        }
    }
}
