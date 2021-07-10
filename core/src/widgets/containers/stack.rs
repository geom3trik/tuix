// A container for a stack of widgets where only one widget is visible at a time

use crate::{IntoChildIterator, widgets::*};
use crate::style::*;

#[derive(Debug, Clone, PartialEq)]
pub enum StackEvent {
    SetIndex(i32),
    IndexChanged(i32, Entity),
}

pub struct Stack {
    current_index: i32,
    pages: Vec<Entity>,
    on_index_changed: Option<Box<dyn Fn(&mut Self, &mut State, Entity)>>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            current_index: -1,
            pages: Vec::new(),
            on_index_changed: None,
        }
    }

    pub fn on_index_changed<F>(mut self, callback: F) -> Self 
    where F: 'static + Fn(&mut Self, &mut State, Entity)
    {
        self.on_index_changed = Some(Box::new(callback));

        self
    }    

    // Getters
    
    pub fn get_current_index(&self) -> i32 {
        self.current_index
    }

    pub fn get_num_pages(&self) -> usize {
        self.pages.len()
    }

    // Setters

    pub fn set_current_index(&mut self, state: &mut State, entity: Entity, new_index: i32) {
        if new_index != self.current_index {
            self.current_index = new_index;

            if let Some(current_child) = state.hierarchy.get_child(entity, self.current_index as usize) {
                for page in self.pages.iter() {
                    page.set_display(state, Display::None);
                }

                current_child.set_display(state, Display::Flexbox);
                
            }

            if let Some(callback) = self.on_index_changed.take() {
                (callback)(self, state, entity);
                self.on_index_changed = Some(callback);
            }
        }
        
    }


}

impl Widget for Stack {
    type Ret = Entity;
    type Data = ();
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity.set_element(state, "stack")
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::ChildAdded(child) => {
                    if self.current_index != 0 {
                        child.set_display(state, Display::None);
                    }
                    self.pages.push(*child);
                    self.set_current_index(state, entity, self.pages.len() as i32 - 1);
                }
                _=> {}
            }
        }

        if let Some(stack_event) = event.message.downcast() {
            match stack_event {
                StackEvent::SetIndex(index) => {
                    self.set_current_index(state, entity, *index)
                }

                _=> {}
            }
        }
    }
}