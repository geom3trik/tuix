#![allow(dead_code)]

const ICON_CHECK: &str = "\u{2713}";

use crate::{Entity, HierarchyTree};

use crate::{BuildHandler, Event, EventHandler, Propagation};
use crate::{PropSet, State};

use crate::widgets::*;

pub struct Radio {
    marker: Entity,
    check: CheckButton,
}

impl Radio {
    pub fn new() -> Self {
        Self {
            marker: Entity::null(),
            check: CheckButton::new(false),
        }
    }

    pub fn on_checked(mut self, event: Event) -> Self {
        self.check = self.check.on_checked(event);
        self
    }

    pub fn on_unchecked(mut self, event: Event) -> Self {
        self.check = self.check.on_unchecked(event);
        self
    }
}

impl BuildHandler for Radio {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        self.marker = Element::new().build(state, entity, |builder| {
            builder.set_hoverability(false).class("marker").set_hoverability(false)
        });

        entity.set_element(state, "radio")
    }
}

impl EventHandler for Radio {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        self.check.on_event(state, entity, event);
    }
}

// pub struct RadioButton {
//     check: CheckButton,
// }

// impl RadioButton {
//     pub fn new() -> Self {
//         Self {
//             check: CheckButton::new(false),
//         }
//     }

//     pub fn on_checked(mut self, event: Event) -> Self {
//         self.check = self.check.on_checked(event);

//         self
//     }

//     pub fn on_unchecked(mut self, event: Event) -> Self {
//         self.check = self.check.on_unchecked(event);

//         self
//     }
// }

// impl BuildHandler for RadioButton {
//     type Ret = Entity;
//     fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
//         self.check.on_build(state, entity);
//         entity.set_element(state, "radio_button").set_font(state, "sans")
//     }
// }

// impl EventHandler for RadioButton {
//     fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
//         self.check.on_event(state, entity, event);
//     }
// }
