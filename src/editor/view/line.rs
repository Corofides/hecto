use std::{cmp, ops::Range};
use unicode_segmentation::UnicodeSegmentation;
use super::textfragment::TextFragment;

pub struct Line {
    string: String,
    text_fragments: Vec<TextFragment>,
    len: usize,
}

impl Line {
    pub fn from(line_str: &str) -> Self {

        let mut text_fragments: Vec<TextFragment> = Vec::new();
        let mut length = 0;

        for character in line_str.chars() {
            
            let fragment = TextFragment::new(character.to_string());
            length = length + fragment.len();
            text_fragments.push(fragment);

        }

        Line {
            string: String::from(line_str),
            text_fragments: text_fragments,
            len: length,
        }
    }
    fn get_graphemes(&self) -> Vec<&str> {
        return self.string.graphemes(true).collect::<Vec<&str>>();
    }
    pub fn get(&self, range: Range<usize>) -> String {
        let start = range.start;
        let graphemes = self.get_graphemes();
        let end = cmp::min(range.end, self.len());
        graphemes[start..end].concat()
        //self.string.get(start..end).unwrap_or_default().to_string()
    }
    pub fn len(&self) -> usize {
        self.len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_line_with_multiple_widths() {
        let line = Line::from("AaBb\u{200b}Ａ");

        let length = line.text_fragments.len();
        let first_char = &line.text_fragments[0];
        let last_char = &line.text_fragments[length - 1];

        assert_eq!(length, 6);
        assert_eq!(first_char.get_character(), "A");
        assert_eq!(last_char.get_character(), "Ａ");
    }

    #[test]
    fn line_length() {
        let line = Line::from("AaBb\u{200b}Ａ");
        let len = line.len();

        assert_eq!(len, 7);
    }
}
