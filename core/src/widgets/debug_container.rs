

use crate::*;

pub struct DebugContainer {

}

impl DebugContainer {

}

impl BuildHandler for DebugContainer {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
    }
}

impl EventHandler for DebugContainer {

}