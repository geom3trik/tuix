use std::{any::TypeId, collections::HashSet};

use crate::{BindEvent, Entity, Event, Model, Node, PropSet, State, Widget};


/// A widget which wraps [Model] data and is responsibe for binding widgets to the data.
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

    pub fn get(&self) -> Option<&D> {
        Some(&self.data_widget)
    }
}

impl<D: Model + Node> Widget for Store<D> {
    type Ret = Entity;
    type Data = ();
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity.set_hoverable(state, false).set_focusable(state, false)
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        
        if let Some(bind_event) = event.message.downcast() {
            match bind_event {
                BindEvent::Bind(target, type_id) => {
                    //println!("Bind: {}", target);
                    if *type_id == TypeId::of::<D>() {
                        //println!("Compatible");
                        self.observers.insert(*target);
                        //entity.emit(state, BindEvent::Update);
                        if let Some(mut event_handler) = state.event_handlers.remove(target) {
                            event_handler.on_update_(state, *target, &self.data_widget);
    
                            state.event_handlers.insert(*target, event_handler);
                        }
                        event.consume();
                    } 
                    // else {
                    //     println!("Not Compatible");
                    // }
                    
                }

                BindEvent::Update => {
                    for observer in self.observers.iter() {
                        if *observer != event.origin {
                            if let Some(mut event_handler) = state.event_handlers.remove(observer) {
                                event_handler.on_update_(state, *observer, &self.data_widget);

                                state.event_handlers.insert(*observer, event_handler);
                            }
                        } 
                    }                        
                    
                }
            }
        }

        //println!("Origin: {} Observers: {:?}", event.origin, self.observers);

        //if self.observers.contains(&event.origin) {
            self.data_widget.on_event(state, entity, event);
        //}
    }
}