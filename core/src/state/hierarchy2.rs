use crate::{Entity};



// #[derive(Debug)]
// pub struct Meta {
//     index: u32,
//     //parent: u32,
// }

#[derive(Debug)]
pub struct Item {
    pub entity: Entity,
    pub parent: Entity,
}

pub struct Hierarchy2 {
    pub meta: Vec<u32>,
    pub entities: Vec<Item>,
}

impl Hierarchy2 {
    pub fn new() -> Self {
        
        let mut meta = Vec::new();
        let mut entities = Vec::new();
        
        meta.push(0);
        entities.push(Item{entity: Entity::new(0,0), parent: Entity::null()});

        Self {
            meta,
            entities,
        }
    }

    pub fn insert(&mut self, entity: Entity, parent: Entity) {
        if let Some(entity_index) = entity.index() {
            if let Some(parent_index) = parent.index() {
                if let Some(parent_meta) = self.meta.get(parent_index) {
                    //println!("parent_meta: {}", parent_meta);
                    if *parent_meta as usize == self.entities.len() - 1 {
                        //println!("1");
                        if entity_index >= self.meta.len() {
                            self.meta.resize_with(entity_index + 1,|| std::u32::MAX);
                        }

                        self.meta[entity_index] = self.entities.len() as u32;
                        self.entities.push(Item{entity, parent});

                    } else {
                        //println!("2");
                        let mut insert_index = *parent_meta as usize + 1;
                        println!("insert_index: {}", insert_index);

                        if let Some(parent_item) = self.entities.get(*parent_meta as usize) {
                            //println!("parent_item_index: {}", parent_item.parent);
                            for (index, item) in (&self.entities[*parent_meta as usize+1..self.entities.len()]).iter().enumerate() {
                                println!("item: {} {}", item.parent, parent_item.parent);
                                if item.parent == parent_item.parent {
                                    //println!("insert: {}", *parent_meta as usize + index);
                                    insert_index = *parent_meta as usize + index + 2;
                                }
                            }
                        }

                        println!("insert_index_new: {}", insert_index);
                        //println!("1: {} {} {}", insert_index, parent_meta, self.entities.len());
                        // for (index, item) in (&self.entities[*parent_meta as usize+1..self.entities.len()]).iter().enumerate() {
                        //     //println!("Item: {:?}", item);
                        //     println!("i: {} {} {}", index, item.parent, parent);
                        //     if item.entity == parent {
                        //         insert_index = *parent_meta as usize + index + 2;
                        //         break;
                        //     }
                        // }
                        //println!("2: {}", insert_index);

                        if entity_index >= self.meta.len() {
                            self.meta.resize_with(entity_index + 1,|| std::u32::MAX);
                        }

                        self.meta[entity_index] = insert_index as u32;
                        self.entities.insert(insert_index, Item{entity, parent});

                    }
                }            
            }            
        }


    }
}






// #[derive(Debug)]
// pub struct MetaData {
//     index: u32,
//     subtree_size: u32,
//     parent: u32,
//     next_sibling: u32,

// }

// impl Default for MetaData {
//     fn default() -> Self {
//         MetaData {
//             index: std::u32::MAX,
//             subtree_size: std::u32::MAX,
//             parent: std::u32::MAX,
//             next_sibling: std::u32::MAX,
//         }
//     }
// }


// pub struct Hierarchy2 {
//     pub indices: Vec<MetaData>,
//     pub entities: Vec<Entity>,
// }

// impl Hierarchy2 {
//     pub fn new() -> Self {

//         let mut indices = Vec::new();
//         let mut entities = Vec::new();

//         indices.push(MetaData { index: 0, subtree_size: 1, parent: std::u32::MAX, next_sibling: std::u32::MAX});
//         entities.push(Entity::root());

//         Self {
//             indices,
//             entities,

//         }
//     }

//     pub fn insert(&mut self, entity: Entity, parent: Entity) {

//         if entity.index_unchecked() == parent.index_unchecked() {
//             return;
//         }

//         if let Some(entity_index) = entity.index() {
//             if let Some(parent_index) = parent.index() {
//                 if let Some(parent_meta) = self.indices.get(parent_index) {
//                     let insert_index = parent_meta.index + parent_meta.subtree_size;

//                     if entity_index >= self.indices.len() {
//                         self.indices.resize_with(entity_index + 1, || MetaData::default());
//                     }

//                     self.indices[entity_index] = MetaData {
//                         index: insert_index as u32,
//                         subtree_size: 1,
//                         parent: parent_index as u32,
//                         next_sibling: std::u32::MAX,
//                     };

//                     self.entities.insert(insert_index as usize, entity);

//                     for entity in (&self.entities[0..parent_index+1]).iter() {
//                         if let Some(entity_index) = entity.index() {
//                             if let Some(entity_meta) = self.indices.get_mut(entity_index) {
//                                 if entity_meta.index + entity_meta.subtree_size >= insert_index {
//                                     entity_meta.subtree_size += 1;
//                                 }
//                             }
//                         }
//                     }

//                     for entity in (&self.entities[insert_index as usize+1..self.entities.len()]).iter() {
//                         if let Some(entity_index) = entity.index() {
//                             if let Some(entity_meta) = self.indices.get_mut(entity_index) {
//                                 entity_meta.index += 1;
//                             }
//                         }
//                     }
//                     //println!("{:?}", self.indices);
//                     //println!("{:?}", self.entities);
//                 }
//             }            
//         }
//     }

//     pub fn remove(&mut self, entity: Entity) {
//         if let Some(entity_index) = entity.index() {
//             if let Some(entity_meta) = self.indices.get(entity_index) {
//                 self.entities.remove(entity_meta.index as usize);
//                 self.indices[entity_index] = MetaData::default();
//             }
//         }
//     }

//     pub fn get_parent(&self, entity: Entity) -> Entity {
//         if let Some(entity_index) = entity.index() {
//             if let Some(entity_meta) = self.indices.get(entity_index) {
//                 return entity_meta.parent;
//             }
//         }

//         return Entity::null();
//     }

//     pub fn get_first_child(&self, entity: Entity) -> Entity {
//         if let Some(entity_index) = entity.index() {
//             if let Some(entity_meta) = self.indices.get(entity_index) {
//                 if let Some(first_child) = self.entities.get(entity_meta.index as usize + 1) {
//                     return *first_child;
//                 }
                
//             }
//         }

//         return Entity::null();
//     }

//     pub fn get_next_sibling(&self, entity: Entity) -> Entity {

//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_entities() {

    }
}
