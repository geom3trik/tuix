#![allow(dead_code)]

const ICON_LEFT_OPEN_BIG: &str = "\u{e761}";
const ICON_RIGHT_OPEN_BIG: &str = "\u{e762}";

const ICON_DOWN_OPEN_MINI: &str = "\u{e760}";
const ICON_UP_OPEN_MINI: &str = "\u{e763}";

use crate::entity::Entity;
use crate::events::*;
use crate::state::style::*;
use crate::{PropSet, State, WindowEvent};

use crate::state::mouse::MouseButton;

use crate::layout::{Align, Justify};

use crate::widgets::{Element, Button, Textbox, TextboxEvent};

#[derive(Debug, Clone, PartialEq)]
pub enum SpinnerEvent {
    Increase,
    Decrease,
    SetValue(f32),
    ValueChanged(f32),
}

//impl Message for NumEditEvent {}

#[derive(Clone)]
pub struct Spinner {
    pub value: f32,
    pub textbox: Entity,
    pub increment: Entity,
    pub decrement: Entity,

    pub inc_value: f32,
}

impl Spinner {
    pub fn new(val: f32, inc_value: f32) -> Self {
        // entity.set_text(state, "Test".to_string())
        //     .set_background(state, nanovg::Color::from_rgb(100, 50, 50));

        Spinner {
            value: val,
            inc_value: inc_value,
            textbox: Entity::null(),
            increment: Entity::null(),
            decrement: Entity::null(),
        }
    }

    // pub fn set_enabled(&self, state: &mut WentitygetState, val: bool) {
    //     if val {
    //         self.entity
    //             .set_background(state, nanovg::Color::from_rgb(100, 50, 50));
    //     } else {
    //         self.entity
    //             .set_background(state, nanovg::Color::from_rgb(50, 50, 100));
    //     }
    // }
}

impl BuildHandler for Spinner {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
            .set_display(state, Display::Flexbox)
            .set_flex_direction(state, FlexDirection::Row);

        self.textbox = Textbox::new(&self.value.to_string())
            .build(state, entity, |builder| builder.set_flex_grow(1.0));


        let arrow_container = Element::new().build(state, entity, |builder| {
            builder.set_width(Length::Pixels(19.0)).set_flex_grow(0.0).class("arrow_container")
        });

        self.increment = Element::new()
            //.on_press(Event::new(SpinnerEvent::Increase))
            .build(state, arrow_container, |builder| {
                builder
                    .set_font("Icons".to_string())
                    .set_text_justify(Justify::Center)
                    .set_text_align(Align::Center)
                    .set_text(ICON_UP_OPEN_MINI)
                    .set_flex_grow(1.0)
                    .class("increment")
            });

        self.decrement = Element::new()
            //.on_press(Event::new(SpinnerEvent::Decrease))
            .build(state, arrow_container, |builder| {
                builder
                    .set_font("Icons".to_string())
                    .set_text_justify(Justify::Center)
                    .set_text_align(Align::Center)
                    .set_text(ICON_DOWN_OPEN_MINI)
                    .set_flex_grow(1.0)
                    .class("decrement")
            });

        state.style.insert_element(entity, "spinner");

        entity
    }
}

impl EventHandler for Spinner {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) -> bool {
        /*
        if let Some(numedit_event) = event.message.downcast::<SpinnerEvent>() {
            match numedit_event {
                SpinnerEvent::Increase => {
                    if event.target == self.increment {
                        self.value += self.inc_value;

                        // if self.value >= 1.0 {
                        //     self.value = 1.0;
                        // }

                        // if self.value <= 0.0 {
                        //     self.value = 0.0;
                        // }

                        let val_str = format!("{:.*}", 5, &self.value.to_string());

                        self.textbox.set_text(state, &val_str);

                        state.insert_event(
                            Event::new(SpinnerEvent::ValueChanged(self.value)).target(entity),
                        );

                        state.insert_event(
                            Event::new(WindowEvent::Restyle).target(Entity::new(0, 0)),
                        );
                    }
                }

                SpinnerEvent::Decrease => {
                    if event.target == self.decrement {
                        self.value -= self.inc_value;

                        let val_str = format!("{:.*}", 5, &self.value.to_string());

                        self.textbox.set_text(state, &val_str);

                        state.insert_event(
                            Event::new(SpinnerEvent::ValueChanged(self.value)).target(entity),
                        );

                        state.insert_event(
                            Event::new(WindowEvent::Restyle).target(Entity::new(0, 0)),
                        );
                    }
                }

                _ => {}
            }
        }
        */

        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::MouseDown(button) => {
                    if *button == MouseButton::Left {
                    
                        if event.target == self.increment {
                            self.value += self.inc_value;

                            let val_str = format!("{:.*}", 5, &self.value.to_string());

                            self.textbox.set_text(state, &val_str);

                            state.insert_event(
                                Event::new(SpinnerEvent::ValueChanged(self.value)).target(entity),
                            );
                        }

                        if event.target == self.decrement {
                            self.value -= self.inc_value;

                            let val_str = format!("{:.*}", 5, &self.value.to_string());

                            self.textbox.set_text(state, &val_str);

                            state.insert_event(
                                Event::new(SpinnerEvent::ValueChanged(self.value)).target(entity),
                            );
                        }
                    }
                }

                _=> {}
            }
        }
        

        if let Some(textbox_event) = event.message.downcast::<TextboxEvent>() {
            match textbox_event {
                TextboxEvent::ValueChanged(text) => {
                    if event.target == self.textbox {
                        if let Ok(value) = text.parse::<f32>() {
                            let val = value;
                            // if val <= 0.0 {
                            //     val = 0.0;
                            // }
                            // if val >= 1.0 {
                            //     val = 1.0;
                            // }

                            let val_str = format!("{:.*}", 5, &val.to_string());
                            state.insert_event(
                                Event::new(TextboxEvent::SetValue(val_str))
                                    .target(self.textbox)
                                    .propagate(Propagation::Direct),
                            );

                            self.value = val;

                            state.insert_event(
                                Event::new(SpinnerEvent::ValueChanged(val)).target(entity),
                            );
                        } else {
                            state.insert_event(
                                Event::new(TextboxEvent::ResetValue)
                                    .target(self.textbox)
                                    .propagate(Propagation::Direct),
                            );
                        }
                    }
                }

                _ => {}
            }
        }

        false
    }
}
