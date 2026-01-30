use std::io::Error;

use super::{
    command::{Edit, Move},
    terminal::{Position, Size, Terminal},
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
    pub fn caret_position(&self, origin_y: usize) -> Position {
        Position {
            row: origin_y,
            col: self.current_command.prompt.len() + self.current_command.command.len() + 1,
        }
    }
    pub fn handle_edit_command(&mut self, command: Edit) {
        match command {
            Edit::Insert(character) => {
                let CommandPrompt { prompt, command } = &self.current_command;

                let new_command = format!("{}{}", &command, character);
                self.update_command(CommandPrompt {
                    prompt: prompt.to_string(),
                    command: new_command,
                });
            },
            Edit::Delete => { 
                // self.delete(),
            },
            Edit::DeleteBackward => {
                // self.delete_backward(),
            }
            Edit::InsertNewLine => {},
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
