use super::terminal::{Terminal, Size};
use crate::editor::{Position};
use std::cmp::min;

mod buffer;
use buffer::Buffer;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub enum Move {
    Left,
    Right,
    Up,
    Down,
    PageUp,
    PageDown,
    StartOfRow,
    EndOfRow,
}

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    position: Position,
    scroll_offset: Position,
    size: Size,
}

impl View {
    pub fn resize(&mut self, to: Size) {
        self.size = to;
        self.needs_redraw = true;
    }
    pub fn get_position(&mut self) -> Position {
        Position {
            col: self.position.col,
            row: self.position.row,
        }
    }
    pub fn render_line(at: usize, line_text: &str) {
        let result = Terminal::print_row(at, line_text);
        debug_assert!(result.is_ok(), "Failed to render line");
    }
    pub fn render(&mut self) {
        if !self.needs_redraw {
            return;
        }

        let Size { width, height } = self.size;

        if height == 0 || width == 0 {
            return;
        }

        #[allow(clippy::integer_division)]
        let vertical_center = height / 3;

        for current_row in 0..height {

            if let Some(line) = self.buffer.lines.get(current_row + self.scroll_offset.row) {

                let length_of_line = line.len();
                let column_offset = self.scroll_offset.col;

                // Don't draw the line if we are out of bounds.
                if (column_offset >= length_of_line) {
                    Self::render_line(current_row, "");
                    continue;
                }

                let truncated_line = if line.len() >= column_offset + width {
                    &line[column_offset..column_offset + width]
                } else {
                    &line[column_offset..line.len()]
                };
                Self::render_line(current_row, truncated_line);
            } else if current_row == vertical_center && self.buffer.is_empty() {
                Self::render_line(current_row, &Self::build_welcome_message(width));
            } else {
                Self::render_line(current_row, "~");
            }
        }
        self.needs_redraw = false;
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
    /* Assignment functions */
    pub fn move_view(&mut self, direction: Move) {
        let Position {mut col, mut row} = self.position;
        //let Position {mut col as offset_col, mut row as offset_row} = self.scroll_offset;
        let mut offset_col = self.scroll_offset.col;
        let mut offset_row = self.scroll_offset.row;
        
        let Size { height, width } = Terminal::size().unwrap_or_default();

        match direction {
            Move::Up => {
                let prev_row = row;
                row = row.saturating_sub(1);
                if (prev_row == 0) && offset_row > 0 {
                    offset_row = offset_row - 1;
                    self.needs_redraw = true;
                }
            }
            Move::Down => {
                let add_offset = row.saturating_add(1) > height.saturating_sub(1);
                row = min(row.saturating_add(1), height.saturating_sub(1));
                if add_offset {
                    offset_row = offset_row + 1;
                    self.needs_redraw = true;
                }
            }
            Move::Left => {
                let prev_col = col;
                col = col.saturating_sub(1);
                if (prev_col == 0) && offset_col > 0 {
                    offset_col = offset_col - 1;
                    self.needs_redraw = true;
                }
            }
            Move::Right => {
                let add_offset = col.saturating_add(1) > width.saturating_sub(1);
                col = min(width.saturating_sub(1), col.saturating_add(1));
                if add_offset {
                    offset_col = offset_col + 1;
                    self.needs_redraw = true;
                }
            }
            Move::PageUp => {
                row = col.saturating_sub(height);
            }
            Move::PageDown => {
                row = row + height;
            }
            Move::StartOfRow => {
                col = 0;
            }
            Move::EndOfRow => {
                col = col + width;
            }
        }

        self.scroll_offset = Position {
            col: offset_col, // sets the offset to the col, row minus our view port.
            row: offset_row,
        };

        self.position = Position { col, row }; 
        self.render();
    }
}

impl Default for View {
    fn default() -> Self {
        Self {
            position: Position::default(),
            scroll_offset: Position::default(),
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(),
        }
    }
}
