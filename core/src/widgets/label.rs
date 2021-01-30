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

impl BuildHandler for Label {
    type Ret = Handle;
    fn on_build(&mut self, state: &mut State, handle: Handle) -> Self::Ret {
        //entity.set_text(state, &self.text);

        //entity.set_element(state, "label");

        //state.insert_style(label_style);

        handle.set_text(&self.text).set_element("label")
    }
}
