use crate::{Entity, State, Update};

use std::any::{Any, TypeId};
pub trait Node: Any {
    fn mutate(&mut self, update: &mut Update) -> bool {
        
        // let mutator = update.mutator.borrow_mut();

        // let new_value = mutator(self as &mut dyn Node);

        false
    }

    fn on_update(&self, state: &mut State, entity: Entity, data: &Box<dyn Node>) {}

    fn build(mut self, state: &mut State, parent: Entity) -> Entity 
    where Self: std::marker::Sized + 'static
    {
        let entity = state.entity_manager.create_entity().unwrap();

        state.data_nodes.insert(entity, Box::new(self));

        entity
    }
}

impl dyn Node {
    // Check if a message is a certain type
    pub fn is<T: Node + 'static>(&self) -> bool {
        // Get TypeId of the type this function is instantiated with
        let t = TypeId::of::<T>();

        // Get TypeId of the type in the trait object
        let concrete = self.type_id();

        // Compare both TypeIds on equality
        t == concrete
    }

    // Casts a message to the specified type if the message is of that type
    pub fn downcast<T>(&mut self) -> Option<&mut T>
    where
        T: Node + 'static,
    {
        if self.is::<T>() {
            unsafe { Some(&mut *(self as *mut dyn Node as *mut T)) }
        } else {
            None
        }
    }

    pub fn downcast_ref<T>(&self) -> Option<&T>
    where
        T: Node + 'static,
    {
        if self.is::<T>() {
            unsafe { Some(&*(self as *const dyn Node as *const T)) }
        } else {
            None
        }
    }
}

// Can't do this apparently, that's annoying
// impl<T: Node> EventHandler for T
// where
//     T: Node + 'static,
// {
//     fn update(&mut self, data: &Self) -> bool {
//         <T as Node>::update(self, data)
//     }
// }

impl Node for bool {}