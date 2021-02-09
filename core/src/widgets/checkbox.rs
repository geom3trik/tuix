#![allow(dead_code)]

use crate::entity::Entity;
use crate::mouse::*;
use crate::{BuildHandler, Event, EventHandler, WindowEvent};
use crate::{PropSet, State};

use crate::style::layout::{Align, Justify};


const ICON_CHECK: &str = "\u{2713}";

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CheckboxEvent {
    Check,
    Uncheck,
    Switch,
    Checked,
    Unchecked,
}

#[derive(Clone, Default)]
pub struct Checkbox {
    checked: bool,

    icon_unchecked: Option<String>,
    icon_checked: Option<String>,

    on_checked: Option<Event>,
    on_unchecked: Option<Event>,
}

impl Checkbox {
    pub fn new(checked: bool) -> Self {
        Checkbox {
            checked,
            icon_unchecked: Some(String::new()),
            icon_checked: Some(ICON_CHECK.to_string()),

            on_checked: None,
            on_unchecked: None,
        }
    }

    fn checked(mut self, flag: bool) -> Self {
        self.checked = flag;

        self
    }

    pub fn with_icon_checked(mut self, icon_checked: &str) -> Self {
        self.icon_checked = Some(icon_checked.to_string());

        self
    }

    pub fn with_icon_unchecked(mut self, icon_unchecked: &str) -> Self {
        self.icon_unchecked = Some(icon_unchecked.to_string());

        self
    }

    fn switch(&mut self, state: &mut State, entity: Entity) {
        if self.checked {
            self.checked = false;
            if let Some(icon_unchecked) = &self.icon_unchecked {
                entity.set_text(state, &icon_unchecked);
            }

            entity.set_checked(state, false);
        } else {
            self.checked = true;
            if let Some(icon_checked) = &self.icon_checked {
                entity.set_text(state, &icon_checked);
            }

            entity.set_checked(state, true);
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

impl BuildHandler for Checkbox {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
            .set_font(state, "Icons")
            .set_text_justify(state, Justify::Center)
            .set_text_align(state, Align::Center);

        if self.checked {
            entity.set_checked(state, true);

            if let Some(icon_checked) = &self.icon_checked {
                entity.set_text(state, &icon_checked);
            }
        } else {
            entity.set_checked(state, false);

            if let Some(icon_unchecked) = &self.icon_unchecked {
                entity.set_text(state, &icon_unchecked);
            }
        }

        state.style.insert_element(entity, "checkbox");

        entity
    }
}

impl EventHandler for Checkbox {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(checkbox_event) = event.message.downcast::<CheckboxEvent>() {
            match checkbox_event {
                CheckboxEvent::Switch => {
                    if event.target == entity {
                        self.switch(state, entity);
                    }
                }

                CheckboxEvent::Check => {
                    self.checked = true;
                    entity.set_checked(state, true);
                    if let Some(icon_checked) = &self.icon_checked {
                        entity.set_text(state, &icon_checked);
                    }
                }

                CheckboxEvent::Uncheck => {
                    self.checked = false;
                    entity.set_checked(state, false);
                    if let Some(icon_unchecked) = &self.icon_unchecked {
                        entity.set_text(state, &icon_unchecked);
                    }
                }

                CheckboxEvent::Checked => {
                    println!("Checked");
                    //if event.target == entity {
                        self.checked = true;
                        if let Some(icon_checked) = &self.icon_checked {
                            entity.set_text(state, &icon_checked);
                        }

                        entity.set_checked(state, true);

                        if let Some(mut on_checked) = self.on_checked.clone() {
                            if on_checked.target == Entity::null() {
                                on_checked.target = entity;
                            }

                            on_checked.origin = entity;
                            state.insert_event(on_checked);
                        }

                        //state.insert_event(Event::new(CheckboxEvent::Checked).target(entity).origin(entity));
                    //}
                }

                CheckboxEvent::Unchecked => {
                    println!("Unchecked");
                    //if event.target == entity {
                        self.checked = false;
                        if let Some(icon_unchecked) = &self.icon_unchecked {
                            entity.set_text(state, &icon_unchecked);
                        }
                        entity.set_checked(state, false);

                        if let Some(mut on_unchecked) = self.on_unchecked.clone() {
                            if on_unchecked.target == Entity::null() {
                                on_unchecked.target = entity;
                            }

                            on_unchecked.origin = entity;

                            state.insert_event(on_unchecked);
                        }
                        //state.insert_event(Event::new(CheckboxEvent::Unchecked).target(entity).origin(entity));

                    //}
                }

                _ => {}
            }
        }

        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {

                WindowEvent::MouseDown(button) => {
                    if *button == MouseButton::Left && event.target == entity {
                        state.capture(entity);
                    }
                },

                WindowEvent::MouseUp(button) => {
     
                    if *button == MouseButton::Left && 
                        event.target == entity && 
                        state.mouse.left.pressed == entity
                    {
                        if state.hovered == entity {
                            if self.checked {
                                // if let Some(mut on_unchecked) = self.on_unchecked.clone() {
                                //     if on_unchecked.target == Entity::null() {
                                //         on_unchecked.target = entity;
                                //     }

                                //     on_unchecked.origin = entity;

                                //     state.insert_event(on_unchecked);
                                   
                                // }
                                state.insert_event(Event::new(CheckboxEvent::Unchecked).target(entity).origin(entity));

                            } else {
                                // if let Some(mut on_checked) = self.on_checked.clone() {
                                //     if on_checked.target == Entity::null() {
                                //         on_checked.target = entity;
                                //     }

                                //     on_checked.origin = entity;

                                //     state.insert_event(on_checked);
                                // }
                                state.insert_event(Event::new(CheckboxEvent::Checked).target(entity).origin(entity));

                            }

                            //self.switch(state, entity);                            
                        }


                        state.release(entity);
                    }
                },

                _ => {}
            }
        }
    }
}
