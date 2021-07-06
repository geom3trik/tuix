use crate::{Selection, Movement, TextHandler};

use unicode_segmentation::GraphemeCursor;
use std::ops::Range;

pub enum TextMessage {

    // Inserts a string into the text
    Insert(String),

    // Move the selection
    Move(Movement),

    MoveSelection(Movement),

    SelectAll,

    SelectLine,

    SelectParagraph,

    SelectWord,

    Delete,


}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextState {
    /// Text is in a normal non-selected state
    Normal,
    /// Text is in a selected state
    Selected,
}

impl Default for TextState {
    fn default() -> Self {
        TextState::Normal
    }
}

#[derive(Debug, Default)]
pub struct TextData {
    /// The underlying text string
    text: String,
    /// The text selection region
    selection: Selection,
}


impl TextData {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_owned(),
            selection: Selection::new(),
        }
    }
}

impl TextHandler for TextData {
    fn selection(&self) -> Selection {
        self.selection
    }

    fn set_selection(&mut self, selection: Selection) {
        self.selection = selection;
    }

    fn is_char_boundary(&self, index: usize) -> bool {
        self.text.is_char_boundary(index)
    }

    fn len(&self) -> usize {
        self.text.len()
    }

    fn replace_range(&mut self, range: Range<usize>, text: &str) {
        self.text.replace_range(range.clone(), text);
        if self.selection.anchor < range.start && self.selection.active < range.start {
            // no need to update selection
        } else if self.selection.anchor > range.end && self.selection.active > range.end {
            self.selection.anchor -= range.len();
            self.selection.active -= range.len();
            self.selection.anchor += text.len();
            self.selection.active += text.len();
        } else {
            self.selection.anchor = range.start + text.len();
            self.selection.active = range.start + text.len();
        }
    }

    fn slice(&self, range: Range<usize>) -> std::borrow::Cow<str> {
        self.text[range].to_string().into()
    }
}

pub trait Fsm {
    type Message;
    type Data;

    fn handle_message(self, message: Self::Message, data: &mut Self::Data) -> Self;
}

impl Fsm for TextState {
    type Message = TextMessage;
    type Data = TextData;

    fn handle_message(self, message: Self::Message, data: &mut Self::Data) -> Self {
        use TextState::*;
        use TextMessage::*;

        match (self, message) {
            (Normal, Insert(text)) => {
                let selection = data.selection();
                data.replace_range(selection.range(), &text);
                let carret_position = selection.min() + text.len();
                data.set_selection(Selection::carret(carret_position));
                
                Normal
            }

            (Normal, Move(movement)) => {
                match movement {
                    Movement::Upstream => {
                        let index = data.selection.min();
                        let mut cursor = GraphemeCursor::new(index, data.len(), true);
                        if let Some(new_index) = cursor.prev_boundary(&data.text, 0).unwrap() {
                            data.set_selection(Selection::carret(new_index));
                        }
                    }

                    Movement::Downstream => {
                        let index = data.selection.min();
                        let mut cursor = GraphemeCursor::new(index, data.len(), true);
                        if let Some(new_index) = cursor.next_boundary(&data.text, 0).unwrap() {
                            data.set_selection(Selection::carret(new_index));
                        }
                    }

                    Movement::UpstreamWord => {
                        // todo
                    }

                    Movement::UpLine => {

                    }

                    Movement::DownLine => {

                    }

                    _=> {}
                }

                Normal
            }

            _=> Normal,
        }
    }
}

// pub fn movement(movement: Movement, selection: Selection, text: &str) -> Selection {
//     let new_index = match movement {
//         Movement::Upstream => {
//             let index = selection.min();
//             let mut cursor = GraphemeCursor::new(index, text.len(), true);
//             if let Some(new_index) = cursor.prev_boundary(text, 0).unwrap() {
//                 new_index
//             } else {
//                 index
//             }
//         }

//         Movement::Downstream => {
//             let index = selection.min();
//             let mut cursor = GraphemeCursor::new(index, text.len(), true);
//             if let Some(new_index) = cursor.next_boundary(text, 0).unwrap() {
//                 new_index
//             } else {
//                 index
//             }
//         }
//     }
// }



