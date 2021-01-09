

#[derive(Debug)]
struct HierarchyItem<T> {
    // Entity stored in the hierarchy
    entity: T,
    // Relative offset to new child of entity
    subtree_size: usize,
}

#[derive(Debug)]
struct Hierarchy<T> {
    entities: Vec<HierarchyItem<T>>,
}

impl<T> Hierarchy<T> 
where T: Copy + PartialEq + std::fmt::Debug
{
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
        }
    }
    
    pub fn insert(&mut self, parent: T, entity: T) {
    
        // Locate parent in hierarchy
        if let Some((parent_index, parent_item)) = self.entities.iter().enumerate().find(|(_,item)| item.entity == parent) {
            // Calculate the absolute positional index the child should be inserted
            let insert_index = parent_index + parent_item.subtree_size;
            
            // Insert entity into hierarchy
            self.entities.insert(insert_index, HierarchyItem{entity, subtree_size: 1});
            
            
            // Loop through the items from start to parent and increment the subtree_size if new entity is in the subtree
            for (index, mut item) in (&mut self.entities[0..parent_index+1]).iter_mut().enumerate() {
                if index + item.subtree_size >= insert_index {
                    item.subtree_size += 1;
                }
            }
        } 
    }

    // Insert an entity into the hierarchy
    pub fn insert2(&mut self, parent: T, entity: T) {
    
        let mut insert_index = std::usize::MAX;
        let mut parent_found = false;
    
        // Iterate up the hierarchy
        for (index, item) in self.entities.iter_mut().enumerate().rev() {
            // Locate the parent and determine the inserttion index for the new child
            if item.entity == parent {
                insert_index = index + item.subtree_size;
                parent_found = true;
            }
            
            // Incremenent the subtree size of the ancestors of the child, starting from the parent
            if parent_found && index + item.subtree_size >= insert_index {
                item.subtree_size += 1;
            }
        }
        
        // Insert the child if the parent exists
        if insert_index < std::usize::MAX {
            self.entities.insert(insert_index, HierarchyItem{entity, subtree_size: 1});
        } else {
            // Handle error here
            println!("No Parent Found");
        }
    }

    // Remove an entity from the hierarchy
    pub fn remove(&mut self, entity: T) {
        // Check if the entity is actually in the hierarchy

        // Locate the entity in the hierarchy
    
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item=T> + 'a {
        return self.entities.iter().map(|item| item.entity)
    }
}

pub struct ChildIterator<'a> {
    hierarchy: &'a Hierarchy,
    
}