#![allow(dead_code)]

use crate::entity::Entity;
use crate::mouse::*;

use crate::{BuildHandler, Event, EventHandler, Propagation, WindowEvent};
use crate::{Handle, State, DefaultDrawHandler, EventData};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ButtonState {
    pub on_press: Option<Event>,
    pub on_release: Option<Event>,
}

impl ButtonState {
    pub fn on_press(mut self, event: Event) -> Self {
        self.on_press = Some(event);
        self
    }

    pub fn on_release(mut self, event: Event) -> Self {
        self.on_release = Some(event);
        self
    }
}

fn button_handler(
    button_state: &mut ButtonState,
    state: &mut State,
    handle: &Handle,
    event_data: &EventData,
    event: &mut WindowEvent,
) -> bool {
    match event {
        WindowEvent::MouseDown(button) => {
            if *button == MouseButton::Left && state.mouse.left.pressed == handle.entity {

                state.capture(handle.entity);

                if let Some(on_press) = button_state.on_press.clone() {
                    state.insert_event(on_press);
                }
            }
        }

        WindowEvent::MouseUp(button) => {
            if *button == MouseButton::Left && 
                state.mouse.left.pressed == handle.entity &&
                state.hovered == handle.entity      
            {
                
                state.release(handle.entity);
                
                if let Some(on_release) = button_state.on_release.clone() {
                    state.insert_event(on_release);
                }
            }
        }

        _ => {}
    }

    false
}

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
    type Ret = Handle;
    fn on_build(&mut self, state: &mut State, handle: Handle) -> Self::Ret {


        let mut button_state = ButtonState::default();

        if let Some(on_press) = self.on_press.clone() {
            button_state.on_press = Some(on_press);
        }

        if let Some(on_release) = self.on_release.clone() {
            button_state.on_release = Some(on_release);
        }

        handle.set_text(if let Some(txt) = &self.text {txt} else {""})
            .set_element("button")
            .add_component(button_state)
            .add_draw_hander(DefaultDrawHandler::default())
            .add_event_handler(button_handler)
    }
}

/*
impl EventHandler for Button {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) -> bool {
        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::MouseDown(button) => match button {
                    MouseButton::Left => {
                        if entity == event.target {
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
                        if entity == event.target {
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
*/