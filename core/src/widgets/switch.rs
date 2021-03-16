#![allow(dead_code)]



use crate::widgets::*;
use crate::widgets::{Element, Checkbox};

pub struct Switch {
    front: Entity,
    checkbox: Checkbox,
}

impl Switch {
    pub fn new(checked: bool) -> Self {
        Switch {
            front: Entity::null(),
            checkbox: Checkbox::new(checked).with_icon_checked("").with_icon_unchecked(""),
        }
    }
}

impl Widget for Switch {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {


        self.checkbox.on_build(state, entity);

        self.front = Element::new().build(state, entity, |builder| {
            builder.class("front").set_hoverability(false)
        });

        state.style.insert_element(entity, "switch");

        entity
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        self.checkbox.on_event(state, entity, event);
    }
}
