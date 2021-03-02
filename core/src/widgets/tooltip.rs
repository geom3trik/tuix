use crate::widgets::*;

pub struct Tooltip {
    text: String,
}

impl Tooltip {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
        }
    }
}

impl BuildHandler for Tooltip {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
            .set_text(state, &self.text)
            .set_element(state, "tooltip")
    }
}

impl EventHandler for Tooltip {}
