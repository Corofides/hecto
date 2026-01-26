use super::terminal::{Size, Terminal};

#[derive(Default)]
pub struct Data {
    pub title: String,
    pub line_count: usize,
    pub position: usize,
    pub edited: bool,
}

pub struct StatusBar {
    //pub size: Size,
    needs_redraw: bool,
    pub data: Data,
}

impl StatusBar {
    pub fn render(&mut self, at: usize) {
        if !self.needs_redraw {
            // Do nothing
            //return;
        }

        let mut edited_string = String::from("");

        if self.data.edited {
            edited_string = String::from("[+]");
        }

        let status = format!("Title {}{}, Line Count {}/{}", self.data.title, edited_string, self.data.position, self.data.line_count);
            //String::from("Title: {} Line:  {}/{}", self.title, sself.line_count);
        let _ = Terminal::print_row(at, &status);

        self.needs_redraw = false;
    }
    pub fn update_data(&mut self, title: String, position: usize, line_count: usize, edited: bool) {
        let mut has_changed = false;

        if title != self.data.title {
            self.data.title = title;
            has_changed = true;
        }

        if line_count != self.data.line_count {
            self.data.line_count = line_count;
            has_changed = true;
        }

        if position != self.data.position {
            self.data.position = position;
            has_changed = true;
        }

        self.data.edited = edited;

        self.needs_redraw = has_changed;
    }
}

impl Default for StatusBar {
    fn default() -> Self {
        /* let mut size = Terminal::size().unwrap_or_default();

        size.height = 2; */
        
        Self {
            //size,
            needs_redraw: true,
            data: Data::default(),
        }
    }
}
