#![allow(dead_code)]

const ICON_LEFT_OPEN_BIG: &str = "\u{e761}";
const ICON_RIGHT_OPEN_BIG: &str = "\u{e762}";

const ICON_DOWN_OPEN_MINI: &str = "\u{e760}";
const ICON_UP_OPEN_MINI: &str = "\u{e763}";

use crate::widgets::*;
use crate::style::*;
use crate::widgets::{Element, Textbox, TextboxEvent};
use num_traits::{Bounded, Num, One, CheckedSub, CheckedAdd};

pub struct Spinbox<T> {
    pub value: T,
    pub textbox: Entity,
    pub increment: Entity,
    pub decrement: Entity,

    pub increment_value: T,
    pub decrement_value: T,

    pub min: T,
    pub max: T,

    // Callback triggered when the spinbox is incremented
    on_increment: Option<Box<dyn Fn(&mut Self, &mut State, Entity)>>,
    // Callback triggered when the spinbox is decremented
    on_decrement: Option<Box<dyn Fn(&mut Self, &mut State, Entity)>>,
    // Callback triggered when the value is changed
    on_change: Option<Box<dyn Fn(&mut Self, &mut State, Entity)>>,
    // Callback triggered when the spinbox value reaches max
    on_max: Option<Box<dyn Fn(&mut Self, &mut State, Entity)>>,
    // Callback triggered when the spinbox value reaches min
    on_min: Option<Box<dyn Fn(&mut Self, &mut State, Entity)>>,
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
        + CheckedAdd
        + CheckedSub
{
    pub fn new(initial_value: T) -> Self {
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

    /// Set the increment value of the spinbox
    pub fn with_increment(mut self, increment_value: T) -> Self {
        self.increment_value = increment_value;
        self
    }

    /// Set the decrement value of the spinbox
    pub fn with_decrement(mut self, decrement_value: T) -> Self {
        self.decrement_value = decrement_value;
        self
    }

    /// Set the minimum value of the spinbox
    pub fn with_min(mut self, min_value: T) -> Self {
        self.min = min_value;
        self
    }

    /// Set the maximum value of the spinbox
    pub fn with_max(mut self, max_value: T) -> Self {
        self.max = max_value;
        self
    }

    /// Set callback triggered when the spinbox is incremented
    pub fn on_increment<F>(mut self, callback: F) -> Self
    where
        F: Fn(&mut Self, &mut State, Entity),
        F: 'static,
    {
        self.on_increment = Some(Box::new(callback));
        self
    }

    /// Set callback triggered when the spinbox is decremented
    pub fn on_decrement<F>(mut self, callback: F) -> Self
    where
        F: Fn(&mut Self, &mut State, Entity),
        F: 'static,
    {
        self.on_decrement = Some(Box::new(callback));
        self
    }

    // Set callback triggered when the value is changed
    pub fn on_change<F>(mut self, callback: F) -> Self
    where
        F: 'static + Fn(&mut Self, &mut State, Entity),
    {
        self.on_change = Some(Box::new(callback));
        self
    }

    // Set callback triggered when the spinbox value reaches the maximum value
    pub fn on_max<F>(mut self, callback: F) -> Self 
    where
        F: 'static + Fn(&mut Self, &mut State, Entity),
    {
        self.on_max = Some(Box::new(callback));
        self
    }

    // Set callback triggered when the spinbox value reaches the minimum value
    pub fn on_min<F>(mut self, callback: F) -> Self 
    where
        F: 'static + Fn(&mut Self, &mut State, Entity),
    {
        self.on_min = Some(Box::new(callback));
        self
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
        + std::ops::AddAssign
        + std::ops::SubAssign
        + std::cmp::PartialOrd
        + CheckedAdd
        + CheckedSub
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
            .set_layout_type(state, LayoutType::Row);

        self.textbox =
            Textbox::new(&self.value.to_string()).build(state, entity, |builder| builder);

        let arrow_container = Element::new().build(state, entity, |builder| {
            builder
                .set_width(Pixels(20.0))
                .set_layout_type(LayoutType::Column)
                .class("arrow_container")
        });

        self.increment = Element::new()
            .build(state, arrow_container, |builder| {
                builder
                    .set_font("icons")
                    .set_child_space(Stretch(1.0))
                    .set_text(ICON_UP_OPEN_MINI)
                    .set_height(Stretch(1.0))
                    .class("increment")
            });

        self.decrement = Element::new()
            .build(state, arrow_container, |builder| {
                builder
                    .set_font("icons")
                    .set_child_space(Stretch(1.0))
                    .set_text(ICON_DOWN_OPEN_MINI)
                    .set_height(Stretch(1.0))
                    .class("decrement")
            });

        if let Some(callback) = self.on_change.take() {
            (callback)(self, state, entity);
            self.on_change = Some(callback);
        }

        entity.set_element(state, "spinbox")
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {

        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::MouseDown(button) => {
                    if *button == MouseButton::Left {
                        if event.target == self.increment {
                           
                            if let Some(new_value) = self.value.checked_add(&self.increment_value) {
                                self.value = new_value;
                                if self.value >= self.max {
                                    self.value = self.max;
                                    if let Some(callback) = self.on_max.take() {
                                        (callback)(self, state, entity);
                                        self.on_max = Some(callback);
                                    }
                                }
                            } else {
                                self.value = self.max;
                                if let Some(callback) = self.on_max.take() {
                                    (callback)(self, state, entity);
                                    self.on_max = Some(callback);
                                }
                            }

                            let val_str = format!("{:.*}", 5, &self.value.to_string());

                            self.textbox.set_text(state, &val_str);

                            if let Some(callback) = self.on_increment.take() {
                                (callback)(self, state, entity);
                                self.on_increment = Some(callback);
                            }

                            if let Some(callback) = self.on_change.take() {
                                (callback)(self, state, entity);
                                self.on_change = Some(callback);
                            }

                            event.consume();
                        }

                        if event.target == self.decrement {
                            //self.value -= self.decrement_value;

                           
                            
                            if let Some(new_value) = self.value.checked_sub(&self.decrement_value) {
                                self.value = new_value;
                                if self.value <= self.min {
                                    self.value = self.min;
                                    if let Some(callback) = self.on_min.take() {
                                        (callback)(self, state, entity);
                                        self.on_min = Some(callback);
                                    }
                                }
                            } else {
                                self.value = self.min;
                                if let Some(callback) = self.on_min.take() {
                                    (callback)(self, state, entity);
                                    self.on_min = Some(callback);
                                }
                            }

                            let val_str = format!("{:.*}", 5, &self.value.to_string());

                            self.textbox.set_text(state, &val_str);

                            if let Some(callback) = self.on_decrement.take() {
                                (callback)(self, state, entity);
                                self.on_decrement = Some(callback);
                            }

                            if let Some(callback) = self.on_change.take() {
                                (callback)(self, state, entity);
                                self.on_change = Some(callback);
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
                        if let Ok(mut value) = text.parse::<T>() {

                            if value <= self.min {
                                value = self.min;
                            }
                            if value >= self.max {
                                value = self.max;
                            }

                            let val_str = format!("{:.*}", 5, &value.to_string());
                            state.insert_event(
                                Event::new(TextboxEvent::SetValue(val_str))
                                    .target(self.textbox)
                                    .propagate(Propagation::Direct),
                            );

                            self.value = value;

                            if let Some(callback) = self.on_change.take() {
                                (callback)(self, state, entity);
                                self.on_change = Some(callback);
                            }
                        } else {
                            state.insert_event(Event::new(TextboxEvent::SetValue(self.value.to_string())).target(self.textbox));
                        }
                    }
                }

                _ => {}
            }
        }
    }
}
