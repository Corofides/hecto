mod documentstatus;
mod fileinfo;
mod command;
mod terminal;
mod view;
mod statusbar;
mod messagebar;
mod commandbar;
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
use documentstatus::DocumentStatus;
use uicomponent::UIComponent;
use std::time::{Duration};

use self::{
    command::{
        Command::{self, Edit, Move, System},
        System::{Quit, Resize, Save},
    },
    messagebar::MessageBar,
    commandbar::{CommandBar, CommandPrompt},
    terminal::Size,
};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

const QUIT_TIMES: u8 = 3;

#[derive(Default)]
pub struct Editor {
    view: View,
    status_bar: StatusBar,
    command_bar: CommandBar,
    message_bar: MessageBar,
    should_quit: bool,
    terminal_size: Size,
    title: String,
    quit_times: u8,
    show_command_prompt: bool,
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
        
        editor.update_message("HELP: Ctrl-S = save | Ctrl-q = quit");

        let args: Vec<String> = env::args().collect();
        if let Some(file_name) = args.get(1) {
            if editor.view.load(file_name).is_err() {
                editor.update_message("ERR: Could not open file: {file_name}");
            }
        }

        editor.refresh_status();
        Ok(editor)
    }
    fn update_message(&mut self, message: &str) {
        self
            .message_bar
            .update_message(message);
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
    pub fn evaluate_event(&mut self, event: Event) {
        let should_process = match &event {
            Event::Key(KeyEvent { kind, .. }) => kind == &KeyEventKind::Press,
            Event::Resize(_, _) => true,
            _ => false,
        };

        if should_process {
            if let Ok(command) = Command::try_from(event) {
                self.process_command(command);
            }
        }
    }
    fn process_command(&mut self, command: Command) {
        match command {
            System(Quit) => self.handle_quit(),
            System(Resize(size)) => self.resize(size),
            _ => self.reset_quit_times(),
        }

        match command {
            System(Quit | Resize(_)) => {},
            System(Save) => {
                self.add_save_prompt();
                //self.handle_save()
            },
            Edit(edit_command) => {
                if self.show_command_prompt {
                    self.command_bar.handle_edit_command(edit_command);
                    return;
                }

                self.view.handle_edit_command(edit_command)
            }
            Move(move_command) => self.view.handle_move_command(move_command),
        }
    }
    fn add_save_prompt(&mut self) {
        self.show_command_prompt = true;

        let new_command_prompt = CommandPrompt::new("Save As:");

        self.command_bar.update_command(new_command_prompt);
    }
    fn handle_save(&mut self) {
        if self.view.save().is_ok() {
            self.message_bar.update_message("File saved successfully.");
        } else {
            self.message_bar.update_message("Error writing file!");
        }
    }
    #[allow(clippy::arithmetic_side_effects)]
    fn handle_quit(&mut self) {
        if !self.view.get_status().is_modified || self.quit_times + 1 == QUIT_TIMES {
            self.should_quit = true;
        } else if self.view.get_status().is_modified {
            self.message_bar.update_message(&format!(
                "WARNING! File has unsaved changes. Press Ctrl-Q {} more times to quit.",
                QUIT_TIMES - self.quit_times - 1
            ))
        }

        self.quit_times += 1;
    }
    fn reset_quit_times(&mut self) {
        if self.quit_times > 0 {
            self.quit_times = 0;
            self.message_bar.update_message("");
        }
    }
    fn refresh_screen(&mut self) {
        if self.terminal_size.height == 0 || self.terminal_size.width == 0 {
            return;
        }

        let _ = Terminal::hide_caret();
        let status_bar_position = self
            .terminal_size
            .height
            .saturating_sub(1);

        // Only render this if the message bar is visible
        //
        if self.show_command_prompt {
            self.command_bar
                .render(status_bar_position);
        } else {
            self.message_bar
                .render(status_bar_position);
        }
        
        if self.terminal_size.height > 1 {
            self.status_bar
                .render(self.terminal_size.height.saturating_sub(2));
        }

        if self.terminal_size.height > 2 {
            self.view.render(0);
        }
        
        if self.show_command_prompt {
            let _ = Terminal::move_caret_to(self.command_bar.caret_position(status_bar_position));
        } else {
            let _ = Terminal::move_caret_to(self.view.caret_position());
        }
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
