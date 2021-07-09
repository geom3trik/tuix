#![allow(dead_code)]

use std::marker::PhantomData;

use crate::widgets::*;
use crate::NodeMap;

pub struct Label {
    text: String,
    // a: PhantomData<*const T>,

    // converter: Option<Box<dyn Fn(&T) -> String>>
}

impl Label {
    pub fn new(text: &str) -> Self {
        Label {
            text: text.to_string(),
            // a: PhantomData::default(),

            // converter: None,
        }
    }

    pub fn bind<L: Lens, F>(self, something: L, converter: F) -> Wrapper<<L as Lens>::Target, Self> 
    where F: 'static + Fn(&<L as Lens>::Target) -> <Self as Widget>::Data
    {
        Wrapper::new(self, converter)
    }

    // pub fn with_converter<F>(mut self, converter: F) -> Self 
    // where F: 'static + Fn(&T) -> String
    // {
    //     self.converter = Some(Box::new(converter));
    //     self
    // }
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

    fn on_update(&mut self, state: &mut State, entity: Entity, data: &Self::Data, nodes: &NodeMap) {
        //if let Some(converter) = &self.converter {
            // self.text = (converter)(data);
            self.text = data.to_owned();
            entity.set_text(state, &self.text);
        //}
    }
}

pub trait Lens {

    type Source;
    type Target;

    fn view(&self, data: &Self::Source) -> Self::Target;
}


pub struct Wrapper<T, W: Widget> {

    widget: W,
    converter: Box<dyn Fn(&T) -> <W as Widget>::Data>,

    t: PhantomData<T>,
    w: PhantomData<W>,
}

impl<T, W: Widget> Wrapper<T,W> {
    pub fn new<F>(widget: W, converter: F) -> Self 
    where F: 'static + Fn(&T) -> <W as Widget>::Data
    {
        Self {

            widget,
            converter: Box::new(converter),

            t: PhantomData::default(),
            w: PhantomData::default(),
        }
    }
}

impl<T: 'static, W: Widget> Widget for Wrapper<T,W> {
    type Ret = <W as Widget>::Ret;
    type Data = T;

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        self.widget.on_build(state, entity)
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        self.widget.on_event(state, entity, event)
    }

    fn on_update(&mut self, state: &mut State, entity: Entity, data: &Self::Data, nodes: &NodeMap) {
        // Do something here
        let value = (self.converter)(data);
        self.widget.on_update(state, entity, &value, nodes);
    }
}