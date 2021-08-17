#![allow(dead_code)]

use std::marker::PhantomData;


use crate::*;

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
    type Data = String;

    fn widget_name(&self) -> String {
        "label".to_string()
    }

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
            .set_text(state, &self.text.to_string())
            .set_element(state, "label")
            //.set_focusable(state, false)
    }

    fn on_update(&mut self, state: &mut State, entity: Entity, data: &Self::Data) {
        self.text = data.to_owned();
        entity.set_text(state, &self.text);
        //entity.set_name(state, &self.text);
    }
}

