use super::{
    terminal::{Size, Terminal},
    DocumentStatus,
    tooltipbar::TooltipBar,
};

pub struct StatusBar {
    pub tooltip_bar: TooltipBar,
    current_status: DocumentStatus,
}

impl StatusBar {
    pub fn new(margin_bottom: usize) -> Self {
        Self {
            current_status: DocumentStatus::default(),
            tooltip_bar: TooltipBar::new(true, margin_bottom),
        }
    }
    pub fn update_status(&mut self, new_status: DocumentStatus) {
        if new_status != self.current_status {
            self.current_status = new_status;
            let message = self.render_message();
            self.tooltip_bar.current_message = message;
            self.tooltip_bar.flag_dirty();
        }
    }
    fn render_message(&self) -> String {
        if let Ok(size) = Terminal::size() {
            let line_count = self.current_status.line_count_to_string();
            let modified_indicator = self.current_status.modified_indicator_to_string();

            let beginning = format!(
                "{} - {line_count} {modified_indicator}",
                self.current_status.file_name
            );

            let position_indicator = self.current_status.position_indicator_to_string();
            let remainder_len = size.width.saturating_sub(beginning.len());

            let status = format!("{beginning}{position_indicator:>remainder_len$}");

            if status.len() <= size.width {
                return status;
            } else {
                return String::new();
            };
        }

        String::new()

    }
    pub fn resize(&mut self, size: Size) {
        self.tooltip_bar.resize(size);
    }
    pub fn render(&mut self) {
        self.tooltip_bar.render();
    }
}
