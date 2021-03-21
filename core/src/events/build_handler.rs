
use crate::{State, Entity, EventHandler};
use crate::builder::Builder;

// Inherited by all widgets
trait BuildHandler: EventHandler {
    type Ret;

    /// A method which is called when a widget is built using `.build()`
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret;

    /// Adds the widget into state and returns the associated type Ret - an entity id or a tuple of entity ids
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
