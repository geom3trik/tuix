use crate::widgets::*;
use crate::{Code, Key};

#[derive(Debug, Clone, PartialEq)]
pub enum ListboxEvent {
    CheckNext(Entity),
    CheckPrev(Entity),
}

pub struct ListboxItem {

}

impl ListboxItem {
    
}

pub struct Listbox {}

impl Listbox {
    pub fn new() -> Self {
        Self {}
    }
}

impl BuildHandler for Listbox {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
    }
}

impl EventHandler for Listbox {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::KeyDown(_, key) => match key {
                    Some(Key::ArrowDown) => state.insert_event(
                        Event::new(ListboxEvent::CheckNext(Entity::null()))
                            .target(entity)
                            .propagate(Propagation::Fall),
                    ),

                    _ => {}
                },

                _ => {}
            }
        }
    }
}
