//use std::io::{self, Read};

use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size};
use crate::terminal::Terminal;

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Editor{should_quit: false} // default constructor, returns an editor. I guess we can skip this if we want.
    }
    pub fn run(&mut self) {
        Self::initialize().unwrap();
        let result = self.repl();
        Self::terminate().unwrap();
        result.unwrap();
    }
    fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::draw_rows()
    }
    fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()
    }
    fn clear_screen() -> Result<(), std::io::Error> {
        Terminal::clear_screen()
        //Ok(())
        // let mut stdout = stdout();
        // execute!(stdout, Clear(ClearType::All))
    }
    fn draw_rows() -> Result<(), std::io::Error> {
        let size = size(); // Get the size of the terminal.
        let height = &size.unwrap().1;
        let mut index = 0;
        while index < *height {
            // execute!(stdout(), MoveTo(0, index))?;
            Terminal::move_to(0, index)?;
            print!("~");
            index += 1;
        }
        Terminal::move_to(2, 0)?;
        Ok(())
        //execute!(stdout(), MoveTo(2, 0))
    }
    pub fn repl(&mut self) -> Result<(), std::io::Error> {
        // use crossterm for input instead of stdin,
        // Editor::draw_rows()?;
        loop {
            let event = read()?;
            self.evaluate_event(&event);
            
            self.refresh_screen()?;

            if self.should_quit {
                break;
            }
        }
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
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.should_quit {
            Self::clear_screen()?;
            print!("Goodbye.\r\n");
        }
        Ok(())
    }
}

