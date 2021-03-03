#![allow(dead_code)]

use crate::{Checkbox, Element, entity::Entity};
use crate::mouse::*;
use crate::{BuildHandler, Event, EventHandler, HierarchyTree, Propagation, WindowEvent};
use crate::{PropSet, State};

use crate::state::style::*;
use crate::widgets::Button;

use crate::state::hierarchy::IntoChildIterator;

// Notes:
// When user clicks menu, the container should appear
// When container is visible, clicking on a menu item activates the item
//  Need the option to close the menu on item press

#[derive(Debug, Clone, PartialEq)]
pub enum MenuEvent {
    Open(Entity),
    Close(Entity),
    CloseAll(Entity),
    OpenHover(bool),
}

#[derive(Debug, Copy, Clone)]
pub enum MenuPosition {
    Auto, // TODO
    Down,
    Right,
}

pub struct Menu {
    container: Entity,
    open: bool,
}

impl Menu {
    pub fn new(text: &str, menu_position: MenuPosition) -> Self {
        Menu {
            container: Entity::default(),
            open: false,
        }
    }
}

impl BuildHandler for Menu {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity.set_flex_direction(state, FlexDirection::Column);

        self.container = Element::new().build(state, entity, |builder| {
            builder
                .set_position(Position::Absolute)
                .set_z_order(1)
                .class("container")
        });

        state.style.insert_element(entity, "menu");

        self.container
    }
}

impl EventHandler for Menu {

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::MouseDown(button) => {
                    if *button == MouseButton::Left {
                        if state.hovered == entity {
                            if !self.open {
                                entity.set_checked(state, true);
                                state.capture(entity);
                                self.open = true;
                            } else {
                                entity.set_checked(state, false);
                                state.release(entity);
                                self.open = false;
                            }
                            
                        } else {


                            // if state.hovered.is_descendant_of(&state.hierarchy, entity) {
                            //     state.insert_event(Event::new(WindowEvent::MouseUp()))
                            // }

                            entity.set_checked(state, false);
                            state.release(entity);
                            self.open = false;
                        }
                    }
                }

                _=> {}


            }
        }

    }
}
