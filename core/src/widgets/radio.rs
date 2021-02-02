#![allow(dead_code)]

use crate::{Entity, HierarchyTree};
use crate::mouse::*;

use crate::{BuildHandler, Event, EventHandler, Propagation, WindowEvent};
use crate::{PropSet, State};

use crate::widgets::Element;

#[derive(Debug, Clone, PartialEq)]
pub enum RadioEvent {
    Activate(Entity),
}

pub struct RadioList {
    
}

impl RadioList {
    pub fn new() -> Self {
        RadioList {
        }
    }
}

impl BuildHandler for RadioList {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        entity.set_element(state, "radio_list")
    }
}

impl EventHandler for RadioList {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(radio_event) = event.message.downcast::<RadioEvent>() {
            match radio_event {
                RadioEvent::Activate(radio) => {
                    if event.target == entity && event.origin != entity {
                        state.insert_event(
                            Event::new(RadioEvent::Activate(*radio))
                                .target(entity)
                                .origin(entity)
                                .propagate(Propagation::Fall),
                        );

                        event.consume();
                    }
                }
            }
        }
    }
}

pub struct RadioButton {
    marker: Entity,
    on_checked: Option<Event>,
    on_unchecked: Option<Event>,
    checked: bool,
}

impl RadioButton {
    pub fn new() -> Self {
        Self {
            marker: Entity::null(),
            on_checked: None,
            on_unchecked: None,
            checked: false,
        }
    }

    pub fn on_checked(mut self, event: Event) -> Self {
        self.on_checked = Some(event);
        self
    }

    pub fn on_unchecked(mut self, event: Event) -> Self {
        self.on_unchecked = Some(event);
        self
    }
}

impl BuildHandler for RadioButton {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        self.marker = Element::new().build(state, entity, |builder| {
            builder
                .set_hoverability(false)
                .class("marker")
        });

        entity.set_element(state, "radio")
    }
}

impl EventHandler for RadioButton {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(system_event) = event.message.downcast::<WindowEvent>() {
            match system_event {
                WindowEvent::MouseDown(button) => match button {
                    MouseButton::Left => {
                        if entity == event.target {
                            state.insert_event(
                                Event::new(RadioEvent::Activate(entity))
                                .target(entity.parent(&state.hierarchy).unwrap())
                                .propagate(Propagation::Up),
                            );
                        }
                    }

                    _ => {}
                },

                _ => {}
            }
        }

        if let Some(radio_event) = event.message.downcast::<RadioEvent>() {
            match radio_event {
                RadioEvent::Activate(radio) => {
                    if Some(event.target) == entity.parent(&state.hierarchy) {
                        if *radio == entity{
                            if !self.checked {
                                entity.set_checked(state, true);

                                if let Some(mut on_checked) = self.on_checked.clone() {
                                    if on_checked.target == Entity::null() {
                                        on_checked.target = entity;
                                    }

                                    on_checked.origin = entity;

                                    state.insert_event(on_checked);
                                }
                                
                                self.checked = true;
                            }

                        } else {
                            if self.checked {
                                entity.set_checked(state, false);

                                if let Some(mut on_unchecked) = self.on_unchecked.clone() {
                                    if on_unchecked.target == Entity::null() {
                                        on_unchecked.target = entity;
                                    }

                                    on_unchecked.origin = entity;

                                    state.insert_event(on_unchecked);
                                }

                                self.checked = false;
                            }
                        }
                    }


                }
            }
        }
    }
}
