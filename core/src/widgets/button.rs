#![allow(dead_code)]

use crate::entity::Entity;
use crate::mouse::*;

use crate::{BuildHandler, Event, EventHandler, Propagation, WindowEvent};
use crate::{Code, PropSet, State};

#[derive(Debug, Clone, PartialEq)]
pub enum ButtonEvent {
    Pressed,
    Released,
    Press,
    Release,
}

#[derive(Default)]
pub struct Button {
    pub id: Entity,

    on_press: Option<Event>,
    on_release: Option<Event>,
    pub text: Option<String>,
}

impl Button {
    pub fn new() -> Self {
        Button {
            id: Entity::default(),
            on_press: None,
            on_release: None,
            text: None,
        }
    }

    pub fn with_label(text: &str) -> Self {
        Button {
            id: Entity::default(),
            on_press: None,
            on_release: None,
            text: Some(text.to_string()),
        }
    }

    pub fn on_press(mut self, event: Event) -> Self {
        self.on_press = Some(event);
        self
    }

    pub fn on_release(mut self, event: Event) -> Self {
        self.on_release = Some(event);
        self
    }
}

impl BuildHandler for Button {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        if let Some(text) = &self.text {
            entity.set_text(state, text);
        }

        entity.set_element(state, "button")
    }
}

impl EventHandler for Button {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(button_event) = event.message.downcast::<ButtonEvent>() {
            match button_event {
                ButtonEvent::Pressed => {
                    if let Some(mut on_press) = self.on_press.clone() {
                        if on_press.target == Entity::null() {
                            on_press.target = entity;
                        }

                        on_press.origin = entity;
                        on_press.propagation = Propagation::Down;

                        state.insert_event(on_press);
                    }

                    entity.set_active(state, true);
                }

                ButtonEvent::Released => {
                    if let Some(mut on_release) = self.on_release.clone() {
                        if on_release.target == Entity::default() {
                            on_release.target = entity;
                        }

                        on_release.origin = entity;
                        on_release.propagation = Propagation::Down;
                        state.insert_event(on_release);
                    }

                    entity.set_active(state, false);
                }

                _ => {}
            }
        }

        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::MouseDown(button) => match button {
                    MouseButton::Left => {
                        //println!("entity: {} {:?}", entity, entity.is_disabled(state));
                        if entity == event.target && !entity.is_disabled(state) {
                            state.capture(entity);
                            state.insert_event(
                                Event::new(ButtonEvent::Pressed)
                                    .target(entity)
                                    .origin(entity),
                            );
                        }
                    }

                    _ => {}
                },

                WindowEvent::MouseUp(button) => match button {
                    MouseButton::Left => {
                        if entity == event.target {
                            state.release(entity);
                            entity.set_active(state, false);
                            if !entity.is_disabled(state) {
                                if state.hovered == entity {
                                    state.insert_event(
                                        Event::new(ButtonEvent::Released)
                                            .target(entity)
                                            .origin(entity),
                                    );
                                }
                            }
                        }
                    }

                    _ => {}
                },

                WindowEvent::KeyDown(code, _) => match code {
                    Code::Space => {
                        if state.focused == entity && !entity.is_disabled(state) {
                            state.insert_event(
                                Event::new(ButtonEvent::Pressed)
                                    .target(entity)
                                    .origin(entity),
                            );
                        }
                    }

                    _ => {}
                },

                WindowEvent::KeyUp(code, _) => match code {
                    Code::Space => {
                        state.insert_event(
                            Event::new(ButtonEvent::Released)
                                .target(entity)
                                .origin(entity),
                        );
                    }

                    _ => {}
                },

                _ => {}
            }
        }
    }
}
