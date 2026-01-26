use crossterm::event::{ read, Event, KeyEvent, KeyEventKind };
use std::{
    env, 
    io::Error,
    panic::{set_hook, take_hook}
};
mod editorcommand;
mod terminal;
mod view;
mod statusbar;

use terminal::{Terminal, Size};
use view::{View};
use statusbar::{StatusBar};

use editorcommand::EditorCommand;

//#[derive(Default)]
pub struct Editor {
    view: View,
    status_bar: StatusBar,
    should_quit: bool,
}
   
impl Default for Editor {
    fn default() -> Self {
        Self {
            view: View::default(),
            status_bar: StatusBar::default(),
            should_quit: false,
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
        let mut status_bar = StatusBar::default();
        let args: Vec<String> = env::args().collect();
        if let Some(file_name) = args.get(1) {
            view.load(file_name);
        }
        status_bar.update_data(
            String::from(view.get_title()),
            view.get_line(),
            view.get_line_count(),
            false,
        );
        Ok(Self {
            should_quit: false,
            view,
            status_bar,
        })
    }
    pub fn update_status_bar(&mut self) {
        self.status_bar.update_data(
            String::from(self.view.get_title()), 
            self.view.get_line(), 
            self.view.get_line_count(),
            self.view.has_edited(),
        );
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
            self.update_status_bar();
        }
    }
    #[allow(clippy::needless_pass_by_value)]
    pub fn evaluate_event(&mut self, event: Event) {
        let should_process = match &event {
            Event::Key(KeyEvent { kind, .. }) => kind == &KeyEventKind::Press,
            Event::Resize(_, _) => true,
            _ => false,
        };

        if should_process {
            if let Ok(command) = EditorCommand::try_from(event) {
                if matches!(command, EditorCommand::Quit) {
                    self.should_quit = true;
                } else {
                    self.view.handle_command(command);
                }
            }
        }
    }
    fn refresh_screen(&mut self) {
        let Size { height, .. } = Terminal::size().expect("Size not found");

        let _ = Terminal::hide_caret();
        self.view.render();
        self.status_bar.render(height - 2);
        let _ = Terminal::move_caret_to(self.view.caret_position());
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
