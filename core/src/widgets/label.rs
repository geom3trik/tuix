#![allow(dead_code)]

use crate::widgets::*;

pub struct Label {
    text: String,
}

impl Label {
    pub fn new(text: &str) -> Self {
        Label {
            text: text.to_string(),
        }
    }
}

impl Widget for Label {
    type Ret = Entity;
    fn on_build(&mut self, context: Context) -> Self::Ret {
        context
            .set_text(&self.text)
            .set_element("label")
            .set_focusability(false)
            .entity()
    }
}
