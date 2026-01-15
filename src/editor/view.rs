use super::terminal::{Terminal, Size};
use crate::editor::{Position};
use std::cmp::{min, max};

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

pub struct MoveCursor {
    x: isize,
    y: isize,
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
        self.adjust_scroll_offset();
        self.needs_redraw = true;
    }
    pub fn get_position(&mut self) -> Position {
        Position {
            col: self.position.col,
            row: self.position.row,
        }
    }
    pub fn get_scroll_offset(&mut self) -> Position {
        Position {
            col: self.scroll_offset.col,
            row: self.scroll_offset.row,
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
    pub fn adjust_scroll_offset(&mut self) {
       
        let mut needs_redraw = false;
        let Position {mut col, mut row} = self.position;
        let Size { height, width } = Terminal::size().unwrap_or_default();

        let mut offset_row = self.scroll_offset.row;
        let mut offset_col = self.scroll_offset.col;

        let row_diff = row.saturating_sub(self.scroll_offset.row);

        if row < offset_row {
            offset_row = row;
            needs_redraw = true;
        }
        
        if row_diff > height.saturating_sub(1) {
            // Move the offset down by 1.
            offset_row = offset_row.saturating_add(
                row_diff - height.saturating_sub(1)
            );
            needs_redraw = true;
        }

        let col_diff = col.saturating_sub(self.scroll_offset.col);

        if col < offset_col {
            offset_col = col;
            needs_redraw = true;
        }

        if col_diff > width.saturating_sub(1) {
            offset_col = offset_col.saturating_add(
                col_diff - width.saturating_sub(1)
            );
            needs_redraw = true;
        }

        self.scroll_offset = Position {
            col: offset_col,
            row: offset_row,
        };
        self.needs_redraw = needs_redraw;

    }
    /* Assignment functions */
    pub fn move_view(&mut self, direction: Move) {
        let Position {mut col, mut row} = self.position;
        //let Position {mut col as offset_col, mut row as offset_row} = self.scroll_offset;
        let mut offset_col = self.scroll_offset.col;
        let mut offset_row = self.scroll_offset.row;
       
        let mut move_by = MoveCursor { x: 0, y: 0 };
        let Size { height, width } = Terminal::size().unwrap_or_default();

        match direction {
            Move::Up => {
                row = row.saturating_sub(1);
            }
            Move::Down => {
                row = row.saturating_add(1);
            }
            Move::Left => {
                col = col.saturating_sub(1);
            }
            Move::Right => {
                col = col.saturating_add(1);
            }
            Move::PageUp => {
                row = row.saturating_sub(height);
            }
            Move::PageDown => {
                row = row.saturating_add(height.saturating_sub(1));
            }
            Move::StartOfRow => {
                col = 0;
            }
            Move::EndOfRow => {
                col = col.saturating_add(width.saturating_sub(1));
            }
        }
                
        self.position = Position { col, row }; 
        self.adjust_scroll_offset();
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
