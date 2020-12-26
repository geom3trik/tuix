


use crate::entity::Entity;

use crate::{State, BuildHandler, EventHandler};

pub struct Element {

}

impl Element {
    pub fn new() -> Self {
        Element {

        }
    }
}

impl BuildHandler for Element {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
    }
}

impl EventHandler for Element {}