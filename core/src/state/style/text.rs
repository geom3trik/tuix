use crate::entity::Entity;

use crate::layout::{Align, Justify};

use crate::style::Color;

#[derive(Debug, Clone)]
pub struct Text {
    pub text: String,
    pub font: String,
}

impl Default for Text {
    fn default() -> Self {
        Text {
            text: "".to_string(),
            font: "sans".to_string(),
        }
    }
}
