use std::io::Error;

use std::time::{Instant};

use super::{
    terminal::{Size, Terminal},
    uicomponent::UIComponent,
};

pub struct MessageBar {
    current_message: String,
    needs_redraw: bool,
    pub instant: Instant,
}

impl MessageBar {
    pub fn update_message(&mut self, new_message: String) {
        if new_message != self.current_message {
            self.current_message = new_message;
            self.mark_redraw(true);
            self.instant = Instant::now();
        }
        
    }
}

impl Default for MessageBar {
    fn default() -> Self {
        Self {
            current_message: String::new(),
            needs_redraw: false,
            instant: Instant::now(),
        }
    }
}

impl UIComponent for MessageBar {
    fn mark_redraw(&mut self, value: bool) {
        self.needs_redraw = value;
    }
    fn needs_redraw(&self) -> bool {
        self.needs_redraw
    }
    fn set_size(&mut self, _: Size) {}
    fn draw(&mut self, origin: usize) -> Result<(), Error> {
        Terminal::print_row(origin, &format!("{}", self.current_message))
    }
}
