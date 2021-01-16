#![allow(dead_code)]

use crate::entity::Entity;
use crate::mouse::*;

use crate::{BuildHandler, Event, EventHandler, Propagation, WindowEvent};
use crate::{PropSet, State};

pub struct Button {
    pub id: Entity,

    on_press: Option<Event>,
    on_release: Option<Event>,
    text: Option<String>,
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

    // Add text to be displayed on the button
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
        
        state.style.insert_element(entity, "button");

        entity
    }
}

impl EventHandler for Button {

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) -> bool {
        
        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                

                WindowEvent::MouseDown(button) => match button {
                    MouseButton::Left => {
                        if entity == event.target {
                            state.focused = entity;
                            
                            if let Some(mut on_release) = self.on_release.clone() {
                                if on_release.target == Entity::null() {
                                    on_release.target = entity;
                                }
                                
                                on_release.origin = entity;
                                on_release.propagation = Propagation::Down;
                                state.insert_event(on_release);
                            }
                            
                        }
                    }

                    _ => {}
                },

                WindowEvent::MouseUp(button) => match button {
                    MouseButton::Left => {
                        if entity == event.target && entity == state.focused {
                            if let Some(mut on_press) = self.on_press.clone() {
                                if on_press.target == Entity::null() {
                                    on_press.target = entity;
                                }
                                
                                on_press.origin = entity;
                                on_press.propagation = Propagation::Down;
                                state.insert_event(on_press);
                            }
                        }
                    }

                    _ => {}
                },

                _ => {}
            }
        }

        false
    }
}
