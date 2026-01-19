use unicode_width::UnicodeWidthStr;

#[derive(Debug, PartialEq)]
pub enum GraphemeWidth {
    Half,
    Full,
}

#[derive(Debug)]
pub struct TextFragment {
    grapheme: String,
    rendered_width: GraphemeWidth,
    replacement: Option<char>,
}

impl TextFragment {

    pub fn get_character(&self) -> String {

        if let Some(character) = self.replacement {
            return character.to_string();
        }

        self.grapheme.clone()
    }
    pub fn new(grapheme: String) -> Self {

        let mut replacement = None;
        let width = grapheme.width();
        let grapheme_width = match width {
            0 => { 
                replacement = Some('·');
                GraphemeWidth::Half
            },
            1 => { GraphemeWidth::Half },
            2 => { GraphemeWidth::Full },
            _ => { panic!("Not valid!") }
        };

        Self {
            grapheme,
            rendered_width: grapheme_width,
            replacement,
        }

    }

    pub fn len(&self) -> usize {
        match self.rendered_width {
            GraphemeWidth::Full => 2,
            GraphemeWidth::Half => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_character() {
        let text_fragment = TextFragment::new(String::from("A"));

        let character = text_fragment.grapheme;
        let width = text_fragment.rendered_width;

        assert_eq!(character, "A");
        assert_eq!(width, GraphemeWidth::Half);
        assert_eq!(text_fragment.replacement, None);
    }

    #[test]
    fn zero_width_character() {
        let text_fragment = TextFragment::new(String::from("\u{200b}"));

        println!("{:?}", text_fragment);
        println!("{:?}", text_fragment.replacement);

        assert_eq!(text_fragment.grapheme, "\u{200b}");
        assert_eq!(text_fragment.rendered_width, GraphemeWidth::Half);
        assert_eq!(text_fragment.replacement, Some('·'));
    }

    #[test]
    fn full_width_character() {
        let text_fragment = TextFragment::new(String::from("Ａ"));

        assert_eq!(text_fragment.grapheme, "Ａ");
        assert_eq!(text_fragment.rendered_width, GraphemeWidth::Full);
        assert_eq!(text_fragment.replacement, None);

    }
}
