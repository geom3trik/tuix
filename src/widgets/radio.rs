#![allow(dead_code)]

use crate::entity::Entity;
use crate::mouse::*;

use crate::{BuildHandler, Event, EventHandler, Propagation, Visibility, WindowEvent};
use crate::{JustifyContent, Length, PropSet, State};

use crate::widgets::Button;

use crate::state::style::AlignItems;

#[derive(Debug, Clone, PartialEq)]
pub enum RadioEvent {
    Activate(Entity, String),
}

pub struct RadioList {
    group_name: String,
}

impl RadioList {
    pub fn new(group_name: &str) -> Self {
        RadioList {
            group_name: group_name.to_string(),
        }
    }
}

impl BuildHandler for RadioList {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        state.style.insert_element(entity, "radio_list");

        entity
    }
}

impl EventHandler for RadioList {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) -> bool {
        let origin = event.origin;

        if let Some(radio_event) = event.is_type::<RadioEvent>() {
            match radio_event {
                RadioEvent::Activate(radio, group) => {
                    if origin != entity {
                        state.insert_event(
                            Event::new(RadioEvent::Activate(*radio, group.clone()))
                                .target(entity)
                                .origin(entity)
                                .propagate(Propagation::Fall),
                        );

                        return true;
                    }
                }
            }
        }

        false
    }
}

pub struct RadioBox {
    marker: Entity,
    active: bool,
    group_name: String,
}

impl RadioBox {
    pub fn new(group_name: &str) -> Self {
        RadioBox {
            marker: Entity::null(),
            active: false,
            group_name: group_name.to_string(),
        }
    }
}

impl BuildHandler for RadioBox {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
            //.set_border_radius(state, Length::Percentage(0.5))
            .set_width(state, Length::Pixels(16.0))
            .set_height(state, Length::Pixels(16.0))
            .set_border_radius(state, Length::Percentage(0.5))
            //.set_border_width(state, 2.0)
            .set_align_items(state, AlignItems::Center)
            .set_justify_content(state, JustifyContent::Center);

        self.marker = Button::new().build(state, entity, |builder| {
            builder
                .set_width(Length::Pixels(8.0))
                .set_height(Length::Pixels(8.0))
                .set_border_radius(Length::Percentage(0.5))
                .class("marker")
        });

        self.marker.set_visibility(state, Visibility::Invisible);

        state.style.insert_element(entity, "radio");

        entity
    }
}

impl EventHandler for RadioBox {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) -> bool {
        if let Some(system_event) = event.message.downcast::<WindowEvent>() {
            match system_event {
                WindowEvent::MouseDown(button) => match button {
                    MouseButton::Left => {
                        if entity == event.target {
                            //state.focused = entity;
                            if !self.active {
                                self.active = true;
                                self.marker.set_visibility(state, Visibility::Visible);
                                state.insert_event(
                                    Event::new(RadioEvent::Activate(
                                        entity,
                                        self.group_name.clone(),
                                    ))
                                    .target(entity)
                                    .propagate(Propagation::Up),
                                );
                            }
                        }
                    }

                    _ => {}
                },

                _ => {}
            }
        }

        if let Some(radio_event) = event.is_type::<RadioEvent>() {
            match radio_event {
                RadioEvent::Activate(radio, group) => {
                    if *radio != entity && group == &self.group_name {
                        self.active = false;
                        self.marker.set_visibility(state, Visibility::Invisible);
                    }
                }
            }
        }

        false
    }
}
