use std::fs::read_to_string;
use std::io::Error;

use super::line::Line;
use super::Location;

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<Line>,
}

impl Buffer {
    pub fn load(file_name: &str) -> Result<Self, Error> {
        let contents = read_to_string(&file_name)?;
        let mut lines = Vec::new();
        for value in contents.lines() {
            lines.push(Line::from(value));
        }
        Ok(Self { lines })
    }
    pub fn delete(&mut self, at: Location) {
        if let Some(line) = self.lines.get(at.line_index) {
            if at.grapheme_index >= line.grapheme_count() && self.lines.len() > at.line_index.saturating_add(1) {
                let next_line = self.lines.remove(at.line_index.saturating_add(1));

                #[allow(clippy::indexing_slicing)]
                self.lines[at.line_index].append(&next_line);
            } else if at.grapheme_index < line.grapheme_count() {
                #[allow(clippy::indexing_slicing)]
                self.lines[at.line_index].delete(at.grapheme_index);
            }
        }
    }
    pub fn insert_char(&mut self, character: char, at: Location) {
        if at.line_index > self.lines.len() {
            return;
        }

        if at.line_index == self.lines.len() {
            self.lines.push(Line::from(&character.to_string()));
        } else if let Some(line) = self.lines.get_mut(at.line_index) {
            line.insert_char(character, at.grapheme_index);
        }
    }
    pub fn insert_newline(&mut self, at: Location) {
        /* let mut op_line = self.lines.get_mut(at.line_index);
        let line = op_line.get_or_insert_with(|| {
            self.lines.insert(at.line_index, Line::from(""));
            let position = self.lines.len() - 1;
            &mut self.lines[position]
        });*/

        let Some(line) = self.lines.get_mut(at.line_index) else {
            self.lines.insert(at.line_index, Line::from(""));
            let line_length = self.lines.len() - 1;
            let line = &mut self.lines[line_length];

            let grapheme_index = at.grapheme_index;
            
            line.insert_char('\n', grapheme_index);
            line.insert_char('\r', grapheme_index);
            let new_line_string = line.split_line(grapheme_index.saturating_add(2));
            self.lines.insert(at.line_index.saturating_add(1), Line::from(&new_line_string));
            return;

        };

        let grapheme_index = at.grapheme_index;
        
        line.insert_char('\n', grapheme_index);
        line.insert_char('\r', grapheme_index);
        let new_line_string = line.split_line(grapheme_index.saturating_add(2));
        self.lines.insert(at.line_index.saturating_add(1), Line::from(&new_line_string));

    }
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
    pub fn height(&self) -> usize {
        self.lines.len()
    }
}
