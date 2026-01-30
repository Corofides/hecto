use std::io::Error;

use super::{
    terminal::{Size, Terminal},
    uicomponent::UIComponent,
};

#[derive(Default, PartialEq)]
pub struct CommandPrompt {
    prompt: String,
    command: String,
}

impl CommandPrompt {
    pub fn new(prompt: &str) -> Self {
        Self {
            prompt: String::from(prompt),
            command: String::new(),
        }
    }
}

#[derive(Default)]
pub struct CommandBar {
    current_command: CommandPrompt,
    needs_redraw: bool,
    size: Size,
}

impl CommandBar {
    pub fn update_command(&mut self, new_command: CommandPrompt) {
        if self.current_command != new_command {
            self.current_command = new_command;
            self.set_needs_redraw(true);
        }
    }
}

impl UIComponent for CommandBar {
    fn set_needs_redraw(&mut self, value: bool) {
        self.needs_redraw = value;
    }
    fn needs_redraw(&self) -> bool {
        self.needs_redraw
    }
    fn set_size(&mut self, size: Size) {
        self.size = size;
    }
    fn draw(&mut self, origin_y: usize) -> Result<(), Error> {

        let to_print = format!("{} {}", self.current_command.prompt, self.current_command.command);
        Terminal::print_inverted_row(origin_y, &to_print)?;

        Ok(())
    }
}
