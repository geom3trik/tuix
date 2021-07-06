use unicode_segmentation::{GraphemeCursor, UnicodeSegmentation};

pub trait Editable {
    fn next_grapheme_boundary(&self, index: usize) -> Option<usize>;

    fn prev_grapheme_boundary(&self, index: usize) -> Option<usize>;

    fn next_word_boundary(&self, index: usize) -> Option<usize>;

    fn prev_word_boundary(&self, index: usize) -> Option<usize>;



}

impl Editable for String {
    fn next_grapheme_boundary(&self, index: usize) -> Option<usize> {
        let mut cursor = GraphemeCursor::new(index, self.len(), true);
        cursor.next_boundary(self, 0).unwrap()
    }

    fn prev_grapheme_boundary(&self, index: usize) -> Option<usize> {
        let mut cursor = GraphemeCursor::new(index, self.len(), true);
        cursor.prev_boundary(self, 0).unwrap()
    }

    fn next_word_boundary(&self, index: usize) -> Option<usize> {
        let mut offset = index;
        let mut passed_alphanumeric = false;
        for next_grapheme in self.get(index..)?.graphemes(true) {
            let is_alphanumeric = next_grapheme.chars().next()?.is_alphanumeric();
            if is_alphanumeric {
                passed_alphanumeric = true;
            } else if passed_alphanumeric {
                return Some(offset)
            }
            offset += next_grapheme.len();
        }

        Some(self.len())
    }

    fn prev_word_boundary(&self, index: usize) -> Option<usize> {
        let mut offset = index;
        let mut passed_alphanumeric = false;
        for next_grapheme in self.get(0..index)?.graphemes(true) {
            let is_alphanumeric = next_grapheme.chars().next()?.is_alphanumeric();
            if is_alphanumeric {
                passed_alphanumeric = true;
            } else if passed_alphanumeric {
                return Some(offset)
            }
            offset -= next_grapheme.len();
        }

        Some(self.len())
    }


}