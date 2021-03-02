
use crate::entity::Entity;
use crate::mouse::*;
use crate::{BuildHandler, Event, EventHandler, Propagation, WindowEvent};
use crate::{PropSet, State, IntoChildIterator};

use crate::widgets::ButtonEvent;

#[derive(Debug, Clone, PartialEq)]
pub enum CheckListEvent {
    Switch,
}

pub struct CheckList {

}

impl CheckList {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl BuildHandler for CheckList {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {


        entity.set_element(state, "cecklist");

        entity
    }
}

impl EventHandler for CheckList {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) -> bool {
        if let Some(button_event) = event.message.downcast::<ButtonEvent>() {
            match button_event {
                ButtonEvent::OnPress => {
                    if event.target.get_parent(state) == Some(entity) {
                        for child in entity.child_iter(&state.hierarchy.clone()) {
                            child.set_checked(state, false);
                        }

                        event.origin.set_checked(state, true); 
                        
                    }


                }

                _=> {}
            }
        }

        false
    }
}