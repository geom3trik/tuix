use std::cmp::{Eq, PartialEq};
use std::collections::VecDeque;
use std::hash::Hash;

// An entity is an id used to reference data in external storages.
// Rather than having widgets own their data, all state is stored in a single database and
// is stored and loaded using the entities.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Entity(u32);

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
        Entity(std::u32::MAX)
    }

    pub fn default() -> Entity {
        Entity(std::u32::MAX)
    }

    pub fn new(index: u32) -> Entity {
        Entity(index)
    }

    pub fn is_null(&self) -> bool {
        if self.0 == std::u32::MAX {
            true
        } else {
            false
        }
    }

    // pub fn index(&self) -> usize {
    //     return self.0 as usize;
    // }

    pub fn index(&self) -> Option<usize> {
        if self.0 < std::u32::MAX - 1 {
            Some(self.0 as usize)
        } else {
            None
        }
    }

    pub(crate) fn index_unchecked(&self) -> usize {
        self.0 as usize
    }

    pub fn root() -> Entity {
        Entity(0)
    }
}

impl std::ops::Not for Entity {
    type Output = bool;
    fn not(self) -> Self::Output {
        self == Entity::null()
    }
}

// impl ToString for Entity {
//     fn to_string(&self) -> String {
//         self.id.to_string()
//     }
// }

#[derive(Clone)]
pub(crate) struct EntityManager {

    count: usize,
}

impl EntityManager {
    pub(crate) fn new() -> EntityManager {
        EntityManager {
            count: 0,
        }
    }

    pub(crate) fn create_entity(&mut self) -> Option<Entity> {
        let index = self.count as u32;
        self.count += 1;
        
        return Some(Entity::new(index));
    }

    // Destroy an entity.
    // pub(crate) fn destroy_entity(&mut self, entity: Entity) {
    //     if let Some(index) = entity.index() {
    //         //self.count -= 1;
    //         self.free_indices.push_back(index as u32);
    //     }
    // }
}
