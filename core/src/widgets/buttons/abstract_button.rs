


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
}

pub struct AbstractButton {
    pub checkable: bool,
    pub checked: bool,
    pub press_x: f32,
    pub press_y: f32,
    pub pressed: bool,
    pub text: String,

    pub on_pressed: Option<Box<dyn Fn(&mut Self, &mut State, Entity)>>,
    pub on_released: Option<Box<dyn Fn(&mut Self, &mut State, Entity)>>,
    pub on_toggled: Option<Box<dyn Fn(&mut Self, &mut State, Entity)>>,
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

            on_pressed: None,
            on_released: None,
            on_toggled: None,
        }
    }
}

impl BuildHandler for AbstractButton {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
    }
}

impl EventHandler for AbstractButton {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::MouseDown(button) => {
                    match button {
                        MouseButton::Left => {
                            if event.target == entity {
                                state.capture(entity);
                                
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
