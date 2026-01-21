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
    pub fn get_width_to(&self, grapheme: usize) -> usize {

        if grapheme >= self.len {
            return self.len;
        }

        let mut index = 0;
        let mut width = 0;

        while index < grapheme && index < self.text_fragments.len() {
            width = width + self.text_fragments[index].len();
            index = index + 1;
        }

        return width;
    }
    pub fn get(&self, range: Range<usize>) -> String {

        let start = range.start;
        let end = range.end;
        let position = 0;

        let end_of_graphemes = self.text_fragments.len();
        
        let mut position = 0;
        let mut index = 0;
        let mut count = 0;

        let mut line_part_string = String::new();

        while position < end && index < self.text_fragments.len() {
            
            let text_fragment = &self.text_fragments[index];

            if position < start {
                position = position + text_fragment.len();

                if position > start {
                    line_part_string = String::from("⋯");
                    count = count + 1;
                }

                index = index + 1;
                continue;
            }
           
            position = position + text_fragment.len();

            if position <= end {
                line_part_string = format!("{line_part_string}{}", text_fragment.get_character());
            } else {
                line_part_string = format!("{line_part_string}⋯");
            }
            index = index + 1;
            count = count + 1;

        }
        
        return line_part_string;

    }
    pub fn fragments_len(&self) -> usize {
        self.text_fragments.len()
    }
    pub fn len(&self) -> usize {
        self.len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_partial_string_ascii() {
        let line = Line::from("Nailed it");

        let part = line.get(0..6);

        assert_eq!(part, "Nailed");
    }

    #[test]
    fn get_partial_string_other() {

        let line = Line::from("ＡＡＡＡＡ");

        let get_one_char = line.get(0..2);
        let get_partial_start = line.get(1..2);
        let get_partial_end = line.get(6..7);

        assert_eq!(get_one_char, "Ａ");
        assert_eq!(get_partial_start, "⋯");
        assert_eq!(get_partial_end, "⋯");
    
    }

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
