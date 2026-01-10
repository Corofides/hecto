use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use std::io::Error;

mod terminal;

use terminal::{Terminal, Size, Position};

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Self {should_quit: false} 
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
            self.evaluate_event(&event);
        }
        Ok(())

    }
    fn draw_rows() -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;
        
        for current_row in 0..height {
            Terminal::clear_line()?;
            Terminal::print("~")?;
            
            if current_row + 1 < height {
                Terminal::print("\r\n")?; //print!("\r\n");
            }
        }
        Terminal::flush()?;
        Ok(())
    }
    fn draw_welcome() -> Result<(), Error> {

        let Size { height, width } = Terminal::size()?;
        let welcome_message = String::from("Welcome to Hecto");
        let version = String::from("Version: 0.0.0");

        #[allow(clippy::cast_possible_truncation)]
        let welcome_length = welcome_message.len() as u16;
        #[allow(clippy::cast_possible_truncation)]
        let version_length = version.len() as u16;

        let half_width = width / 2;
        let half_height = height / 2;

        let rows = 3;

        Terminal::move_cursor_to(Position {x: half_width - (welcome_length / 2), y: half_height - (rows / 2)})?;
        Terminal::print(&welcome_message[..])?; // print slice of welcome message.
        
        Terminal::move_cursor_to(Position {x: half_width - (version_length / 2), y: half_height + (rows / 2)})?;
        Terminal::print(&version[..])?; // print slice of version number.

        Terminal::flush()?;
        Ok(())
    }
    pub fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, .. 
        }) = event {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                },
                _ => (),
            }
        }
    }
    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
        } else {
            Self::draw_rows()?;
            Self::draw_welcome()?;
            Terminal::move_cursor_to(Position {x: 0, y: 0})?;
        }
        Terminal::show_cursor()?;
        Terminal::flush()?;
        Ok(())
    }
}

