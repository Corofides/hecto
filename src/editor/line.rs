use std::{
    fmt,
    ops::{Deref, Range},
};

use super::annotatedstring::{AnnotatedString, AnnotationType, Annotation};

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

type GraphemeIdx = usize;
type ByteIdx = usize;

#[derive(Copy, Clone, Debug)]
pub enum GraphemeWidth {
    Half,
    Full,
}

impl GraphemeWidth {
    const fn saturating_add(self, other: usize) -> usize {
        match self {
            Self::Half => other.saturating_add(1),
            Self::Full => other.saturating_add(2),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TextFragment {
    grapheme: String,
    rendered_width: GraphemeWidth,
    replacement: Option<char>,
    start_byte_idx: usize,
}

#[derive(Default, Clone)]
pub struct Line {
    fragments: Vec<TextFragment>,
    //annotated_string: AnnotatedString,
    string: String,
}

impl Line {
    pub fn from(line_str: &str) -> Self {
        debug_assert!(line_str.is_empty() || line_str.lines().count() == 1);
        let fragments = Self::str_to_fragments(line_str);

        let line_str = line_str.replace('\u{200D}', "");
        Self { 
            fragments,
            string: String::from(line_str),
            //annotated_string: AnnotatedString::new(line_str),
        }
    }
    fn str_to_fragments(line_str: &str) -> Vec<TextFragment> {

        let mut fixed_string = String::from(line_str);

        fixed_string = fixed_string.replace('\u{200D}', "");

        fixed_string 
            .grapheme_indices(false)
            .map(|(byte_idx, grapheme)| {
                let (replacement, rendered_width) = Self::get_replacement_character(grapheme)
                    .map_or_else(
                        || {
                            let unicode_width = grapheme.width();
                            let rendered_width = match unicode_width {
                                0 | 1 => GraphemeWidth::Half,
                                2 => GraphemeWidth::Full,
                                _ => {
                                    GraphemeWidth::Full
                                }
                            };
                            (None, rendered_width)
                        },
                        |replacement| (Some(replacement), GraphemeWidth::Half),
                    );
                TextFragment {
                    grapheme: grapheme.to_string(),
                    rendered_width,
                    replacement,
                    start_byte_idx: byte_idx,
                }
            })
            .collect()
    }
    fn rebuild_fragments(&mut self) {
        self.fragments = Self::str_to_fragments(&self.string);
    }
    fn get_replacement_character(for_str: &str) -> Option<char> {
        let width = for_str.width();
        match for_str {
            " " => None,
            "\t" => Some(' '),
            _ if width > 0 && for_str.trim().is_empty() => Some('␣'),
            _ if width == 0 => {
                let mut chars = for_str.chars();
                if let Some(ch) = chars.next() {
                    if ch.is_control() && chars.next().is_none() {
                        return Some('▯');
                    }
                }
                Some('·')
            },
            _ => {
                let mut chars = for_str.chars();
                if let Some(ch) = chars.next() {
                    if ch.is_control() && chars.next().is_none() {
                        return Some('▯');
                    }
                }
                None
            }
        }
    }
    pub fn delete(&mut self, at: usize) {
        debug_assert!(at <= self.grapheme_count());
        if let Some(fragment) = self.fragments.get(at) {
            let start = fragment.start_byte_idx;
            let end = fragment
                .start_byte_idx
                .saturating_add(fragment.grapheme.len());

            self.string.drain(start..end);

            self.rebuild_fragments();
        }
    }
    pub fn delete_last(&mut self) {
        self.delete(self.grapheme_count().saturating_sub(1));
    }
    pub fn append(&mut self, other: &Self) {
        self.string.push_str(&other.string);
        self.rebuild_fragments();
    }
    pub fn insert_char(&mut self, character: char, at: usize) {
        debug_assert!(at.saturating_sub(1) <= self.grapheme_count());
        if let Some(fragment) = self.fragments.get(at) {
            self.string.insert(fragment.start_byte_idx, character);
        } else {
            self.string.push(character);
        }

        self.rebuild_fragments();
    }
    pub fn append_char(&mut self, character: char) {
        self.insert_char(character, self.grapheme_count());
    }
    pub fn split(&mut self, at: usize) -> Self {
        if let Some(fragment) = self.fragments.get(at) {
            let remainder = self.string.split_off(fragment.start_byte_idx);
            self.rebuild_fragments();
            Self::from(&remainder)
        } else {
            Self::default()
        }
    }
    pub fn get_visible_graphemes(&self, range: Range<GraphemeIdx>) -> String {
        if range.start >= range.end {
            return String::new();
        }

        let mut result = String::new();
        let mut current_pos = 0;

        for fragment in &self.fragments {
            let fragment_end = fragment.rendered_width.saturating_add(current_pos);
            if current_pos >= range.end {
                break
            }
            if fragment_end > range.start {
                if fragment_end > range.end || current_pos < range.start {
                    result.push('⋯');
                } else if let Some(char) = fragment.replacement {
                    result.push(char);
                } else {
                    result.push_str(&fragment.grapheme);
                }
            }

            current_pos = fragment_end;

        }

        result

    }
    pub fn grapheme_count(&self) -> GraphemeIdx {
        self.fragments.len()
    }
    pub fn width_until(&self, grapheme_index: GraphemeIdx) -> GraphemeIdx {
        self.fragments
            .iter()
            .take(grapheme_index)
            .map(|fragment| match fragment.rendered_width {
                GraphemeWidth::Half => 1,
                GraphemeWidth::Full => 2,
            })
            .sum()
    }
    pub fn width(&self) -> GraphemeIdx {
        self.width_until(self.grapheme_count())
    }
    fn byte_idx_to_grapheme_idx(&self, byte_idx: ByteIdx) -> GraphemeIdx {
        debug_assert!(byte_idx <= self.string.len());
        self.fragments
            .iter()
            .position(|fragment| fragment.start_byte_idx >= byte_idx)
            .map_or_else(|| {
                #[cfg(debug_assertions)]
                {
                    panic!("Fragment not found for byte index: {byte_idx:?}");
                }
                #[cfg(not(debug_assertions))]
                {
                    0
                }
            }, |grapheme_idx| grapheme_idx)
    }
    fn grapheme_idx_to_byte_idx(&self, grapheme_idx: GraphemeIdx) -> ByteIdx {
        debug_assert!(grapheme_idx <= self.grapheme_count());
        if grapheme_idx == 0 || self.grapheme_count() == 0 {
            return 0;
        }
        self.fragments
            .get(grapheme_idx)
            .map_or_else(|| {
                #[cfg(debug_assertions)]
                {
                    panic!("Fragment not found for grapheme index: {grapheme_idx:?}");
                }
                #[cfg(not(debug_assertions))]
                {
                    0
                }
            }, |fragment| fragment.start_byte_idx)
    }
    pub fn get_annotated_visible_substr(&self, range: Range<GraphemeIdx>, query: &str) -> AnnotatedString {
        let sub_str = self.get_visible_graphemes(range);
        //let sub_str_fragments = Self::string_to_fragments();

        let mut annotated_string = AnnotatedString::new(&sub_str);
        let search_results = self.search(&query);

        let mut last_index = 0;
        
        for (index, fragment) in self.fragments.iter().enumerate() {
            log::debug!("Fragment {index}: {:?}", fragment);
        }
        for annotation in search_results {

            let annotation_byte_idx = self.grapheme_idx_to_byte_idx(annotation);

            if last_index < annotation_byte_idx {
                annotated_string.add_annotation(Annotation::new(
                    last_index,
                    annotation_byte_idx,
                    AnnotationType::None
                ));
            }

            // we add an annotation to take care of the current bit.
            annotated_string.add_annotation(Annotation::new(
                annotation_byte_idx, 
                annotation_byte_idx + query.len(),
                AnnotationType::Highlight
            ));
            
            last_index = annotation_byte_idx + query.len();
        }

        if last_index < sub_str.len().saturating_sub(1) {
            annotated_string.add_annotation(Annotation::new(
                last_index,
                sub_str.len().saturating_sub(1), // - last_index).saturating_sub(1), //query.len() - 1,
                AnnotationType::None
            ));
        }

        annotated_string
    }
    pub fn search(&self, query: &str) -> Vec<GraphemeIdx> {
        self.string
            .match_indices(query)
            .map(|(index, _)| self.byte_idx_to_grapheme_idx(index))
            .collect()
    }
    pub fn search_forward(&self, query: &str, from_grapheme_idx: GraphemeIdx) -> Option<GraphemeIdx> {
        debug_assert!(from_grapheme_idx <= self.grapheme_count());
        if from_grapheme_idx == self.grapheme_count() {
            return None;
        }

        let start_byte_idx = self.grapheme_idx_to_byte_idx(from_grapheme_idx);

        self.string
            .get(start_byte_idx..)
            .and_then(|substr| substr.find(query))
            .map(|byte_idx| self.byte_idx_to_grapheme_idx(byte_idx.saturating_add(start_byte_idx)))
    }
    pub fn search_backward(&self, query: &str, from_grapheme_idx: GraphemeIdx) -> Option<GraphemeIdx> {
        debug_assert!(from_grapheme_idx <= self.grapheme_count());

        if from_grapheme_idx == 0 {
            return None;
        }

        let end_byte_index = if from_grapheme_idx == self.grapheme_count() {
            self.string.len()
        } else {
            self.grapheme_idx_to_byte_idx(from_grapheme_idx)
        };

        self.string
            .get(..end_byte_index)
            .and_then(|substr| substr.match_indices(query).last())
            .map(|(index, _)| self.byte_idx_to_grapheme_idx(index))

    }
        
}

impl fmt::Display for Line {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.string)
    }
}

impl Deref for Line {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.string
    }
}

