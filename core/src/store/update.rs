


use crate::{Entity, Node};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub struct Update {
    pub origin: Entity,
    pub target: Entity,
    pub mutator: Rc<RefCell<dyn FnMut(&mut dyn Node) -> bool>>,
}

impl Update {
    pub fn new<T: Node>(origin: Entity, mut mutator: impl FnMut(&mut T) + 'static) -> Self {
        Self {
            origin,
            target: Entity::null(),
            mutator: Rc::new(RefCell::new(move |node: &mut dyn Node| {
                if let Some(data) = node.downcast::<T>() {
                    mutator(data);
                    true
                } else {
                    false
                }
            }))
        }   
    }

    pub fn target(mut self, target: Entity) -> Self {
        self.target = target;

        self
    }
}