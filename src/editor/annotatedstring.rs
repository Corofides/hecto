
#[derive(Default, Clone)]
pub enum AnnotationType {
    #[default]
    None,
    Highlight,
}

#[derive(Default, Clone)]
pub struct Annotation {
    pub start_byte_idx: usize,
    pub end_byte_idx: usize,
    pub annotation_type: AnnotationType,
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

    pub fn add_annotation(&mut self, annotation: Annotation) {
        self.annotations.push(annotation);
    }
    pub fn get_display_string(&self) -> &str {
        &self.string
    }

}
