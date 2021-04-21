use std::cmp::{Eq, PartialEq};
use std::collections::VecDeque;
use std::hash::Hash;

// An entity is an id used to reference data in external storages.
// Rather than having widgets own their data, all state is stored in a single database and
// is stored and loaded using the entities.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Entity {
    index: u32,
    generation: u32,
}

impl Default for Entity {
    fn default() -> Self {
        Entity::default()
    }
}

impl std::fmt::Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.index_unchecked())
    }
}

impl Entity {
    pub fn null() -> Entity {
        Entity {
            index: std::u32::MAX,
            generation: std::u32::MAX,
        }
    }

    pub fn default() -> Entity {
        Entity::null()
    }

    pub fn new(index: u32, generation: u32) -> Entity {
        Entity { index, generation }
    }

    pub fn is_null(&self) -> bool {
        if self.index == std::u32::MAX {
            true
        } else {
            false
        }
    }

    // pub fn index(&self) -> usize {
    //     return self.0 as usize;
    // }

    pub fn index(&self) -> Option<usize> {
        if self.index < std::u32::MAX {
            Some(self.index as usize)
        } else {
            None
        }
    }

    pub(crate) fn index_unchecked(&self) -> usize {
        self.index as usize
    }

    pub fn root() -> Entity {
        Entity {
            index: 0,
            generation: 0,
        }
    }
}

impl std::ops::Not for Entity {
    type Output = bool;
    fn not(self) -> Self::Output {
        self == Entity::null()
    }
}


#[derive(Clone)]
pub(crate) struct EntityManager {
    count: u32,
    free_list: VecDeque<Entity>,
}

impl EntityManager {
    pub fn new() -> EntityManager {
        EntityManager {
            count: 0,
            free_list: VecDeque::with_capacity(1024),
        }
    }

    pub(crate) fn create_entity(&mut self) -> Option<Entity> {
        if self.free_list.len() > 1024 {
            if let Some(mut entity) = self.free_list.pop_front() {
                entity.generation += 1;
                Some(entity)
            } else {
                None
            }
        } else {
            self.count += 1;
            Some(Entity::new(self.count, 0))
        }
    }

    pub(crate) fn destroy_entity(&mut self, entity: Entity) {
        println!("Destroy Entity");
        self.free_list.push_back(entity);
    }
}

pub trait AsEntity {
    fn entity(&self) -> Entity;
}

impl AsEntity for Entity {
    fn entity(&self) -> Entity {
        *self
    }
}
