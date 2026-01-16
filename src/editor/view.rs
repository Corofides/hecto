use super::{
    editorcommand::{Direction, EditorCommand},
    terminal::{Position, Terminal, Size},
};
use std::cmp::min;
mod buffer;
use buffer::Buffer;
mod location;
use location::Location;
mod line;
const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
    location: Location,
    scroll_offset: Location,
    last_x_position: usize,
}

impl View {
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
        let top = self.scroll_offset.y;

        for current_row in 0..height {
            if let Some(line) = self.buffer.lines.get(current_row.saturating_add(top)) {
                let left = self.scroll_offset.x;
                let right = self.scroll_offset.x.saturating_add(width);
                Self::render_line(current_row, &line.get(left..right));
            } else if current_row == vertical_center && self.buffer.is_empty() {
                Self::render_line(current_row, &Self::build_welcome_message(width));
            } else {
                Self::render_line(current_row, "~");
            }
        }
        self.needs_redraw = false;
    }
    pub fn handle_command(&mut self, command: EditorCommand) {
        match command {
            EditorCommand::Resize(size) => self.resize(size),
            EditorCommand::Move(direction) => self.move_text_location(&direction),
            EditorCommand::Quit => {},
        }
    }
    pub fn load(&mut self, filename: &String) {
        if let Ok(buffer) = Buffer::load(filename) {
            self.buffer = buffer;
            self.needs_redraw = true;
        }
    }
    pub fn get_position(&mut self) -> Position {
        self.location.subtract(&self.scroll_offset).into()
    }
    fn get_x_position(&self, x: usize, line: usize) -> usize {
        let top = self.scroll_offset.y;
        let mut max = 0;

        if let Some(line) = self.buffer.lines.get(line.saturating_add(top)) {
            max = line.len().saturating_sub(1)
        }

        if x > max {
            return max;
        }

        if x < self.last_x_position {
            return min(self.last_x_position, max);
        }

        x
    }
    pub fn move_text_location(&mut self, direction: &Direction) {
        let Location { mut x, mut y } = self.location;
        let Size { height, width } = self.size;
        let top = self.scroll_offset.y;
        let left = self.scroll_offset.x;
        let mut current_line_length = 0;

        // Get the current line length
        if let Some(line) = self.buffer.lines.get(y.saturating_add(top)) {
            current_line_length = line.len();
        }

        match direction {
            Direction::Up => {
                y = y.saturating_sub(1);
                x = self.get_x_position(x, y);
            },
            Direction::Down => {
                if y < self.buffer.lines.len().saturating_sub(1) {
                    y = y.saturating_add(1);
                }

                x = self.get_x_position(x, y);
            },
            Direction::Left => {
                
                if x == 0 && y > 0 {
                    y = y.saturating_sub(1); 
                    //x = std::usize::MAX;
                    x = self.get_x_position(std::usize::MAX, y)
                } else if x > 0 {
                    x = x.saturating_sub(1);
                }

                self.last_x_position = x;
            },
            Direction::Right => {
                let x_at_end = x == current_line_length.saturating_sub(1);

                if x < current_line_length.saturating_sub(1) {
                    x = x.saturating_add(1);
                } else if y < self.buffer.lines.len() {
                    x = 0;
                    y = y.saturating_add(1);
                }

                self.last_x_position = x;
            }
            Direction::PageUp => {
                y = y.saturating_sub(height);
            },
            Direction::PageDown => {
                y = min(self.buffer.lines.len(), y.saturating_add(height));
            },
            Direction::Home => {
                x = 0;
            },
            Direction::End => {
                x = current_line_length
            },
        }

        self.location = Location { x, y };
        self.scroll_location_into_view();
    }
    fn resize(&mut self, to: Size) {
        self.size = to;
        self.scroll_location_into_view();
        self.needs_redraw = true;
    }
    fn scroll_location_into_view(&mut self) {
        let Location { x, y } = self.location;
        let Size { width, height } = self.size;
        let mut offset_changed = false;

        if y < self.scroll_offset.y {
            self.scroll_offset.y = y;
            offset_changed = true;
        } else if y >= self.scroll_offset.y.saturating_add(height) {
            self.scroll_offset.y = y.saturating_sub(height).saturating_add(1);
            offset_changed = true;
        }

        if x < self.scroll_offset.x {
            self.scroll_offset.x = x;
            offset_changed = true;
        } else if x >= self.scroll_offset.x.saturating_add(width) {
            self.scroll_offset.x = x.saturating_sub(width).saturating_add(1);
            offset_changed = true;
        }

        self.needs_redraw = offset_changed;
    }
    fn render_line(at: usize, line_text: &str) {
        let result = Terminal::print_row(at, line_text);
        debug_assert!(result.is_ok(), "Failed to render line");
    }
    fn build_welcome_message(width: usize) -> String {
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
}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(),
            location: Location::default(),
            scroll_offset: Location::default(),
            last_x_position: 0,
        }
    }
}
