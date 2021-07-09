#![allow(dead_code)]

use std::marker::PhantomData;

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

pub trait Lens {

    type Source;
    type Target;

    fn view(&self, data: &Self::Source) -> Self::Target;
}

#[derive(Debug, Clone, PartialEq)]
pub enum BindEvent {
    Bind(Entity),
    Update,
}


// A wrapper on a widget which adds the setup for binding as well as the conversion of data + lensing
pub struct Wrapper<L: Lens, W: Widget, > {

    widget: W,
    lens: L,
    converter: Box<dyn Fn(&<L as Lens>::Target) -> <W as Widget>::Data>,
}

impl<L: Lens, W: Widget> Wrapper<L,W> {
    pub fn new<F>(widget: W, lens: L, converter: F) -> Self 
    where F: 'static + Fn(&<L as Lens>::Target) -> <W as Widget>::Data
    {
        Self {

            widget,
            lens,
            converter: Box::new(converter),
        }
    }
}

impl<L: 'static + Lens, W: Widget> Widget for Wrapper<L,W> {
    type Ret = <W as Widget>::Ret;
    type Data = <L as Lens>::Source;

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        state.insert_event(Event::new(BindEvent::Bind(entity)).target(entity).propagate(Propagation::Up));

        self.widget.on_build(state, entity)
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        self.widget.on_event(state, entity, event)
    }

    fn on_update(&mut self, state: &mut State, entity: Entity, data: &Self::Data) {
        // Apply the lens
        let view_data = self.lens.view(data);
        // Apply the converter function
        let value = (self.converter)(&view_data);
        // Update the underlying widget with the lensed and converted data
        self.widget.on_update(state, entity, &value);
    }
}