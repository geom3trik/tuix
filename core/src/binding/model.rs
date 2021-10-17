use crate::{Entity, Event, Node, State, Store, Widget};



/// Trait which describes application data which can be bound to.
/// 
/// The [Model] trait is similar to the [Widget] trait but without any visual components.
/// The trait provides an `on_event()` method to respond to user events and mutate the app data.
/// Building a type which implements [Model], using the `build()` method, places the app data in a [Store],
/// which provides event handling for binding events.
pub trait Model {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        let _ = state;
        let _ = entity;
        let _ = event;
    }

    /// Adds the widget into state and returns the associated type Ret - an entity id or a tuple of entity ids
    fn build(self, state: &mut State, parent: Entity) -> Entity
    where Self: std::marker::Sized + Model + Node
    {
        Store::new(self).build(state, parent, |builder| builder)
    }
}