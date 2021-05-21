// A container for a stack of widgets where only one widget is visible at a time

use crate::widgets::*;
pub enum StackEvent {
    SetIndex(i32),
    IndexChanged(i32, Entity),
}

pub struct Stack {
    current_index: i32,

    on_index_changed: Option<Box<dyn Fn(&mut Self, &mut State, Entity)>>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            current_index: -1,

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

    // Setters

    pub fn set_current_index(&mut self, state: &mut State, entity: Entity, new_index: i32) {
        if new_index != self.current_index {
            self.current_index = new_index;

            println!("Switch to: {}", new_index);

            if let Some(callback) = self.on_index_changed.take() {
                (callback)(self, state, entity);
                self.on_index_changed = Some(callback);
            }
        }
        
    }


}

impl Widget for Stack {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
    }
}