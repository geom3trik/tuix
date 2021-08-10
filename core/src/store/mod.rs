

pub mod node;
pub mod lens;
use std::{any::TypeId, collections::HashSet};

pub use node::*;
pub use lens::*;

use crate::{EventHandler, IntoChildIterator, PropType, widgets::*};

pub trait Model {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {}

    /// Adds the widget into state and returns the associated type Ret - an entity id or a tuple of entity ids
    fn build(mut self, state: &mut State, parent: Entity) -> Entity
    where Self: std::marker::Sized + Model + Node
    {

        Store::new(self).build(state, parent, |builder| builder)
        // Create a new entity
        //let entity = state.add(parent);

        // Call the on_build function of the widget
        //let ret = self.on_build(state, entity);

        // Call the builder closure
        //builder(Builder::new(state, entity)).build(self);

        // Return the entity or entities returned by the on_build method
        //ret
    }
}

// impl<T> EventHandler for T 
// where T: Model
// {
//     fn on_update(&mut self, state: &mut State, entity: Entity, node: &dyn Node) {}

//     fn on_event_(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
//         <T as Model>::on_event(self, state, entity, event);
//     }

//     fn on_style(&mut self, state: &mut State, entity: Entity, property: (String, PropType)) {}

//     fn on_draw_(&mut self, state: &mut State, entity: Entity, canvas: &mut Canvas) {}
// }

pub struct Store<D> {
    data_widget: D,
    observers: HashSet<Entity>,
}

impl<D: Model> Store<D> {
    pub fn new(data_widget: D) -> Self {
        Self {
            data_widget,
            observers: HashSet::new(),
        }
    }
}

impl<D: Model + Node> Widget for Store<D> {
    type Ret = Entity;
    type Data = ();
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity.set_hoverability(state, false).set_focusability(state, false)
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        
        if let Some(bind_event) = event.message.downcast() {
            match bind_event {
                BindEvent::Bind(target, type_id) => {
                    println!("Bind: {}", target);
                    if *type_id == TypeId::of::<D>() {
                        println!("Compatible");
                        self.observers.insert(*target);
                        //entity.emit(state, BindEvent::Update);
                        if let Some(mut event_handler) = state.event_handlers.remove(target) {
                            event_handler.on_update(state, *target, &self.data_widget);
    
                            state.event_handlers.insert(*target, event_handler);
                        }
                        event.consume();
                    } else {
                        println!("Not Compatible");
                    }
                    
                }

                BindEvent::Update => {
                    for observer in self.observers.iter() {
                        if *observer != event.origin {
                            if let Some(mut event_handler) = state.event_handlers.remove(observer) {
                                event_handler.on_update(state, *observer, &self.data_widget);

                                state.event_handlers.insert(*observer, event_handler);
                            }
                        } 
                    }                        
                    
                }
            }
        }

        self.data_widget.on_event(state, entity, event);
    }
}



#[derive(Debug, Clone, PartialEq)]
pub enum BindEvent {
    Bind(Entity, TypeId),
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

        let type_id = TypeId::of::<Self::Data>();
        state.insert_event(Event::new(BindEvent::Bind(entity, type_id)).target(entity).propagate(Propagation::Up));

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
        for (index, child) in entity.child_iter(&state.tree.clone()).enumerate() {
            
            if let Some(mut event_handler) = state.event_handlers.remove(&child) {
                event_handler.on_update(state, child, &value);

                state.event_handlers.insert(child, event_handler);
            }
        }

        // Update the underlying widget with the lensed and converted data
        self.widget.on_update(state, entity, &value);
    }

    fn on_draw(&mut self, state: &mut State, entity: Entity, canvas: &mut Canvas) {
        self.widget.on_draw(state, entity, canvas)
    }
}