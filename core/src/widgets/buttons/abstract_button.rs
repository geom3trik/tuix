


use crate::{Message, widgets::*};
use crate::style::*;


#[derive(Debug,Clone,PartialEq)]
pub enum ButtonEvent {
    //Clicked,
    //DoubleClicked,
    //PressAndHold,
    Pressed,
    Released,
    Toggled,
    SetLabel(String),
}

pub struct AbstractButton {
    pub checkable: bool,
    pub checked: bool,
    pub press_x: f32,
    pub press_y: f32,
    pub pressed: bool,
    pub text: String,

    pub on_pressed: Vec<Event>,
    pub on_released: Vec<Event>,
    pub on_checked: Vec<Event>,
    pub on_unchecked: Vec<Event>,
}

impl AbstractButton {
    pub fn new() -> Self {
        Self {
            checkable: false,
            checked: false,
            press_x: 0.0,
            press_y: 0.0,
            pressed: false,
            text: String::new(),

            on_pressed: Vec::new(),
            on_released: Vec::new(),
            on_checked: Vec::new(),
            on_unchecked: Vec::new(),
        }
    }

    pub fn on_pressed(mut self, event: Event) -> Self {
        self.on_pressed.push(event);

        self
    }

    pub fn on_released(mut self, event: event) -> Self {
        self.on_released.push(event);

        self
    }

    pub fn on_checked(mut self, event: event) -> Self {
        self.on_checked.push(event);

        self
    }

    pub fn on_unchecked(mut self, event: event) -> Self {
        self.on_unchecked.push(event);

        self
    }
}

impl Widget for AbstractButton {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::MouseDown(button) => {
                    match button {
                        MouseButton::Left => {
                            if event.target == entity {
                                state.capture(entity);
                                state.insert_event(
                                    Event::new(ButtonEvent::Pressed)
                                        .target(entity)
                                        .origin(entity),
                                );
                                if self.checkable {
                                    
                                }
                            }
                        }

                        _=> {}
                    }
                }

                WindowEvent::MouseUp(button) => {
                    match button {
                        MouseButton::Left => {
                            if event.target == entity && state.hovered == entity {
                                state.release(entity);
                                
                            }
                        }
                        
                        _=> {}
                    }
                }

                _=> {}
            }
        }
    }
}
