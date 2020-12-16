#![allow(dead_code)]

use crate::widgets::*;

use crate::events::{BuildHandler, EventHandler};
use crate::state::style::FlexDirection;

pub struct HBox {}

impl HBox {
    pub fn new() -> Self {
        HBox {}
    }
}

impl BuildHandler for HBox {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity.set_flex_direction(state, FlexDirection::Row);

        entity.set_element(state, "hbox");

        entity
    }
}

impl EventHandler for HBox {}

pub struct VBox {}

impl VBox {
    pub fn new() -> Self {
        VBox {}
    }
}

impl BuildHandler for VBox {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity.set_flex_direction(state, FlexDirection::Column);

        entity.set_element(state, "vbox");

        entity
    }
}

impl EventHandler for VBox {}
