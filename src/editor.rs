use crossterm::event::{read, 
    Event::{self, Key, Resize}, 
    KeyCode, KeyCode::Char, KeyEvent, KeyModifiers
};
use std::{env, io::Error};
use std::cmp::{ min };

mod terminal;
mod view;

use terminal::{Terminal, Size, Position};
use view::{View};

// const NAME: &str = env!("CARGO_PKG_NAME");
// const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct Editor {
    view: View,
    should_quit: bool,
    position: Position,
    location: Location,
}

/* pub struct Position {
    column: usize,
    row: usize,
} */

#[derive(Copy, Clone, Default)]
pub struct Location {
    column: usize,
    row: usize,
}
    

impl Editor {
    pub fn run(&mut self) {

        Terminal::initialize().unwrap();
        self.handle_args();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }
    fn handle_args(&mut self) {
        let args: Vec<String> = env::args().collect();
        if let Some(file_name) = args.get(1) {
            self.view.load(file_name);
        }
        self.view.needs_redraw = true;
    }
    fn move_point(&mut self, key_code: KeyCode) -> Result< (), Error> {
        let Position { mut x, mut y } = self.position;
        let Location { mut column, mut row } = self.location;

        let Size { height, width } = Terminal::size()?;

        match key_code {
            KeyCode::Up => {
                y = y.saturating_sub(1);
                row = row.saturating_sub(1);
            }
            KeyCode::Down => {
                y = min(height.saturating_sub(1), y.saturating_add(1));
                row = min(height.saturating_sub(1), row.saturating_add(1));
            },
            KeyCode::Left => {
                x = x.saturating_sub(1);
                column = column.saturating_sub(1);
            }
            KeyCode::Right => {
                x = min(width.saturating_sub(1), x.saturating_add(1));
                column = min(width.saturating_sub(1), column.saturating_add(1));
            }
            KeyCode::PageUp => {
                y = 0;
                row = 0;
            }
            KeyCode::PageDown => {
                y = height.saturating_sub(1);
                row = height.saturating_sub(1);
            }
            KeyCode::Home => {
                x = 0;
                column = 0;
            }
            KeyCode::End => {
                x = width.saturating_sub(1);
                column = width.saturating_sub(1);
            }
            _ => (),
        }
        self.location = Location { column, row };
        self.position = Position { x, y };
        Ok(())
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
    pub fn evaluate_event(&mut self, event: &Event) -> Result<(), Error> {
        if let Key(KeyEvent {
            code, modifiers, .. 
        }) = event {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                },
                KeyCode::Up |
                KeyCode::Down |
                KeyCode::Left |
                KeyCode::Right |
                KeyCode::PageDown |
                KeyCode::PageUp |
                KeyCode::Home |
                KeyCode::End => {
                    self.move_point(*code)?;
                },
                _ => (),
            }
        }

        if let Resize(_width, _height) = event {
            self.view.needs_redraw = true;
        }

        Ok(())
    }
    fn refresh_screen(&mut self) -> Result<(), Error> {
        Terminal::hide_caret()?;
        Terminal::move_caret_to(Position::default())?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
        } else {
            self.view.render()?;
            Terminal::move_caret_to(Position {
                x: self.position.x, 
                y: self.position.y
            })?;
        }
        Terminal::show_caret()?;
        Terminal::execute()?;
        Ok(())
    }
}

