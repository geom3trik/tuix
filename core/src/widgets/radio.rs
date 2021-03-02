#![allow(dead_code)]

const ICON_CHECK: &str = "\u{2713}";

use crate::{Entity, HierarchyTree};

use crate::{BuildHandler, Event, EventHandler, Propagation};
use crate::{PropSet, State};

use crate::widgets::*;

#[derive(Default)]
pub struct RadioList {}

impl RadioList {
    pub fn new() -> Self {
        RadioList {}
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
                CheckboxEvent::Unchecked => {
                    if event.target != entity {
                        event.consume();
                    }
                }

                CheckboxEvent::Checked => {
                    //println!("Received Radio Event: {}", event.target);
                    if event.target.is_descendant_of(&state.hierarchy, entity) {
                        if event.target != entity && event.origin != entity {
                            state.insert_event(
                                Event::new(CheckboxEvent::Unchecked)
                                    .target(entity)
                                    .origin(event.target)
                                    .propagate(Propagation::Fall),
                            );

                            event.consume();
                        }

                        if event.target != entity && event.origin != entity {
                            state.insert_event(
                                Event::new(CheckboxEvent::Checked)
                                    .target(event.target)
                                    .origin(entity)
                                    .propagate(Propagation::Direct),
                            );

                            event.consume();
                        }
                    }
                }

                CheckboxEvent::Check => {
                    if event.target != entity {
                        event.consume();
                    }

                    if event.target.is_descendant_of(&state.hierarchy, entity) {
                        if event.target != entity && event.origin != entity {
                            state.insert_event(
                                Event::new(CheckboxEvent::Uncheck)
                                    .target(entity)
                                    .origin(event.target)
                                    .propagate(Propagation::Fall),
                            );

                            event.consume();
                        }

                        if event.target != entity && event.origin != entity {
                            state.insert_event(
                                Event::new(CheckboxEvent::Check)
                                    .target(event.target)
                                    .origin(entity)
                                    .propagate(Propagation::Direct),
                            );

                            event.consume();
                        }
                    }
                }

                CheckboxEvent::Uncheck => {
                    if event.target != entity {
                        event.consume();
                    }
                }
                _ => {}
            }
        }
    }
}

#[derive(Default)]
pub struct Radio {
    marker: Entity,
    checkbox: Checkbox,
}

impl Radio {
    pub fn new() -> Self {
        Self {
            marker: Entity::null(),
            checkbox: Checkbox::new(false).with_icon_checked("").with_icon_unchecked(""),
        }
    }

    pub fn on_checked(mut self, event: Event) -> Self {
        self.checkbox = self.checkbox.on_checked(event);
        self
    }

    pub fn on_unchecked(mut self, event: Event) -> Self {
        self.checkbox = self.checkbox.on_unchecked(event);
        self
    }
}

impl BuildHandler for Radio {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        self.marker = Element::new().build(state, entity, |builder| {
            builder.set_hoverability(false).class("marker").set_hoverability(false)
        });

        entity.set_element(state, "radio")
    }
}

impl EventHandler for Radio {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        self.checkbox.on_event(state, entity, event);
    }
}

#[derive(Default)]
pub struct RadioButton {
    checkbox: Checkbox,
}

impl RadioButton {
    pub fn new() -> Self {
        Self {
            checkbox: Checkbox::new(false).with_icon_checked("").with_icon_unchecked(""),
        }
    }

    pub fn with_label(label: &str) -> Self {
        Self {
            checkbox: Checkbox::new(false).with_icon_checked(label).with_icon_unchecked(label),
        }
    }

    pub fn on_checked(mut self, event: Event) -> Self {
        self.checkbox = self.checkbox.on_checked(event);

        self
    }

    pub fn on_unchecked(mut self, event: Event) -> Self {
        self.checkbox = self.checkbox.on_unchecked(event);

        self
    }
}

impl BuildHandler for RadioButton {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        self.checkbox.on_build(state, entity);
        entity.set_element(state, "radio_button").set_font(state, "sans")
    }
}

impl EventHandler for RadioButton {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        self.checkbox.on_event(state, entity, event);
    }
}
