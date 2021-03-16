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
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
            .set_text(state, &self.text)
            .set_element(state, "label")
            .set_focusability(state, false)
    }
}

impl EventHandler for Label {}
