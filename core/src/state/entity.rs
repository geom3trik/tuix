use std::cmp::{Eq, PartialEq};
use std::collections::VecDeque;
use std::hash::{Hash, Hasher};

// An entity is an id used to reference data in external storages.
// Rather than having widgets own their data, all state is stored in a single database and
// is stored and loaded using the entities.

#[derive(Clone, Copy, Debug)]
pub struct Entity {
    id: u32,
}

impl Default for Entity {
    fn default() -> Self {
        Entity::null()
    }
}

impl std::fmt::Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.index())
    }
}

impl Entity {
    pub fn null() -> Entity {
        Entity { id: std::u32::MAX }
    }

    pub fn new(index: u16, generation: u16) -> Entity {
        Entity {
            id: (index as u32) | (generation as u32) << 16,
        }
    }

    pub fn is_null(&self) -> bool {
        if self.id == std::u32::MAX {
            true
        } else {
            false
        }
    }

    pub fn index(&self) -> usize {
        const INDEX_MASK: u32 = std::u16::MAX as u32;
        return (self.id & INDEX_MASK) as usize;
    }

    pub fn generation(&self) -> usize {
        const GEN_MASK: u32 = (std::u16::MAX as u32) << 16;
        return ((self.id & GEN_MASK) >> 16) as usize;
    }
}

impl PartialEq for Entity {
    fn eq(&self, other: &Entity) -> bool {
        self.id == other.id
    }
}

impl Eq for Entity {}

impl Hash for Entity {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

// impl ToString for Entity {
//     fn to_string(&self) -> String {
//         self.id.to_string()
//     }
// }

pub struct EntityManager {
    generations: Vec<u16>,
    free_indices: VecDeque<u16>,
}

impl EntityManager {
    pub fn new() -> EntityManager {
        EntityManager {
            generations: Vec::new(),
            free_indices: VecDeque::new(),
        }
    }

    // Create a new entity. Will reuse a previously destroyed entity id if one is available.
    pub fn create_entity(&mut self) -> Option<Entity> {
        let idx;
        if self.free_indices.len() > 1024 {
            idx = self.free_indices.pop_front();
        } else {
            self.generations.push(0);
            idx = Some((self.generations.len() - 1) as u16);
            //make sure that idx does not overflow
        }

        if let Some(index) = idx {
            return Some(Entity::new(index, self.generations[index as usize]));
        }

        return None;
    }

    // Destroy an entity.
    pub fn destroy_entity(&mut self, entity: Entity) {
        let idx = entity.index();
        self.generations[idx as usize] += 1;
        self.free_indices.push_back(idx as u16);
    }

    pub fn is_alive(&self, entity: Entity) -> bool {
        return self.generations[entity.index()] as usize == entity.generation();
    }
}
