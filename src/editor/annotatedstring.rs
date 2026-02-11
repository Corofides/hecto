
#[derive(Default, Clone, Copy, Debug)]
pub enum AnnotationType {
    #[default]
    None,
    Highlight,
}

#[derive(Default, Clone)]
pub struct AnnotatedFragment {
    pub string: String,
    pub annotation_type: AnnotationType,
}

impl AnnotatedFragment {
    pub fn new(string: &str, annotation_type: AnnotationType) -> Self {
        Self {
            string: String::from(string),
            annotation_type,
        }
    }
}

#[derive(Default, Clone, Debug)]
pub struct Annotation {
    pub start_byte_idx: usize,
    pub end_byte_idx: usize,
    pub annotation_type: AnnotationType,
}

impl Annotation {
    pub fn new(start_byte_idx: usize, end_byte_idx: usize, annotation_type: AnnotationType) -> Self {
        Self {
            start_byte_idx,
            end_byte_idx,
            annotation_type,
        }
    }
}

#[derive(Default, Clone)]
pub struct AnnotatedString {
    pub string: String,
    pub annotations: Vec<Annotation>,
}

impl AnnotatedString {
    pub fn new(string: &str) -> Self {
        AnnotatedString {
            string: String::from(string),
            annotations: vec![],
        }
    }

    pub fn get_annotated_fragments(&self) -> Vec<AnnotatedFragment> {
        let mut annotated_fragments = vec![];

        //debug_assert!(false, "{:?}", self.annotations);

        for annotation in &self.annotations {
            debug_assert!(annotation.start_byte_idx < self.string.len());
            debug_assert!(annotation.end_byte_idx != 77, "{:?}", annotation);
            annotated_fragments.push(AnnotatedFragment::new(
                &self.string[annotation.start_byte_idx..=annotation.end_byte_idx],
                annotation.annotation_type,
            ));
        }

        annotated_fragments
    }

    pub fn add_annotation(&mut self, annotation: Annotation) {
        debug_assert!(annotation.start_byte_idx < self.string.len());
        debug_assert!(
            annotation.end_byte_idx >= annotation.start_byte_idx,
            "{}, {}",
            annotation.start_byte_idx, annotation.end_byte_idx,
        );
        self.annotations.push(annotation);
    }
    pub fn get_display_string(&self) -> &str {
        &self.string
    }

}
