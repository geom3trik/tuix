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

use crate::widgets::{Element, Textbox, TextboxEvent};

use num_traits::{Bounded, Num, One};

// #[derive(Debug, Clone, PartialEq)]
// pub enum SpinnerEvent {
//     Increase,
//     Decrease,
//     SetValue(f32),
//     ValueChanged(f32),
// }

//impl Message for NumEditEvent {}

pub struct Spinbox<T> {
    pub value: T,
    pub textbox: Entity,
    pub increment: Entity,
    pub decrement: Entity,

    pub increment_value: T,
    pub decrement_value: T,

    min: T,
    max: T,

    // Triggered when the spinner is incremented
    on_increment: Option<Box<dyn Fn(T) -> Event>>,
    // Triggered when the spinner is decremented
    on_decrement: Option<Box<dyn Fn(T) -> Event>>,
    // Triggered when the value is changed
    on_change: Option<Box<dyn Fn(T) -> Event>>,
    // Triggered when the spinner value reaches max
    on_max: Option<Event>,
    // Triggered when the spinner value reaches min
    on_min: Option<Event>,
}

impl<T> Spinbox<T>
where
    T: 'static
        + Default
        + std::fmt::Debug
        + std::fmt::Display
        + Copy
        + PartialEq
        + std::str::FromStr
        + Num
        + One
        + Bounded
        + std::ops::AddAssign
        + std::ops::SubAssign
{
    pub fn new(initial_value: T) -> Self {
        // entity.set_text(state, "Test".to_string())
        //     .set_background(state, nanovg::Color::from_rgb(100, 50, 50));

        Spinbox {
            value: initial_value,
            increment_value: T::one(),
            decrement_value: T::one(),

            min: T::min_value(),
            max: T::max_value(),

            textbox: Entity::null(),
            increment: Entity::null(),
            decrement: Entity::null(),

            on_increment: None,
            on_decrement: None,
            on_change: None,
            on_max: None,
            on_min: None,
        }
    }

    pub fn with_increment(mut self, increment_value: T) -> Self {
        self.increment_value = increment_value;
        self
    }

    pub fn with_decrement(mut self, decrement_value: T) -> Self {
        self.decrement_value = decrement_value;
        self
    }

    pub fn with_min(mut self, min_value: T) -> Self {
        self.min = min_value;
        self
    }

    pub fn with_max(mut self, max_value: T) -> Self {
        self.max = max_value;
        self
    }

    pub fn on_increment<F>(mut self, message: F) -> Self
    where
        F: Fn(T) -> Event,
        F: 'static,
    {
        self.on_increment = Some(Box::new(message));
        self
    }

    pub fn on_decrement<F>(mut self, message: F) -> Self
    where
        F: Fn(T) -> Event,
        F: 'static,
    {
        self.on_decrement = Some(Box::new(message));
        self
    }

    pub fn on_change<F>(mut self, message: F) -> Self
    where
        F: Fn(T) -> Event,
        F: 'static,
    {
        self.on_change = Some(Box::new(message));
        self
    }

    pub fn on_max(mut self, event: Event) -> Self {
        self.on_max = Some(event);
        self
    }

    pub fn on_min(mut self, event: Event) -> Self {
        self.on_min = Some(event);
        self
    }
}

impl<T> BuildHandler for Spinbox<T>
where
    T: 'static
        + Default
        + std::fmt::Debug
        + std::fmt::Display
        + Copy
        + PartialEq
        + std::str::FromStr
        + Num
        + One
        + std::ops::AddAssign
        + std::ops::SubAssign
        + std::cmp::PartialOrd
{
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        if self.value <= self.min {
            self.value = self.min;
        }

        if self.value >= self.max {
            self.value = self.max;
        }

        entity
            .set_display(state, Display::Flexbox)
            .set_flex_direction(state, FlexDirection::Row);

        self.textbox = Textbox::new(&self.value.to_string())
            .build(state, entity, |builder| builder.set_flex_grow(1.0));

        let arrow_container = Element::new().build(state, entity, |builder| {
            builder
                .set_width(Length::Pixels(20.0))
                .set_flex_grow(0.0)
                .class("arrow_container")
        });

        self.increment = Element::new()
            //.on_press(Event::new(SpinnerEvent::Increase))
            .build(state, arrow_container, |builder| {
                builder
                    .set_font("icons")
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
                    .set_font("icons")
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

impl<T> EventHandler for Spinbox<T>
where
    T: 'static
        + Default
        + std::fmt::Debug
        + std::fmt::Display
        + Copy
        + PartialEq
        + std::str::FromStr
        + Num
        + std::ops::AddAssign
        + std::ops::SubAssign
        + std::cmp::PartialOrd
{
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
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
                            self.value += self.increment_value;

                            if self.value <= self.min {
                                self.value = self.min;
                                if let Some(mut on_min) = self.on_min.clone() {
                                    if !on_min.target {
                                        on_min.target = entity;
                                    }

                                    on_min.origin = entity;
                                    state.insert_event(on_min);
                                }
                            }

                            if self.value >= self.max {
                                self.value = self.max;
                                if let Some(mut on_max) = self.on_max.clone() {
                                    if !on_max.target {
                                        on_max.target = entity;
                                    }

                                    on_max.origin = entity;
                                    state.insert_event(on_max);
                                }
                            }

                            let val_str = format!("{:.*}", 5, &self.value.to_string());

                            self.textbox.set_text(state, &val_str);

                            if let Some(on_increment) = &self.on_increment {
                                let mut event = (on_increment)(self.value);
                                if !event.target {
                                    event.target = entity;
                                }

                                event.origin = entity;
                                state.insert_event(event);
                            }

                            event.consume();
                        }

                        if event.target == self.decrement {
                            self.value -= self.decrement_value;

                            if self.value <= self.min {
                                self.value = self.min;
                                if let Some(mut on_min) = self.on_min.clone() {
                                    if !on_min.target {
                                        on_min.target = entity;
                                    }

                                    on_min.origin = entity;
                                    state.insert_event(on_min);
                                }
                            }

                            if self.value >= self.max {
                                self.value = self.max;
                                if let Some(mut on_max) = self.on_max.clone() {
                                    if !on_max.target {
                                        on_max.target = entity;
                                    }

                                    on_max.origin = entity;
                                    state.insert_event(on_max);
                                }
                            }

                            let val_str = format!("{:.*}", 5, &self.value.to_string());

                            self.textbox.set_text(state, &val_str);

                            if let Some(on_decrement) = &self.on_decrement {
                                let mut event = (on_decrement)(self.value);
                                if !event.target {
                                    event.target = entity;
                                }

                                event.origin = entity;
                                state.insert_event(event);
                            }

                            event.consume();
                        }
                    }
                }

                _ => {}
            }
        }

        if let Some(textbox_event) = event.message.downcast::<TextboxEvent>() {
            match textbox_event {
                TextboxEvent::ValueChanged(text) => {
                    if event.target == self.textbox {
                        if let Ok(value) = text.parse::<T>() {
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

                            if let Some(on_change) = &self.on_change {
                                let mut event = (on_change)(self.value);
                                if !event.target {
                                    event.target = entity;
                                }

                                event.origin = entity;
                                state.insert_event(event);
                            }
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
    }
}
