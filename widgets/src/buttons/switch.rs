#![allow(dead_code)]

use crate::widgets::*;
use crate::widgets::{Checkbox, Element};

pub struct Switch {
    front: Entity,
    checkbox: Checkbox,
}

impl Switch {
    pub fn new(checked: bool) -> Self {
        Switch {
            front: Entity::null(),
            checkbox: Checkbox::new(checked)
                .with_icon_checked("")
                .with_icon_unchecked(""),
        }
    }

    pub fn on_checked<F>(mut self, callback: F) -> Self 
    where
        F: 'static + Fn(&mut Checkbox, &mut State, Entity)
    {
        self.checkbox = self.checkbox.on_checked(callback);

        self
    }

    pub fn on_unchecked<F>(mut self, callback: F) -> Self 
    where
        F: 'static + Fn(&mut Checkbox, &mut State, Entity)
    {
        self.checkbox = self.checkbox.on_unchecked(callback);

        self
    }
}

impl Widget for Switch {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        self.checkbox.on_build(state, entity);

        self.front = Element::new().build(state, entity, |builder| {
            builder.class("front").set_hoverable(false)
        });

        entity
            .set_child_top(state, Stretch(1.0))
            .set_child_bottom(state, Stretch(1.0))
            .set_element(state, "switch")
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        self.checkbox.on_event(state, entity, event);
    }
}
