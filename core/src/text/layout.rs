use std::ops::Range;


#[derive(Debug, Default)]
pub struct TextLayout {
    text: String,
    lines: Vec<Range<usize>>,
}

impl TextLayout {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            lines: Vec::new(),
        }
    }
}