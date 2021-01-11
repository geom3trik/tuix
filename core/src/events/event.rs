use crate::entity::Entity;
use crate::state::*;

use std::any::{Any, TypeId};
use std::fmt::Debug;

// Determines how the event propagates through the hierarchy
#[derive(Debug, Clone)]
pub enum Propagation {
    Down,
    Up,
    DownUp,
    Fall,
    Direct,
}

//TODO - Determines if the event can be cancelled
#[derive(Debug, Clone)]
pub enum Cancellable {
    Yes,
    No,
}

// A message is a wrapper around an Any but with the added ability to Clone the message
pub trait Message: Any + MessageClone + Debug {
    // An &Any can be cast to a reference to a concrete type.
    fn as_any(&self) -> &dyn Any;

    // Perform the test.
    fn equals_a(&self, _: &dyn Message) -> bool;
}

// An Any is not normally clonable. This is a way around that.
pub trait MessageClone {
    fn clone_message(&self) -> Box<Message>;
}

// Implements MessageClone for any type that Implements Message and Clone
impl<T> MessageClone for T
where
    T: 'static + Message + Clone,
{
    fn clone_message(&self) -> Box<Message> {
        Box::new(self.clone())
    }
}

// an implementation of clone for boxed messages
impl Clone for Box<Message> {
    fn clone(&self) -> Box<Message> {
        self.clone_message()
    }
}

//impl<T> Message for T where T: 'static + Any + Clone {}

impl dyn Message {
    pub fn is<T: Message>(&self) -> bool {
        // Get TypeId of the type this function is instantiated with
        let t = TypeId::of::<T>();

        // Get TypeId of the type in the trait object
        let concrete = self.type_id();

        // Compare both TypeIds on equality
        t == concrete
    }

    pub fn downcast<T>(&mut self) -> Option<&mut T>
    where
        T: Message,
    {
        if self.is::<T>() {
            unsafe { Some(&mut *(self as *mut dyn Message as *mut T)) }
        } else {
            None
        }
    }
}

// Implements message for any static type that implements PartialEq, Debug and Clone
impl<S: 'static + PartialEq + std::fmt::Debug + Clone> Message for S {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn equals_a(&self, other: &dyn Message) -> bool {
        other
            .as_any()
            .downcast_ref::<S>()
            .map_or(false, |a| self == a)
    }
}

#[derive(Clone, Debug)]
pub struct Event {
    // The entity that produced the event. Entity::null() for OS events or unspecified.
    pub origin: Entity,
    // The entity the event should be sent to. Entity::null() to send to all entities.
    pub target: Entity,
    // How the event propagates through the tree.
    pub propagation: Propagation,
    // Whether the event can be cancelled (consumed?)
    pub cancellable: bool,
    // Whether the event is unique (only the latest copy can exist in a queue at a time)
    pub unique: bool,
    pub order: i32,
    // The event type
    pub message: Box<dyn Message>,
}

impl PartialEq for Event {
    fn eq(&self, other: &Event) -> bool {
        self.message.equals_a(&*other.message) && self.origin == other.origin
    }
}

impl Event {
    pub fn new<M>(message: M) -> Self
    where
        M: Message,
    {
        Event {
            origin: Entity::null(),
            target: Entity::new(0, 0),
            propagation: Propagation::DownUp,
            cancellable: true,
            unique: true,
            order: 0,
            message: Box::new(message),
        }
    }

    pub fn is_type<T>(&mut self) -> Option<&mut T>
    where
        T: Message,
    {
        self.message.downcast::<T>()
    }

    pub fn target(mut self, entity: Entity) -> Self {
        self.target = entity;
        self
    }

    pub fn origin(mut self, entity: Entity) -> Self {
        self.origin = entity;
        self
    }

    pub fn unique(mut self) -> Self {
        self.unique = true;
        self
    }

    pub fn propagate(mut self, propagation: Propagation) -> Self {
        self.propagation = propagation;

        self
    }

    pub fn get_propagate_up(&self) -> bool {
        match self.propagation {
            Propagation::Up => true,
            Propagation::DownUp => true,
            _ => false,
        }
    }

    pub fn get_propagate_down(&self) -> bool {
        match self.propagation {
            Propagation::Down => true,
            Propagation::DownUp => true,
            _ => false,
        }
    }

    pub fn get_propagate_fall(&self) -> bool {
        match self.propagation {
            Propagation::Fall => true,
            _ => false,
        }
    }
}
