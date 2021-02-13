use crate::{entity, Entity, EventHandler, EventManager, Selector, State};

use crate::{Align, Display, FlexDirection, Hierarchy, Justify};

use crate::state::style::flexbox::{AlignContent, AlignItems, AlignSelf};
use super::builder::Builder;
use crate::style::*;


// Inherited by all widgets
pub trait BuildHandler: EventHandler {
    type Ret;

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret;

    // Adds the widget into state and returns Ret - an entity id or a tuple of entity ids
    fn build<F>(mut self, state: &mut State, parent: Entity, mut builder: F) -> Self::Ret
    where
        F: FnMut(Builder) -> Builder,
        Self: std::marker::Sized + 'static,
    {
        // Create a new entity
        let entity = state.add(parent);

        // Call the on_build function of the widget
        let ret = self.on_build(state, entity);

        // Call the builder closure
        builder(Builder::new(state, entity)).build(self);

        // Return the entity or entities returned by the on_build method
        ret
    }
}


