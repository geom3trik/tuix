use crate::entity::Entity;

use crate::layout::{Align, Justify};

use crate::style::Color;

#[derive(Debug, Clone)]
pub struct Text {
    pub text: String,
    pub font: String,
    //pub font_size: f32,
    //pub font_color: Color,
    //pub indent: f32,
}

impl Default for Text {
    fn default() -> Self {
        Text {
            text: "".to_string(),
            font: "Sans".to_string(),
            //font_size: 16.0,
            //font_color: Color::rgba(255, 255, 255, 255),
            //indent: 0.0,
        }
    }
}
