

pub mod node;
use std::collections::HashSet;

pub use node::*;

use crate::{IntoChildIterator, widgets::*};

pub struct Store<D> {
    data_widget: D,
    observers: HashSet<Entity>,
}

impl<D: Widget> Store<D> {
    pub fn new(data_widget: D) -> Self {
        Self {
            data_widget,
            observers: HashSet::new(),
        }
    }
}

impl<D: Widget + Node> Widget for Store<D> {
    type Ret = <D as Widget>::Ret;
    type Data = ();
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        self.data_widget.on_build(state, entity)
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        
        if let Some(bind_event) = event.message.downcast() {
            match bind_event {
                BindEvent::Bind(target) => {
                    println!("Bind: {}", target);
                    self.observers.insert(*target);
                    //entity.emit(state, BindEvent::Update);
                    if let Some(mut event_handler) = state.event_handlers.remove(target) {
                        event_handler.on_update(state, *target, &self.data_widget);

                        state.event_handlers.insert(*target, event_handler);
                    }
                }

                BindEvent::Update => {
                    for observer in self.observers.iter() {
                        if let Some(mut event_handler) = state.event_handlers.remove(observer) {
                            event_handler.on_update(state, *observer, &self.data_widget);

                            state.event_handlers.insert(*observer, event_handler);
                        }
                    }                        
                    
                }
            }
        }

        self.data_widget.on_event(state, entity, event);
    }
}

pub trait Lens {

    type Source: Node;
    type Target;

    fn view<'a>(&self, data: &'a Self::Source) -> &'a Self::Target;
}

#[derive(Debug, Clone, PartialEq)]
pub enum BindEvent {
    Bind(Entity),
    Update,
    //Init,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UpdateEvent<'a, T> {
    Update(&'a T),
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

        // if let Some(update_event) = event.message.downcast() {
        //     match update_event {
        //         UpdateEvent::Update::<Self::Data>(value) => {
        //             let view_data = self.lens.view(value);
        //             let value = (self.converter)(&view_data);
        //             entity.emit(state, UpdateEvent::Update(&value));
        //         }
        //     }
        // }

        self.widget.on_event(state, entity, event)
    }

    fn on_update(&mut self, state: &mut State, entity: Entity, data: &Self::Data) {
        // Apply the lens
        let view_data = self.lens.view(data);
        // Apply the converter function
        let value = (self.converter)(&view_data);

        // Update children
        for (index, child) in entity.child_iter(&state.hierarchy.clone()).enumerate() {
            
            if let Some(mut event_handler) = state.event_handlers.remove(&child) {
                event_handler.on_update(state, child, &value);

                state.event_handlers.insert(child, event_handler);
            }
        }

        // Update the underlying widget with the lensed and converted data
        self.widget.on_update(state, entity, &value);
    }
}