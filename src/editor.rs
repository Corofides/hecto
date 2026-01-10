use crossterm::event::{read, Event, Event::Key, KeyCode, KeyCode::*, KeyEvent, KeyModifiers};
use std::io::Error;
use std::cmp::{max, min};

mod terminal;

use terminal::{Terminal, Size, Position};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
    position: Position,
    location: Location,
}

/* pub struct Position {
    column: usize,
    row: usize,
} */

pub struct Location {
    column: usize,
    row: usize,
}
    

impl Editor {
    pub const fn default() -> Self {
        Self {
            should_quit: false,
            position: Position{ x: 0, y: 0},
            location: Location{ column: 0, row: 0},
        }
    }
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }
    pub fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event)?;
        }
        Ok(())

    }
    fn draw_empty_row() -> Result<(), Error> {
        Terminal::print("~")?;
        Ok(())
    }
    fn draw_rows() -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;
        Terminal::move_cursor_to(Position {x: 0, y: 0})?;
        
        for current_row in 0..height {
            Terminal::clear_line()?;

            #[allow(clippy::integer_division)]
            if current_row == height / 3 {
                Self::draw_welcome_message()?;
            } else {
                Self::draw_empty_row()?;
            }
            
            if current_row.saturating_add(1) < height {
                Terminal::print("\r\n")?; //print!("\r\n");
            }
        }
        Terminal::flush()?;
        Ok(())
    }
    fn draw_welcome_message() -> Result<(), Error> {

        let mut welcome_message = format!("{NAME} editor -- version {VERSION}");

        let width = Terminal::size()?.width;
        let len = welcome_message.len();

        #[allow(clippy::integer_division)]
        let padding = (width.saturating_sub(len)) / 2;

        let spaces = " ".repeat(padding.saturating_sub(1));

        welcome_message = format!("~{spaces}{welcome_message}");
        welcome_message.truncate(width);

        Terminal::print(welcome_message)?;
        Ok(())
    }
    pub fn evaluate_event(&mut self, event: &Event) -> Result<(), Error> {
        let Size { height, width} = Terminal::size()?;

        if let Key(KeyEvent {
            code, modifiers, .. 
        }) = event {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                },
                KeyCode::PageUp => {
                    self.position.y = 0;
                    self.location.row = 0;

                },
                KeyCode::PageDown => {
                    self.position.y = height - 1;
                    self.location.row += height - 1;
                },
                KeyCode::Home => {
                    self.position.x = 0;
                    self.location.row = 0;
                },
                KeyCode::End => {
                    self.position.x = width - 1;
                    self.location.row = width - 1;
                }
                KeyCode::Down => {
                    self.position.y = min(self.position.y + 1, height - 1);
                    self.location.row = min(self.location.row + 1, height - 1);
                },
                KeyCode::Up => {
                    if self.position.y > 0 {
                        self.position.y = max(0, self.position.y - 1);
                    }
                    if self.location.row > 0 {
                        self.location.row = max(0, self.location.row - 1);
                    }
                },
                KeyCode::Left => {
                    if self.position.x > 0 {
                        self.position.x = max(0, self.position.x - 1);
                    }
                    if self.location.column > 0 {
                        self.location.column = max(0, self.location.column - 1); // yeah, I know
                                                                                 // this is
                                                                                 // pointless
                    }
                },
                KeyCode::Right => {
                    self.position.x = min(self.position.x + 1, width - 1);
                    self.location.column = min(self.location.column + 1, width - 1);
                }
                _ => (),
            }
        };

        Ok(())
    }
    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(Position {x: self.position.x, y: self.position.y})?;
        }
        Terminal::show_cursor()?;
        Terminal::flush()?;
        Ok(())
    }
}

