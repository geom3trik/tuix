use crate::entity::Entity;
use crate::State;
use crate::{BuildHandler, Event, EventHandler};

use femtovg::{renderer::OpenGl, Canvas};

pub struct Grid {}

impl Grid {
    pub fn new() -> Self {
        Grid {}
    }
}

impl BuildHandler for Grid {
    type Ret = Entity;
    fn on_build(&mut self, _state: &mut State, entity: Entity) -> Self::Ret {
        entity
    }
}

impl EventHandler for Grid {
    fn on_event(&mut self, _state: &mut State, _entity: Entity, _event: &mut Event) {}

    fn on_draw(&mut self, _state: &mut State, _entity: Entity, _canvas: &mut Canvas<OpenGl>) {

        // Draw background

        // Draw vertical lines

        // Draw horizontal lines
    }
}
