#![allow(dead_code)]

use std::marker::PhantomData;

use crate::widgets::*;
use crate::NodeMap;

pub struct Label<T> {
    text: String,
    a: PhantomData<*const T>,

    converter: Option<Box<dyn Fn(&T) -> String>>
}

impl<T> Label<T> {
    pub fn new(text: &str) -> Self {
        Label {
            text: text.to_string(),
            a: PhantomData::default(),

            converter: None,
        }
    }

    pub fn with_converter<F>(mut self, converter: F) -> Self 
    where F: 'static + Fn(&T) -> String
    {
        self.converter = Some(Box::new(converter));
        self
    }
}

impl<T: 'static> Widget for Label<T> {
    type Ret = Entity;
    type Data = T;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
            .set_text(state, &self.text.to_string())
            .set_element(state, "label")
            .set_focusability(state, false)
    }

    fn on_update(&mut self, state: &mut State, entity: Entity, data: &T, nodes: &NodeMap) {
        if let Some(converter) = &self.converter {
            self.text = (converter)(data);
            entity.set_text(state, &self.text);
        }
    }
}