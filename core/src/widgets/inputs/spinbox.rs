#![allow(dead_code)]

const ICON_LEFT_OPEN_BIG: &str = "\u{e761}";
const ICON_RIGHT_OPEN_BIG: &str = "\u{e762}";

const ICON_DOWN_OPEN_MINI: &str = "\u{e760}";
const ICON_UP_OPEN_MINI: &str = "\u{e763}";

use crate::state::style::*;
use crate::widgets::*;

use crate::widgets::{Element, Textbox, TextboxEvent};

use num_traits::{Bounded, Num, One};

// #[derive(Debug, Clone, PartialEq)]
// pub enum SpinnerEvent {
//     Increase,
//     Decrease,
//     SetValue(f32),
//     ValueChanged(f32),
// }

pub struct Spinbox<T> 
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
        + std::ops::SubAssign,
{

    textbox: Entity,
    increment: Entity,
    decrement: Entity,

    pub increment_value: T,
    pub decrement_value: T,

    value: T,
    min: T,
    max: T,

    // Triggered when the spinner is incremented
    on_increment: Option<Box<dyn Fn(T) -> Event>>,
    // Triggered when the spinner is decremented
    on_decrement: Option<Box<dyn Fn(T) -> Event>>,
    // Triggered when the value is changed
    on_change: Option<Box<dyn Fn(T) -> Event>>,
    // Triggered when the spinner value reaches max
    on_max: Option<Box<dyn Fn(T) -> Event>>,
    // Triggered when the spinner value reaches min
    on_min: Option<Box<dyn Fn(T) -> Event>>,
}

impl<T> Default for Spinbox<T> 
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
        + std::cmp::PartialOrd,
{
    fn default() -> Self {
        Self {
            value: T::zero(),
            increment_value: T::one(),
            decrement_value: T::one(),

            min: T::min_value(),
            max: T::max_value(),

            ..Default::default()
        }
    }
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
        + std::cmp::PartialOrd,
{
    pub fn new() -> Self {
        Spinbox::default()
    }

    pub fn with_initial_value(mut self, value: T) -> Self {
        self.value = value;
        self
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
        F: 'static + Fn(T) -> Event,
    {
        self.on_change = Some(Box::new(message));
        self
    }

    pub fn on_max<F>(mut self, message: F) -> Self 
    where
        F: 'static + Fn(T) -> Event,
    {
        self.on_max = Some(Box::new(message));
        self
    }

    pub fn on_min<F>(mut self, message: F) -> Self 
    where
        F: 'static + Fn(T) -> Event,
    {
        self.on_min = Some(Box::new(message));
        self
    }

    // Helper functions

    // Helper function for sending events in response to on_changing, on_changed, on_min, and on_max
    fn send_value_event<F>(&mut self, state: &mut State, entity: Entity, message: &Option<F>)
    where
        F: 'static + Fn(T) -> Event,
    {
        if let Some(on_event) = &message {
            let mut event = (on_event)(self.value);
            event.origin = entity;

            if event.target == Entity::null() {
                event.target = entity;
            }

            state.insert_event(event);
        }  
    }

    // Helper function for sending events in response to on_press, on_release, on_over, on_out
    fn send_event(&self, state: &mut State, entity: Entity, on_event: Option<Event>) {
        if let Some(mut event) = on_event {
            event.origin = entity;

            if event.target == Entity::null() {
                event.target = entity;
            }

            state.insert_event(event);
        }
    }
}

impl<T> Widget for Spinbox<T>
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
        + std::cmp::PartialOrd,
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
            .set_flex_direction(state, FlexDirection::Row)
            .set_layout_type(state, LayoutType::Horizontal);

        self.textbox = Textbox::new(&self.value.to_string())
            .build(state, entity, |builder| builder.set_flex_grow(1.0));

        let arrow_container = Element::new().build(state, entity, |builder| {
            builder
                .set_width(Pixels(20.0))
                .set_flex_grow(0.0)
                .set_layout_type(LayoutType::Vertical)
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
                    .set_height(Stretch(1.0))
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
                    .set_height(Stretch(1.0))
                    .class("decrement")
            });

        state.style.insert_element(entity, "spinbox");

        entity
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        
        // Handle window events
        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::MouseDown(button) => {
                    if *button == MouseButton::Left {
                        if event.target == self.increment {
                            self.value += self.increment_value;

                            if self.value <= self.min {
                                self.value = self.min;
                                self.send_value_event(state, entity, &self.on_min);
                            }

                            if self.value >= self.max {
                                self.value = self.max;
                                self.send_value_event(state, entity, &self.on_max);
                            }

                            let val_str = format!("{:.*}", 5, &self.value.to_string());
                            self.textbox.set_text(state, &val_str);

                            self.send_value_event(state, entity, &self.on_increment);

                            event.consume();
                        }

                        if event.target == self.decrement {
                            self.value -= self.decrement_value;

                            if self.value <= self.min {
                                self.value = self.min;
                                self.send_value_event(state, entity, &self.on_min);
                            }

                            if self.value >= self.max {
                                self.value = self.max;
                                self.send_value_event(state, entity, &self.on_max);
                            }

                            let val_str = format!("{:.*}", 5, &self.value.to_string());
                            self.textbox.set_text(state, &val_str);

                            self.send_value_event(state, entity, &self.on_decrement);

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
                            let value = value;
                            // if value <= 0.0 {
                            //     value = 0.0;
                            // }
                            // if value >= 1.0 {
                            //     value = 1.0;
                            // }

                            let val_str = format!("{:.*}", 5, &value.to_string());
                            state.insert_event(
                                Event::new(TextboxEvent::SetValue(val_str))
                                    .target(self.textbox)
                                    .propagate(Propagation::Direct),
                            );

                            self.value = value;

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
