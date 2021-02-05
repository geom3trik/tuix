#![allow(dead_code)]

use crate::{Entity, HierarchyTree};
use crate::mouse::*;

use crate::{BuildHandler, Event, EventHandler, Propagation, WindowEvent};
use crate::{PropSet, State};

use crate::widgets::Element;

use crate::widgets::checkbox::*;

// #[derive(Debug, Clone, PartialEq)]
// pub enum RadioEvent {
//     Check,
//     Uncheck,
//     Checked,
//     Unchecked,
// }

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
        if let Some(radio_event) = event.message.downcast::<CheckboxEvent>() {
            match radio_event {
                CheckboxEvent::Checked | CheckboxEvent::Unchecked => {
                    //println!("Received Radio Event: {}", event.target);
                    //if event.target == entity && event.origin != entity {
                        state.insert_event(
                            Event::new(CheckboxEvent::Uncheck)
                                .target(entity)
                                .origin(event.target)
                                .propagate(Propagation::Fall),
                        );

                        state.insert_event(
                            Event::new(CheckboxEvent::Check)
                                .target(event.target)
                                .origin(entity)
                                .propagate(Propagation::Direct),
                        );

                        // state.insert_event(
                        //     Event::new(RadioEvent::Check)
                        //         .target(entity)
                        //         .origin(event.target)
                        //         .propagate(Propagation::Fall),
                        // );

                        event.consume();
                    //}
                }

                CheckboxEvent::Check => {
                    if event.target != entity {
                        event.consume();
                    }
                }

                CheckboxEvent::Uncheck => {
                    if event.target != entity {
                        event.consume();
                    }
                }

                _=> {}
            }
        }
    }
}

#[derive(Default)]
pub struct Radio {
    marker: Entity,
    checkbox: Checkbox,
    //on_checked: Option<Event>,
    //on_unchecked: Option<Event>,
}

impl Radio {
    pub fn new() -> Self {
        Self {
            marker: Entity::null(),
            checkbox: Checkbox::default(),
            //on_checked: None,
            //on_unchecked: None,
        }
    }

    pub fn on_checked(mut self, event: Event) -> Self {
        self.checkbox.on_checked = Some(event);
        self
    }

    pub fn on_unchecked(mut self, event: Event) -> Self {
        self.checkbox.on_unchecked = Some(event);
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

        self.checkbox.on_event(state, entity, event);

        /*
        if let Some(system_event) = event.message.downcast::<WindowEvent>() {
            match system_event {
                WindowEvent::MouseDown(button) => match button {
                    MouseButton::Left => {
                        if entity == event.target {
                            if !entity.is_checked(state) {
                                //entity.set_checked(state, true);

                                // if let Some(mut on_checked) = self.on_checked.clone() {
                                //     if on_checked.target == Entity::null() {
                                //         on_checked.target = entity;
                                //     }

                                //     on_checked.origin = entity;

                                //     state.insert_event(on_checked);
                                // }
                                
                                state.insert_event(Event::new(CheckboxEvent::Checked).target(entity));
                            }                            
                        }



                    }

                    _ => {}
                },

                _ => {}
            }
        }

        if let Some(radio_event) = event.message.downcast::<CheckboxEvent>() {
            match radio_event {

                CheckboxEvent::Checked => {
                    if event.target == entity {
                        entity.set_checked(state, true);

                        if let Some(mut on_checked) = self.on_checked.clone() {
                            if on_checked.target == Entity::null() {
                                on_checked.target = entity;
                            }

                            on_checked.origin = entity;

                            state.insert_event(on_checked);
                        }
                    }
                }

                CheckboxEvent::Unchecked => {

                }

                CheckboxEvent::Uncheck => {
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

                CheckboxEvent::Check => {
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
                }

                /*
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
                */
                _=> {}
            }
        }
        */
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