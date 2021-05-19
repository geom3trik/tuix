


use crate::{Entity, Node};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub struct Update {
    pub origin: Entity,
    pub mutator: Rc<RefCell<dyn FnMut(&mut dyn Node)>>,
}

impl Update {
    pub fn new<T: Node>(origin: Entity, mut mutator: impl FnMut(&mut T) + 'static) -> Self {
        Self {
            origin,
            mutator: Rc::new(RefCell::new(move |node: &mut dyn Node| mutator(node.downcast::<T>().unwrap())))
        }   
    }
}