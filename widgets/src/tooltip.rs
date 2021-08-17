use crate::common::*;

pub struct Tooltip {
    text: String,
}

impl Tooltip {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_owned(),
        }
    }
}

impl Widget for Tooltip {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        let tooltip = entity.get_tooltip(state);
        entity
            .set_text(state, &tooltip)
            .set_hoverable(state, false)
            .set_focusable(state, false)
            .set_element(state, "tooltip")
    }
}
