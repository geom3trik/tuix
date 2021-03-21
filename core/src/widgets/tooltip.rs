use crate::widgets::*;

pub struct Tooltip {

}

impl Tooltip {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl Widget for Tooltip {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        let tooltip = entity.get_tooltip(state);
        entity
            .set_text(state, &tooltip)
            .set_element(state, "tooltip")
    }
}
