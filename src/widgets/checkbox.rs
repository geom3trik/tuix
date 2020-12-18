#![allow(dead_code)]

use crate::entity::Entity;
use crate::mouse::*;
use crate::{BuildHandler, Event, EventHandler, Propagation, WindowEvent};
use crate::{PropSet, State};

use crate::style::layout::{Align, Justify};

use crate::widgets::radio::RadioEvent;

const ICON_CHECK: &str = "\u{2713}";
const ICON_DOWN_OPEN_BIG: &str = "\u{e764}";
const ICON_RIGHT_OPEN_BIG: &str = "\u{e766}";
const ICON_FLOPPY_DISK: &str = "\u{1f4be}";

const ICON_DOWN_OPEN_MINI: &str = "\u{e760}";

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CheckboxEvent {
    Check,
    Uncheck,
    Switch,
    Checked,
    Unchecked,
}

pub enum CheckEvent {
    Switch(Entity),
}

pub struct CheckList {}

impl CheckList {
    pub fn new() -> Self {
        CheckList {}
    }
}

impl BuildHandler for CheckList {
    type Ret = Entity;
    fn on_build(&mut self, _state: &mut State, entity: Entity) -> Self::Ret {
        entity
    }
}

impl EventHandler for CheckList {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) -> bool {
        // let origin = event.origin;

        // if let Some(check_event) = event.is_type::<CheckEvent>() {
        //     match check_event {
        //         CheckEvent::Switch(check) => {
        //             if origin != entity {
        //                 state.insert_event(Event::new(CheckEvent::Switch(*check)).target(entity).propagate(Propagation::Fall));
        //             }
        //         }
        //     }
        // }

        false
    }
}

#[derive(Clone)]
pub struct Checkbox {
    checked: bool,

    icon_unchecked: String,
    icon_checked: String,

    group_name: String,
}

impl Checkbox {
    pub fn new(checked: bool) -> Self {
        Checkbox {
            checked,
            icon_unchecked: "".to_string(),
            icon_checked: ICON_CHECK.to_string(),
            group_name: "".to_string(),
        }
    }

    fn checked(mut self, flag: bool) -> Self {
        self.checked = flag;

        self
    }

    pub fn with_icon_checked(mut self, icon_checked: &str) -> Self {
        self.icon_checked = icon_checked.to_string();

        self
    }

    pub fn with_icon_unchecked(mut self, icon_unchecked: &str) -> Self {
        self.icon_unchecked = icon_unchecked.to_string();

        self
    }

    fn switch(&mut self, state: &mut State, id: Entity) {
        if self.checked {
            self.checked = false;
            id.set_text(state, &self.icon_unchecked);
            id.set_checked(state, false);
        //.set_border_color(state, nanovg::Color::from_rgb(144, 144, 144))
        //.set_background_color(state, nanovg::Color::from_rgb(56, 56, 56));
        } else {
            self.checked = true;
            id.set_text(state, &self.icon_checked);
            id.set_checked(state, true);
            //.set_border_color(state, nanovg::Color::from_rgb(42, 152, 240))
            //.set_background_color(state, nanovg::Color::from_rgb(42, 152, 240));
        }
    }
}

impl BuildHandler for Checkbox {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
            // .set_width(state, Length::Pixels(15.0))
            // .set_height(state, Length::Pixels(15.0))
            // .set_flex_basis(state, 15.0)
            // .set_flex_grow(state, 0.0)
            // .set_flex_shrink(state, 0.0)
            .set_font(state, "Icons".to_string())
            .set_font_size(state, 14.0)
            .set_text(
                state,
                if self.checked {
                    &self.icon_checked
                } else {
                    &self.icon_unchecked
                },
            )
            .set_text_justify(state, Justify::Center)
            .set_text_align(state, Align::Center);
        //.set_text_margin_left(state, 30.0)
        //.set_border_width(state, 1.0)
        //.set_border_color(state, Color::rgb(0, 0, 0));

        state.style.insert_element(entity, "checkbox");

        entity
    }
}

impl EventHandler for Checkbox {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) -> bool {

        if let Some(checkbox_event) = event.message.downcast::<CheckboxEvent>() {
            match checkbox_event {
                CheckboxEvent::Switch => {
                    if !self.checked {
                        state.insert_event(
                            Event::new(RadioEvent::Activate(entity, self.group_name.clone()))
                                .target(entity)
                                .propagate(Propagation::Up),
                        );
                    }

                    if event.target == entity {
                        self.switch(state, entity);
                    }
                    
                }

                CheckboxEvent::Check => {
                    if event.target == entity {
                        self.checked = true;
                        entity.set_text(state, &self.icon_checked);
                        entity.set_checked(state, true);
                    }
                }

                CheckboxEvent::Uncheck => {
                    if event.target == entity {
                        self.checked = false;
                        entity.set_text(state, &self.icon_unchecked);
                        entity.set_checked(state, false);
                    }
                }

                _ => {}
            }
        }

        if let Some(radio_event) = event.is_type::<RadioEvent>() {
            match radio_event {
                RadioEvent::Activate(radio, group) => {
                    if *radio != entity && group == &self.group_name {
                        self.checked = false;
                        entity.set_text(state, &self.icon_unchecked);
                        entity.set_checked(state, false);
                    }
                }
            }
        }

        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::MouseUp(button) => match button {
                    MouseButton::Left => {
                        if entity == event.target {
                            if self.checked {
                                state.insert_event(
                                    Event::new(CheckboxEvent::Unchecked).target(entity),
                                );
                            } else {
                                state.insert_event(
                                    Event::new(CheckboxEvent::Checked).target(entity),
                                );
                                state.insert_event(
                                    Event::new(RadioEvent::Activate(
                                        entity,
                                        self.group_name.clone(),
                                    ))
                                    .target(entity)
                                    .propagate(Propagation::Up),
                                );
                            }

                            self.switch(state, entity);

                            //state.insert_event(Event::new(WindowEvent::Restyle));
                            //state.insert_event(Event::new(WindowEvent::Redraw));

                            // if let Some(mut on_press) = self.on_press.clone() {
                            //     on_press.target = id;
                            //     state.insert_event(on_press);
                            // }
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
