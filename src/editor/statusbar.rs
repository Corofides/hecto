use super::terminal::{Size, Terminal};

pub struct StatusBar {
    pub size: Size,
    needs_redraw: bool,
}

impl StatusBar {
    pub fn render(&mut self, at: usize) {
        if !self.needs_redraw {
            // Do nothing
            return;
        }

        let status = String::from("Status Bar Text");
        let result = Terminal::print_row(at, &status);

        self.needs_redraw = false;
    }
}

impl Default for StatusBar {
    fn default() -> Self {
        let mut size = Terminal::size().unwrap_or_default();

        size.height = 2;
        
        Self {
            size,
            needs_redraw: true,
        }
    }
}
