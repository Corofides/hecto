use super::{
    terminal::{Size, Terminal},
};

pub struct TooltipBar {
    pub current_message: String,
    inverted: bool,
    needs_redraw: bool,
    margin_bottom: usize,
    width: usize,
    position_y: usize,
    is_visible: bool,
}

impl TooltipBar {
    pub fn new(inverted: bool, margin_bottom: usize) -> Self {
        let size = Terminal::size().unwrap_or_default();

        let mut tooltip_bar = Self {
            current_message: String::new(),
            inverted,
            needs_redraw: true,
            margin_bottom,
            width: size.width,
            position_y: 0,
            is_visible: false,
        };

        tooltip_bar.resize(size);
        tooltip_bar
    }
    pub fn resize(&mut self, size: Size) {
        self.width = size.width;

        let mut position_y = 0;
        let mut is_visible = false;

        if let Some(result) = size
            .height
            .checked_sub(self.margin_bottom)
            .and_then(|result| result.checked_sub(1))
        {
            position_y = result;
            is_visible = true;
        }

        self.position_y = position_y;
        self.is_visible = is_visible;
        self.needs_redraw = true;
    }
    pub fn render(&mut self) {
        if !self.needs_redraw || !self.is_visible {
            return;
        }
        
        if self.inverted {
            let result = Terminal::print_inverted_row(self.position_y, &self.current_message);
            debug_assert!(result.is_ok(), "Failed to render bar");
            return;
        }

        let result = Terminal::print_row(self.position_y, &self.current_message);
        debug_assert!(result.is_ok(), "Failed to render bar");
    }
    pub fn flag_dirty(&mut self) {
        self.needs_redraw = true;
    }
}
