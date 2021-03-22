use crate::widgets::*;

use femtovg::{renderer::OpenGl, Canvas};

pub struct Grid {}

impl Grid {
    pub fn new() -> Self {
        Grid {}
    }
}

impl Widget for Grid {
    type Ret = Entity;
    fn on_build(&mut self, _state: &mut State, entity: Entity) -> Self::Ret {
        entity
    }

    fn on_event(&mut self, _state: &mut State, _entity: Entity, _event: &mut Event) {}

    fn on_draw(&mut self, _state: &mut State, _entity: Entity, _canvas: &mut Canvas<OpenGl>) {

        // Draw background

        // Draw vertical lines

        // Draw horizontal lines
    }
}
