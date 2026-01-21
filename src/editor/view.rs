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
use self::line::Line;
mod textfragment;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
    location: Location,
    scroll_offset: Position,
    // last_x_position: usize,
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
        let top = self.scroll_offset.row;

        for current_row in 0..height {
            if let Some(line) = self.buffer.lines.get(current_row.saturating_add(top)) {
                let left = self.scroll_offset.col;
                let right = self.scroll_offset.col.saturating_add(width);
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

        let Location {x, y} = self.location;

        let mut x_position = 0;
        let current_line = &self.buffer.lines.get(y); //.unwrap_or(0);

        if let Some(line) = current_line {
            x_position = line.get_width_to(x);
        }

        let position_in_grid = Position {
            col: x_position,
            row: y,
        };
        /*let position_in_grid = Position {
            col: self.location.x,
            row: self.location.y,
        };*/

        //self.location.subtract(&self.scroll_offset).into()
        position_in_grid.subtract(&self.scroll_offset)
    }
    pub fn move_text_location(&mut self, direction: &Direction) {
        let Location { mut x, mut y } = self.location;
        let Size { height, .. } = self.size;

        match direction {
            Direction::Up => {
                y = y.saturating_sub(1);
            },
            Direction::Down => {
                y = y.saturating_add(1);
            },
            Direction::Left => {
               
                //x = x.saturating_sub(1);

                if x > 0 {
                    x -= 1;
                } else if y > 0 {
                    y -= 1;
                    x = self.buffer.lines.get(y).map_or(0, Line::fragments_len);
                }

                /* if x == 0 && y > 0 {
                    y = y.saturating_sub(1); 
                    //x = std::usize::MAX;
                    x = self.get_x_position(std::usize::MAX, y)
                } else if x > 0 {
                    x = x.saturating_sub(1);
                }

                self.last_x_position = x; */
            },
            Direction::Right => {

                let width = self.buffer.lines.get(y).map_or(0, Line::fragments_len);

                if x < width {
                    x += 1;
                } else {
                    y = y.saturating_add(1);
                    x = 0;
                }
            }
            Direction::PageUp => {
                y = y.saturating_sub(height).saturating_sub(1);
            },
            Direction::PageDown => {
                y = y.saturating_add(height).saturating_sub(1);            
            },
            Direction::Home => {
                x = 0;
            },
            Direction::End => {
                x = self.buffer.lines.get(y).map_or(0, Line::fragments_len);
            },
        }

        x = self.buffer.lines.get(y).map_or(0, | line | min(line.len(), x));
        y = min(y, self.buffer.lines.len());

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
        let current_line = &self.buffer.lines.get(y);
        
        let mut col_position = 0;

        if let Some(line) = current_line {
            col_position = line.get_width_to(x);
        }

        let position_in_text = Position {
            col: col_position,
            row: y,
        };

        let Size { width, height } = self.size;
        let mut offset_changed = false;

        if position_in_text.row < self.scroll_offset.row {
            self.scroll_offset.row = position_in_text.row;
            offset_changed = true;
        } else if position_in_text.row >= self.scroll_offset.row.saturating_add(height) {
            self.scroll_offset.row = position_in_text.row.saturating_sub(height).saturating_add(1);
            offset_changed = true;
        }

        if position_in_text.col < self.scroll_offset.col {
            self.scroll_offset.col = position_in_text.col;
            offset_changed = true;
        } else if position_in_text.col >= self.scroll_offset.col.saturating_add(width) {
            self.scroll_offset.col = position_in_text.col.saturating_sub(width).saturating_add(1);
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
        
        
        let welcome_message = Line::from(&format!("{NAME} editor -- version {VERSION}"));

        let len = welcome_message.len();

        if width <= len {
            "~" .to_string();
        }

        #[allow(clippy::integer_division)]
        let padding = (width.saturating_sub(len).saturating_sub(1)) / 2;
        
        let mut full_message = format!("~{}{}", " ".repeat(padding), welcome_message.get(0..welcome_message.len()));
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
            scroll_offset: Position::default(), //Location::default(),
            // last_x_position: 0,
        }
    }
}
