use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use std::{
    env, 
    io::Error,
    panic::{set_hook, take_hook}
};
use std::cmp::{ min };

mod terminal;
mod view;

use terminal::{Terminal, Size, Position};
use view::{View, Move};

// const NAME: &str = env!("CARGO_PKG_NAME");
// const VERSION: &str = env!("CARGO_PKG_VERSION");

//#[derive(Default)]
pub struct Editor {
    view: View,
    should_quit: bool,
    position: Position,
}

/* pub struct Position {
    column: usize,
    row: usize,
} */

/*#[derive(Copy, Clone, Default)]
pub struct Location {
    column: usize,
    row: usize,
}*/
   
impl Default for Editor {
    fn default() -> Self {
        Self {
            view: View::default(),
            should_quit: false,
            position: Position::default(),
        }
    }
}

impl Editor {
    pub fn new() -> Result<Self, Error> {
        let current_hook = take_hook();
        set_hook(Box::new(move | panic_info | {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));
        Terminal::initialize()?;
        let mut view = View::default();
        let args: Vec<String> = env::args().collect();
        if let Some(file_name) = args.get(1) {
            view.load(file_name);
        }
        Ok(Self {
            should_quit: false,
            position: Position::default(),
            view
        })
    }
    pub fn run(&mut self) {
        loop {
            self.refresh_screen();
            if self.should_quit {
                break;
            }
            match read() {
                Ok(event) => self.evaluate_event(event),
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not read event: {err:?}");
                    }
                }
            }
        }
    }
    fn move_point(&mut self, key_code: KeyCode) {
        let Position { mut col, mut row } = self.position;

        let Size { height, width } = Terminal::size().unwrap_or_default();

        match key_code {
            KeyCode::Up => {
                self.view.move_view(Move::Up);
                row = col.saturating_sub(1);
            }
            KeyCode::Down => {
                self.view.move_view(Move::Down);
                row = min(height.saturating_sub(1), row.saturating_add(1));
            },
            KeyCode::Left => {
                self.view.move_view(Move::Left);
                row = row.saturating_sub(1);
            }
            KeyCode::Right => {
                self.view.move_view(Move::Right);
                col = min(width.saturating_sub(1), col.saturating_add(1));
            }
            KeyCode::PageUp => {
                self.view.move_view(Move::PageUp);
                row = 0;
            }
            KeyCode::PageDown => {
                self.view.move_view(Move::PageDown);
                row = height.saturating_sub(1);
            }
            KeyCode::Home => {
                self.view.move_view(Move::StartOfRow);
                col = 0;
            }
            KeyCode::End => {
                self.view.move_view(Move::EndOfRow);
                col = width.saturating_sub(1);
            }
            _ => (),
        }
        self.position = Position { col, row };
    }
    #[allow(clippy::needless_pass_by_value)]
    pub fn evaluate_event(&mut self, event: Event) {
        match event {
            Event::Key(KeyEvent {
                code,
                modifiers,
                ..
            }) => match (code, modifiers) {
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => {
                    self.should_quit = true;
                },
                (
                    KeyCode::Up |
                    KeyCode::Down |
                    KeyCode::Left |
                    KeyCode::Right |
                    KeyCode::PageDown |
                    KeyCode::PageUp |
                    KeyCode::Home |
                    KeyCode::End,
                    _,
                ) => {
                    self.move_point(code);
                },
                _ => (),

            },
            Event::Resize(width_u16, height_u16) => {

                #[allow(clippy::as_conversion)]
                let width = width_u16 as usize;
                
                #[allow(clippy::as_conversion)]
                let height = height_u16 as usize;

                self.view.resize(Size {
                    width,
                    height,
                });
            }
            _ => {}
        }
    }
    fn refresh_screen(&mut self) {
        let _ = Terminal::hide_caret();
        let view_position = self.view.get_position();
        let scroll_offset = self.view.get_scroll_offset();
        self.view.render();
        let _ = Terminal::move_caret_to(Position {
            col: view_position.col - scroll_offset.col,
            row: view_position.row - scroll_offset.row,
        });
        let _ = Terminal::show_caret();
        let _ = Terminal::execute();
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        let _ = Terminal::terminate();
        if self.should_quit {
            let _ = Terminal::print("Goodbye.\r\n");
        }
    }
}
