#![allow(dead_code)]

use crate::{Entity, HierarchyTree};
use crate::mouse::*;

use crate::{BuildHandler, Event, EventHandler, Propagation, WindowEvent};
use crate::{PropSet, State};

use crate::widgets::Element;

#[derive(Debug, Clone, PartialEq)]
pub enum RadioEvent {
    Check,
    Checked,
}

#[derive(Default)]
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
                RadioEvent::Checked => {
                    //println!("Received Radio Event: {}", event.target);
                    //if event.target == entity && event.origin != entity {
                        state.insert_event(
                            Event::new(RadioEvent::Check)
                                .target(entity)
                                .origin(event.target)
                                .propagate(Propagation::Fall),
                        );

                        event.consume();
                    //}
                }

                RadioEvent::Check => {
                    if event.target != entity {
                        event.consume();
                    }
                }
            }
        }
    }
}

#[derive(Default)]
pub struct Radio {
    marker: Entity,
    on_checked: Option<Event>,
    on_unchecked: Option<Event>,
}

impl Radio {
    pub fn new() -> Self {
        Self {
            marker: Entity::null(),
            on_checked: None,
            on_unchecked: None,
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

impl BuildHandler for Radio {
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

impl EventHandler for Radio {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(system_event) = event.message.downcast::<WindowEvent>() {
            match system_event {
                WindowEvent::MouseDown(button) => match button {
                    MouseButton::Left => {
                        if entity == event.target {
                            state.insert_event(
                                Event::new(RadioEvent::Checked)
                                .target(entity)
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
                RadioEvent::Check => {
                    if event.origin == entity {
                        if !entity.is_checked(state) {
                            entity.set_checked(state, true);

                            if let Some(mut on_checked) = self.on_checked.clone() {
                                if on_checked.target == Entity::null() {
                                    on_checked.target = entity;
                                }

                                on_checked.origin = entity;

                                state.insert_event(on_checked);
                            }
                        }

                    } else {
                        if entity.is_checked(state) {
                            entity.set_checked(state, false);

                            if let Some(mut on_unchecked) = self.on_unchecked.clone() {
                                if on_unchecked.target == Entity::null() {
                                    on_unchecked.target = entity;
                                }

                                on_unchecked.origin = entity;

                                state.insert_event(on_unchecked);
                            }
                        }
                    }
                }
                _=> {}
            }
        }
    }
}

#[derive(Default)]
pub struct RadioButton {
    radio: Radio,
}

impl RadioButton {
    pub fn new() -> Self {
        Self {
            radio: Radio::new(),
        }
    }
}

impl BuildHandler for RadioButton {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity.set_element(state, "radio_button")
    }
}

impl EventHandler for RadioButton {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        self.radio.on_event(state, entity, event);
    }
}