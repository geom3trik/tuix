#![allow(dead_code)]

use std::marker::PhantomData;

use crate::Lens;
use crate::Wrapper;
use crate::widgets::*;
use crate::style::*;

pub struct Label {
    text: String,
}

impl Label {
    pub fn new(text: &str) -> Self {
        Label {
            text: text.to_string(),
        }
    }

    // This method will be part of a trait (maybe the Widget trait)
    pub fn bind<L: Lens, F>(self, lens: L, converter: F) -> Wrapper<L, Self> 
    where F: 'static + Fn(&<L as Lens>::Target) -> <Self as Widget>::Data
    {
        Wrapper::new(self, lens, converter)
    }
}

impl Widget for Label {
    type Ret = Entity;
    type Data = String;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
            .set_text(state, &self.text.to_string())
            .set_element(state, "label")
            .set_focusability(state, false)
    }

    fn on_update(&mut self, state: &mut State, entity: Entity, data: &Self::Data) {
        self.text = data.to_owned();
        entity.set_text(state, &self.text);
    }
}

