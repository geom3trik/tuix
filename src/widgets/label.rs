#![allow(dead_code)]

use crate::widgets::*;

// Default style for labels
const label_style: &str = r#"
    label {
        width: 100px;
        height: 30px;
    }
"#;

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
        entity.set_text(state, &self.text);

        entity.set_element(state, "label");

        //state.insert_style(label_style);

        entity
    }
}

impl EventHandler for Label {}
