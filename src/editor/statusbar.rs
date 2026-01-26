use super::{
    terminal::{Size, Terminal},
    DocumentStatus,
};
use crossterm::style::Attribute;

pub struct StatusBar {
    current_status: DocumentStatus,
    needs_redraw: bool,
    margin_bottom: usize,
    width: usize,
    position_y: usize,
}

impl StatusBar {
    pub fn new(margin_bottom: usize) -> Self {
        let size = Terminal::size().unwrap_or_default();
        Self {
            current_status: DocumentStatus::default(),
            needs_redraw: true,
            margin_bottom,
            width: size.width,
            position_y: size.height.saturating_sub(margin_bottom).saturating_sub(1),
        }
    }
    pub fn resize(&mut self, size: Size) {
        self.width = size.width;
        self.position_y = size.height.saturating_sub(self.margin_bottom).saturating_sub(1);
        self.needs_redraw = true;
    }
    pub fn update_status(&mut self, new_status: DocumentStatus) {
        if new_status != self.current_status {
            self.current_status = new_status;
            self.needs_redraw = true;
        }
    }
    pub fn render(&mut self) {
        if !self.needs_redraw {
            return;
        }

        let file_name = match &self.current_status.file_name {
            Some(file_name) => String::from(file_name),
            None => String::from("New Document"),
        };

        let line_count = format!("{} lines", self.current_status.total_lines);
        let mut modified_line = String::from("");

        if self.current_status.is_modified {
            modified_line = String::from("(modified)");
        }

        let position_string = format!("{}/{}", self.current_status.current_line_index, self.current_status.total_lines);
        let screen_width = self.width;
        let left_status = format!("{file_name} - {line_count} {modified_line}");
        let right_status = format!("{position_string}");
        let padding = screen_width - left_status.len(); // - right_status.len();

        let status = format!("{}{left_status}{right_status:>padding$}{}", Attribute::Reverse, Attribute::Reset, padding = padding);
        let result = Terminal::print_row(self.position_y, &status);
        debug_assert!(result.is_ok(), "Failed to render status bar");
        self.needs_redraw = false;
    }
}

