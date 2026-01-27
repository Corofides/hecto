use super::{
    terminal::{Size},
    tooltipbar::TooltipBar,
};



pub struct MessageBar {
    pub tooltip_bar: TooltipBar,
}

impl MessageBar {
    pub fn new(margin_bottom: usize) -> Self {
        Self {
            tooltip_bar: TooltipBar::new(false, margin_bottom),
        }
    }
    pub fn update_message(&mut self, new_message: String) {
        if new_message != self.tooltip_bar.current_message {
            self.tooltip_bar.current_message = new_message;
            self.tooltip_bar.flag_dirty();
        }
    }
    pub fn resize(&mut self, size: Size) {
        self.tooltip_bar.resize(size);
    }
    pub fn render(&mut self) {
        self.tooltip_bar.render();
    }
}
