mod documentstatus;
mod fileinfo;
mod editorcommand;
mod terminal;
mod view;
mod statusbar;
mod messagebar;
mod uicomponent;

use crossterm::event::{ read, poll, Event, KeyEvent, KeyEventKind };
use std::{
    env, 
    io::Error,
    panic::{set_hook, take_hook}
};
use terminal::{Terminal};
use view::{View};
use statusbar::{StatusBar};
use messagebar::{MessageBar};
use documentstatus::DocumentStatus;
use editorcommand::{DisplayCommand, InsertionCommand, ControlCommand};
use uicomponent::UIComponent;
use self::{terminal::Size};
use std::time::{Duration};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct Editor {
    view: View,
    status_bar: StatusBar,
    message_bar: MessageBar,
    should_quit: bool,
    terminal_size: Size,
    title: String,
}

impl Editor {
    pub fn new() -> Result<Self, Error> {
        let current_hook = take_hook();
        set_hook(Box::new(move | panic_info | {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));
        Terminal::initialize()?;
        let mut editor = Self::default();
        let size = Terminal::size().unwrap_or_default();
        editor.resize(size);
        
        let args: Vec<String> = env::args().collect();
        if let Some(file_name) = args.get(1) {
            editor.view.load(file_name);
        }

        editor
            .message_bar
            .update_message("HELP: Ctrl-S = save | Ctrl-q = quit".to_string());

        editor.refresh_status();
        Ok(editor)
    }
    fn resize(&mut self, size: Size) {
        self.terminal_size = size;
        self.view.resize(Size {
            height: size.height.saturating_sub(2),
            width: size.width,
        });
        self.message_bar.resize(Size {
            height: 1,
            width: size.width,
        });
        self.status_bar.resize(Size {
            height: 1,
            width: size.width,
        });
    }
    // pub fn refresh_message(&mut self) {
    //     let message = String::from("HELP: Ctrl-S = save | Ctrl+Q = quit");
    //     self.message_bar.update_message(message);
    // }
    pub fn refresh_status(&mut self) {
        let status = self.view.get_status();
        let title = format!("{} - {NAME}", status.file_name);
        self.status_bar.update_status(status);

        if title != self.title && matches!(Terminal::set_title(&title), Ok(())) {
            self.title = title;
        }
    }
    pub fn run(&mut self) {
        loop {
            self.refresh_screen();
            if self.should_quit {
                break;
            }

            let has_event = poll(Duration::from_secs(0)).unwrap();

            if has_event {
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
            
            //let status = self.view.get_status();
            //self.status_bar.update_status(status);
            self.refresh_status(); //this may need to be removed.
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
           
            // Handle control commands associated with movement, saving, quiting, etc
            if let Ok(command) = ControlCommand::try_from(&event) {
                if matches!(command, ControlCommand::Quit) {
                    self.should_quit = true;
                } else {
                    self.view.handle_control_command(command);
                }
            }

            // Handle display commands currently just resize events.
            if let Ok(command) = DisplayCommand::try_from(&event) {
                let DisplayCommand::Resize(size) = command; 
                self.resize(size);
            }

            // Handle Insertion Commands.
            if let Ok(command) = InsertionCommand::try_from(&event) {
                self.view.handle_insertion_command(command);
            }


            /*if let Ok(command) = EditorCommand::try_from(event) {
                if matches!(command, EditorCommand::Quit) {
                    self.should_quit = true;
                } else {
                    self.view.handle_command(command);
                    if let DisplayCommand::Resize(size) = command {
                        self.resize(size);
                    }
                }
            } */
        }
    }
    fn refresh_screen(&mut self) {
        if self.terminal_size.height == 0 || self.terminal_size.width == 0 {
            return;
        }

        let _ = Terminal::hide_caret();

        // Only render this if the message bar is visible
        self.message_bar
            .render(self.terminal_size.height.saturating_sub(1));

        if self.terminal_size.height > 1 {
            self.status_bar
                .render(self.terminal_size.height.saturating_sub(2));
        }

        if self.terminal_size.height > 2 {
            self.view.render(0);
        }

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
