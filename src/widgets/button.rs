#![allow(dead_code)]

use crate::entity::Entity;
use crate::mouse::*;

use crate::{BuildHandler, Event, EventHandler, Propagation, WindowEvent};
use crate::{PropSet, State};

use crate::window::KeyboardInput;

pub struct Button {
    pub id: Entity,

    on_press: Option<Event>,
    shortcut: Option<KeyboardInput>,
    text: Option<String>,
}

impl Default for Button {
    fn default() -> Self {
        Button {
            id: Entity::default(),
            on_press: None,
            shortcut: None,
            text: None,
        }
    }
}

impl Button {
    pub fn new() -> Self {
        Button {
            id: Entity::default(),
            on_press: None,
            shortcut: None,
            text: None,
        }
    }

    pub fn with_label(text: &str) -> Self {
        Button {
            id: Entity::default(),
            on_press: None,
            shortcut: None,
            text: Some(text.to_string()),
        }
    }

    pub fn on_press(mut self, message: Event) -> Self {
        self.on_press = Some(message);
        self
    }

    pub fn temp_on_press(&mut self, message: Event) -> &mut Self {
        self.on_press = Some(message);

        self
    }

    pub fn with_keyboard_shortcut(mut self, key_input: KeyboardInput) -> Self {
        self.shortcut = Some(key_input);

        self
    }
}

impl BuildHandler for Button {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        if let Some(text) = &self.text {
            entity.set_text(state, text);
            state.style.insert_element(entity, "button");
        //builder.set_text(text).element("button")
        } else {
            state.style.insert_element(entity, "button");
            //builder.element("button")
        }

        entity
    }
}

impl EventHandler for Button {
    // fn on_build(&mut self, state: &mut State, entity: Entity) -> Entity {
    //     if let Some(text) = &self.text {
    //         entity.set_text(state, text);
    //         state.style.insert_element(entity, "button");
    //     //builder.set_text(text).element("button")
    //     } else {
    //         state.style.insert_element(entity, "button");
    //         //builder.element("button")
    //     }

    //     entity
    // }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) -> bool {
        if let Some(system_event) = event.message.downcast::<WindowEvent>() {
            match system_event {
                WindowEvent::WindowClose => {
                    if state.hovered == entity {
                        println!("Window Close Event: {:?}", event.target);
                    }
                }

                WindowEvent::MouseDown(button) => match button {
                    MouseButton::Left => {
                        if entity == event.target {
                            state.focused = entity;
                        }
                    }

                    _ => {}
                },

                WindowEvent::MouseUp(button) => match button {
                    MouseButton::Left => {
                        if entity == event.target && entity == state.focused {
                            println!("Trigger Button Event");
                            if let Some(mut on_press) = self.on_press.clone() {
                                on_press.target = entity;
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
