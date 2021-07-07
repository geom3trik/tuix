
use crate::geometry::{Point, Rect};

use std::borrow::Cow;
use std::ops::Range;

pub mod editable;
pub use editable::*;

pub mod selection;
pub use selection::*;

pub mod movement;
pub use movement::*;

pub mod fsm;
pub use fsm::*;


pub trait TextHandler {
    fn selection(&self) -> Selection;

    fn set_selection(&mut self, selection: Selection);

    fn is_char_boundary(&self, index: usize) -> bool;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn slice(&self, range: Range<usize>) -> Cow<str>;

    fn replace_range(&mut self, range: Range<usize>, text: &str);

    // fn hit_test_point(&self, point: Point) -> usize;
    // fn line_range(&self, index: usize, affinity: Affinity) -> Range<usize>;
    // fn bounding_box(&self) -> Option<Rect>;
    // fn slice_bounding_box(&self, range: Range<usize>) -> Option<Rect>;

}


// #[derive(Debug, Default)]
// pub struct TextState {
//     text: String,
//     selection: Selection,
// }

// impl TextState {
//     pub fn new(text: &str) -> Self {
//         Self {
//             text: text.to_owned(),
//             selection: Selection::new(),
//         }
//     }
// }

// Random ideas for text handling

// text string
// split into text runs
// font lookup (with fallback)
// font read 
// shaping (iterate till all characters in all runs are shaped) -> 
// cache shaping results
// reshape only the parts that change
// word breaking & line breaking
// 
// What about rich text?  