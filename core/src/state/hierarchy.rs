use crate::entity::Entity;

pub enum HierarchyErrorKind {}

pub struct HierarchyError {}

#[derive(Clone)]
pub struct Hierarchy {
    pub entities: Vec<Entity>,
    pub parent: Vec<Option<Entity>>,
    pub first_child: Vec<Option<Entity>>,
    pub next_sibling: Vec<Option<Entity>>,
    pub prev_sibling: Vec<Option<Entity>>,
}

impl Hierarchy {
    pub fn new() -> Hierarchy {
        Hierarchy {
            entities: Vec::new(),
            parent: Vec::new(),
            first_child: Vec::new(),
            next_sibling: Vec::new(),
            prev_sibling: Vec::new(),
        }
    }

    pub fn root(&self) -> Option<Entity> {
        return *self.parent.first().unwrap();
    }

    /// Returns the last child of an entity
    pub fn get_last_child(&self, entity: Entity) -> Option<Entity> {
        //check if entity exists
        if let Some(index) = entity.index() {
            let mut f = self.first_child[index];
            let mut r = None;
            while f != None {
                r = f;
                f = self.next_sibling[f.unwrap().index().unwrap()];
            }

            return r;
        } else {
            None
        }
    }

    /// Returns the nth child of an entity
    pub fn get_child(&self, entity: Entity, index: usize) -> Option<Entity> {
        if let Some(index) = entity.index() {
            let mut f = self.first_child[index];
            let mut i = 0;
            while f != None {
                if i == index {
                    break;
                }
                f = self.next_sibling[f.unwrap().index().unwrap()];
                i += 1;
            }

            return f;
        } else {
            None
        }
    }

    /// Returns the number of children of an entity
    pub fn get_num_children(&self, entity: Entity) -> Option<u32> {
        if let Some(index) = entity.index() {
            let mut f = self.first_child[index];
            let mut r = 0;
            while f != None {
                r += 1;
                f = self.next_sibling[f.unwrap().index().unwrap()];
            }

            Some(r)
        } else {
            None
        }
    }

    /// Returns the parent of an entity
    pub fn get_parent(&self, entity: Entity) -> Option<Entity> {
        if let Some(index) = entity.index() {
            if index >= self.parent.len() {
                None
            } else {
                self.parent[index]
            }
        } else {
            None
        }
    }

    /// Returns the first child of an entity
    pub fn get_first_child(&self, entity: Entity) -> Option<Entity> {
        if let Some(index) = entity.index() {
            self.first_child[index]
        } else {
            None
        }
    }

    /// Returns the next sibling of an entity
    pub fn get_next_sibling(&self, entity: Entity) -> Option<Entity> {
        if let Some(index) = entity.index() {
            self.next_sibling[index]
        } else {
            None
        }
    }

    /// Returns the previous sibling of an entity
    pub fn get_prev_sibling(&self, entity: Entity) -> Option<Entity> {
        if let Some(index) = entity.index() {
            self.prev_sibling[index]
        } else {
            None
        }
    }

    /// Returns true if the entity is the first child of its parent
    pub fn is_first_child(&self, entity: Entity) -> bool {
        if let Some(parent) = self.get_parent(entity) {
            if let Some(first_child) = self.get_first_child(parent) {
                if first_child == entity {
                    return true;
                } else {
                    return false;
                }
            }
        }

        false
    }

    // Checks if entity1 is the sibling of entity2
    pub fn is_sibling(&self, entity1: Entity, entity2: Entity) -> bool {
        if let Some(parent1) = self.get_parent(entity1) {
            if let Some(parent2) = self.get_parent(entity2) {
                return parent1 == parent2;
            }
        }

        false
    }

    // Not decided yet how this should work
    pub fn remove_children(&mut self, _entity: Entity) {}

    /// Returns true if the entity has children
    pub fn has_children(&self, entity: Entity) -> bool {
        if let Some(index) = entity.index() {
            self.first_child[index].is_some()
        } else {
            false
        }
    }

    pub fn remove(&mut self, entity: Entity) {
        // Recursively remove all of the nodes below this one
        // if let Some(child) = self.get_first_child(entity) {
        //     self.remove(child);
        // }

        if let Some(index) = entity.index() {
            if let Some((index, _)) = self.entities.iter().enumerate().find(|(_, &e)| e == entity) {
                self.entities.remove(index);
            }

            if let Some(parent) = self.get_parent(entity) {
                if self.is_first_child(entity) {
                    self.first_child[parent.index_unchecked()] = self.get_next_sibling(entity);
                }
            }

            if let Some(prev_sibling) = self.get_prev_sibling(entity) {
                self.next_sibling[prev_sibling.index_unchecked()] = self.get_next_sibling(entity);
            }

            if let Some(next_sibling) = self.get_next_sibling(entity) {
                self.prev_sibling[next_sibling.index_unchecked()] = self.get_prev_sibling(entity);
            }

            self.next_sibling[index] = None;
            self.prev_sibling[index] = None;
            self.parent[index] = None;
        }
    }

    // Makes the entity the first child of its parent
    pub fn set_first_child(&mut self, entity: Entity) -> Result<(), HierarchyError> {
        if let Some(index) = entity.index() {
            // Check is sibline exists in the hierarchy
            if index >= self.parent.len() {
                return Err(HierarchyError {});
            }

            // Check if the parent is in the hierarchy
            if let Some(parent) = self.get_parent(entity) {
                if parent.index_unchecked() >= self.parent.len() {
                    return Err(HierarchyError {});
                }
            }

            let parent = self.get_parent(entity).unwrap();

            let previous_first_child = self.first_child[parent.index_unchecked()];

            let entity_prev_sibling = self.get_prev_sibling(entity);
            let entity_next_sibling = self.get_next_sibling(entity);

            // Remove the entity from the children
            if let Some(eps) = entity_prev_sibling {
                self.next_sibling[eps.index_unchecked()] = entity_next_sibling; //C
            }

            if let Some(ens) = entity_next_sibling {
                self.prev_sibling[ens.index_unchecked()] = entity_prev_sibling; //F
            }

            if let Some(pfc) = previous_first_child {
                self.prev_sibling[pfc.index_unchecked()] = Some(entity);
            }

            self.next_sibling[index] = previous_first_child;

            self.first_child[parent.index_unchecked()] = Some(entity);

            Ok(())
        } else {
            Err(HierarchyError {})
        }
    }

    pub fn set_next_sibling(
        &mut self,
        entity: Entity,
        sibling: Entity,
    ) -> Result<(), HierarchyError> {
        if self.next_sibling[entity.index_unchecked()] == Some(sibling) {
            return Err(HierarchyError {});
        }

        // Check is sibline exists in the hierarchy
        if sibling.index_unchecked() >= self.parent.len() {
            return Err(HierarchyError {});
        }

        // Check if sibling has the same parent
        if let Some(parent) = self.get_parent(entity) {
            if let Some(sibling_parent) = self.get_parent(entity) {
                if parent != sibling_parent {
                    return Err(HierarchyError {});
                }
            }
        } else {
            return Err(HierarchyError {});
        }

        // Safe to unwrap because we already checked if it has a parent
        let parent = self.get_parent(entity).unwrap();

        // Temporarily store the prev_sibling of the desired sibling
        let sibling_prev_sibling = self.get_prev_sibling(sibling);
        let sibling_next_sibling = self.get_next_sibling(sibling);

        // println!("sibling_prev_sibling: {:?}", sibling_prev_sibling);
        // println!("entity_prev_sibling: {:?}", entity_prev_sibling);
        // println!("entity_next_sibling: {:?}", entity_next_sibling);
        // println!("entity: {:?}", entity);
        // println!("sibling: {:?}", sibling);

        // Remove sibling
        if let Some(sps) = sibling_prev_sibling {
            self.next_sibling[sps.index_unchecked()] = sibling_next_sibling; // C
        } else {
            self.first_child[parent.index_unchecked()] = sibling_next_sibling;
        }

        if let Some(sns) = sibling_next_sibling {
            self.prev_sibling[sns.index_unchecked()] = sibling_prev_sibling; // F
        }

        // Temporarily store the next_sibling of the entity
        let entity_next_sibling = self.get_next_sibling(entity);

        if let Some(ens) = entity_next_sibling {
            self.prev_sibling[ens.index_unchecked()] = Some(sibling); //B
        }

        self.next_sibling[sibling.index_unchecked()] = entity_next_sibling; //E
        self.prev_sibling[sibling.index_unchecked()] = Some(entity); // D
        self.next_sibling[entity.index_unchecked()] = Some(sibling); // A

        Ok(())
    }

    pub fn set_prev_sibling(
        &mut self,
        entity: Entity,
        sibling: Entity,
    ) -> Result<(), HierarchyError> {
        if self.prev_sibling[entity.index_unchecked()] == Some(sibling) {
            return Err(HierarchyError {});
        }

        // Check is sibline exists in the hierarchy
        if sibling.index_unchecked() >= self.parent.len() {
            return Err(HierarchyError {});
        }

        // Check if sibling has the same parent
        if let Some(parent) = self.get_parent(entity) {
            if let Some(sibling_parent) = self.get_parent(entity) {
                if parent != sibling_parent {
                    return Err(HierarchyError {});
                }
            }
        } else {
            return Err(HierarchyError {});
        }

        // Safe to unwrap because we already checked if it has a parent
        let parent = self.get_parent(entity).unwrap();

        // Temporarily store the prev_sibling of the desired sibling
        let sibling_prev_sibling = self.get_prev_sibling(sibling);
        let sibling_next_sibling = self.get_next_sibling(sibling);

        // Remove sibling
        if let Some(sps) = sibling_prev_sibling {
            self.next_sibling[sps.index_unchecked()] = sibling_next_sibling; // C
        } else {
            self.first_child[parent.index_unchecked()] = sibling_next_sibling;
        }

        if let Some(sns) = sibling_next_sibling {
            self.prev_sibling[sns.index_unchecked()] = sibling_prev_sibling; // F
        }

        // Temporarily store the prev_sibling of the entity
        let entity_prev_sibling = self.get_prev_sibling(entity);

        if let Some(eps) = entity_prev_sibling {
            self.next_sibling[eps.index_unchecked()] = Some(sibling); // A
        } else {
            self.first_child[parent.index_unchecked()] = Some(sibling);
        }

        self.next_sibling[sibling.index_unchecked()] = Some(entity); //E

        self.prev_sibling[sibling.index_unchecked()] = entity_prev_sibling; // D

        self.prev_sibling[entity.index_unchecked()] = Some(sibling); // B

        Ok(())
    }

    pub fn set_parent(&mut self, entity: Entity, parent: Entity) {
        if let Some(old_parent) = self.get_parent(entity) {
            if self.is_first_child(entity) {
                self.first_child[old_parent.index_unchecked()] = self.get_next_sibling(entity);
            }
        }

        if let Some(prev_sibling) = self.get_prev_sibling(entity) {
            self.next_sibling[prev_sibling.index_unchecked()] = self.get_next_sibling(entity);
        }

        if let Some(next_sibling) = self.get_next_sibling(entity) {
            self.prev_sibling[next_sibling.index_unchecked()] = self.get_prev_sibling(entity);
        }

        if self.first_child[parent.index_unchecked()] == None {
            self.first_child[parent.index_unchecked()] = Some(entity);
        } else {
            let mut temp = self.first_child[parent.index_unchecked()];

            loop {
                if self.next_sibling[temp.unwrap().index_unchecked()] == None {
                    break;
                }

                temp = self.next_sibling[temp.unwrap().index_unchecked()];
            }

            self.next_sibling[temp.unwrap().index_unchecked()] = Some(entity);
            self.prev_sibling[entity.index_unchecked()] = temp;
        }

        self.parent[entity.index_unchecked()] = Some(parent);
    }

    pub fn add(&mut self, entity: Entity, parent: Option<Entity>) {
        if let Some(index) = entity.index() {
            self.entities.push(entity);

            if index >= self.parent.len() {
                self.parent.resize(index + 1, None);
                self.first_child.resize(index + 1, None);
                self.next_sibling.resize(index + 1, None);
                self.prev_sibling.resize(index + 1, None);
            }

            self.parent[index] = parent;
            self.first_child[index] = None;
            self.next_sibling[index] = None;
            self.prev_sibling[index] = None;

            if let Some(p) = parent {
                // If the parent has no first child then this entity is the first child
                if self.first_child[p.index_unchecked()] == None {
                    self.first_child[p.index_unchecked()] = Some(entity);
                } else {
                    let mut temp = self.first_child[p.index_unchecked()];

                    loop {
                        if self.next_sibling[temp.unwrap().index_unchecked()] == None {
                            break;
                        }

                        temp = self.next_sibling[temp.unwrap().index_unchecked()];
                    }

                    self.next_sibling[temp.unwrap().index_unchecked()] = Some(entity);
                    self.prev_sibling[index] = temp;
                }
            }
        }
    }

    pub fn add_with_sibling(&mut self, entity: Entity, sibling: Entity) {
        if let Some(index) = entity.index() {
            if let Some(sibling) = self.entities.iter_mut().find(|e| **e == sibling) {
                let sibling = sibling.to_owned();
                self.entities.push(entity);

                if index >= self.parent.len() {
                    self.parent.resize(index + 1, None);
                    self.first_child.resize(index + 1, None);
                    self.next_sibling.resize(index + 1, None);
                    self.prev_sibling.resize(index + 1, None);
                }

                if let Some(next_sib) = self.get_next_sibling(sibling) {
                    self.prev_sibling[next_sib.index_unchecked()] = Some(entity);
                }

                self.parent[index] = self.get_parent(sibling);
                self.first_child[index] = None;
                self.next_sibling[index] = self.get_next_sibling(sibling);
                self.prev_sibling[index] = Some(sibling);

                self.next_sibling[sibling.index_unchecked()] = Some(entity);
            }
        }
    }
}

impl<'a> IntoIterator for &'a Hierarchy {
    type Item = Entity;
    type IntoIter = HierarchyIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        HierarchyIterator {
            hierarchy: self,
            current_node: Some(*self.entities.first().unwrap()),
            //current_back: Some(*self.entities.last().unwrap()),
        }
    }
}

pub struct BranchIterator<'a> {
    hierarchy: &'a Hierarchy,
    start_node: Entity,
    current_node: Option<Entity>,
}

impl<'a> Iterator for BranchIterator<'a> {
    type Item = Entity;
    fn next(&mut self) -> Option<Entity> {
        let r = self.current_node;

        if let Some(current) = self.current_node {
            if let Some(child) = self.hierarchy.first_child[current.index_unchecked()] {
                self.current_node = Some(child);
            } else {
                let mut temp = Some(current);
                while temp.is_some() {
                    if let Some(sibling) =
                        self.hierarchy.next_sibling[temp.unwrap().index_unchecked()]
                    {
                        self.current_node = Some(sibling);
                        return r;
                    } else {
                        temp = self.hierarchy.parent[temp.unwrap().index_unchecked()];
                        if Some(self.start_node) == temp {
                            self.current_node = None;
                            temp = None;
                        }
                    }
                }

                self.current_node = None;
            }
        }

        return r;
    }
}

// Iterator for iterating through the hierarchy from top to bottom
pub struct HierarchyIterator<'a> {
    hierarchy: &'a Hierarchy,
    current_node: Option<Entity>,
    //current_back: Option<Entity>,
}

impl<'a> Iterator for HierarchyIterator<'a> {
    type Item = Entity;
    fn next(&mut self) -> Option<Entity> {
        let r = self.current_node;

        if let Some(current) = self.current_node {
            if let Some(child) = self.hierarchy.first_child[current.index_unchecked()] {
                self.current_node = Some(child);
            } else {
                let mut temp = Some(current);
                while temp.is_some() {
                    if let Some(sibling) =
                        self.hierarchy.next_sibling[temp.unwrap().index_unchecked()]
                    {
                        self.current_node = Some(sibling);
                        return r;
                    } else {
                        temp = self.hierarchy.parent[temp.unwrap().index_unchecked()];
                    }
                }

                self.current_node = None;
            }
        }

        return r;
    }
}

// TODO
// impl<'a> DoubleEndedIterator for HierarchyIterator<'a> {
//     fn next_back(&mut self) -> Option<Self::Item> {
//         let r = self.current_back;
//         if let Some(current) = self.current_node {
//             if let Some(prev_sibling) = self.hierarchy.prev_sibling[current.index()] {
//                 self.current_node = Some(prev_sibling)
//             }
//         }

//         return r;
//     }
// }

// Iterator for iterating through the parents of widgets.
pub struct ParentIterator<'a> {
    hierarchy: &'a Hierarchy,
    current: Option<Entity>,
}

impl<'a> Iterator for ParentIterator<'a> {
    type Item = Entity;
    fn next(&mut self) -> Option<Entity> {
        if let Some(entity) = self.current {
            self.current = self.hierarchy.parent[entity.index_unchecked()];
            return Some(entity);
        }

        None
    }
}

pub trait IntoParentIterator<'a> {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;
    fn parent_iter(self, hierarchy: &'a Hierarchy) -> Self::IntoIter;
}

impl<'a> IntoParentIterator<'a> for &'a Entity {
    type Item = Entity;
    type IntoIter = ParentIterator<'a>;

    fn parent_iter(self, h: &'a Hierarchy) -> Self::IntoIter {
        ParentIterator {
            hierarchy: h,
            current: Some(*self),
        }
    }
}

// Iterator for iterating through the children of widgets.
pub struct ChildIterator<'a> {
    hierarchy: &'a Hierarchy,
    current_forward: Option<Entity>,
    current_backward: Option<Entity>,
}

impl<'a> Iterator for ChildIterator<'a> {
    type Item = Entity;
    fn next(&mut self) -> Option<Entity> {
        if let Some(entity) = self.current_forward {
            self.current_forward = self.hierarchy.next_sibling[entity.index_unchecked()];
            return Some(entity);
        }

        None
    }
}

impl<'a> DoubleEndedIterator for ChildIterator<'a> {
    fn next_back(&mut self) -> Option<Entity> {
        if let Some(entity) = self.current_backward {
            self.current_backward = self.hierarchy.prev_sibling[entity.index_unchecked()];
            return Some(entity);
        }

        None
    }
}

pub trait IntoChildIterator<'a> {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;
    fn child_iter(self, hierarchy: &'a Hierarchy) -> Self::IntoIter;
}

impl<'a> IntoChildIterator<'a> for &'a Entity {
    type Item = Entity;
    type IntoIter = ChildIterator<'a>;

    fn child_iter(self, h: &'a Hierarchy) -> Self::IntoIter {
        ChildIterator {
            hierarchy: h,
            current_forward: h.first_child[self.index_unchecked()],
            current_backward: h.get_last_child(*self),
        }
    }
}

pub trait IntoHierarchyIterator<'a> {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;
    fn into_iter(self, hierarchy: &'a Hierarchy) -> Self::IntoIter;
}

impl<'a> IntoHierarchyIterator<'a> for &'a Entity {
    type Item = Entity;
    type IntoIter = HierarchyIterator<'a>;

    fn into_iter(self, h: &'a Hierarchy) -> Self::IntoIter {
        HierarchyIterator {
            hierarchy: h,
            current_node: Some(*self),
        }
    }
}

pub trait IntoBranchIterator<'a> {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;
    fn branch_iter(self, hierarchy: &'a Hierarchy) -> Self::IntoIter;
}

impl<'a> IntoBranchIterator<'a> for &'a Entity {
    type Item = Entity;
    type IntoIter = BranchIterator<'a>;

    fn branch_iter(self, h: &'a Hierarchy) -> Self::IntoIter {
        BranchIterator {
            hierarchy: h,
            start_node: *self,
            current_node: Some(*self),
        }
    }
}

//Think of better name for this
pub trait HierarchyTree<'a> {
    fn parent(&self, hierarchy: &'a Hierarchy) -> Option<Entity>;
    fn is_sibling(&self, hierarchy: &'a Hierarchy, entity: Entity) -> bool;
    fn is_child_of(&self, hierarchy: &'a Hierarchy, entity: Entity) -> bool;
    fn is_descendant_of(&self, hierarchy: &'a Hierarchy, entity: Entity) -> bool;
}

impl<'a> HierarchyTree<'a> for Entity {
    fn parent(&self, hierarchy: &'a Hierarchy) -> Option<Entity> {
        hierarchy.get_parent(*self)
    }

    fn is_sibling(&self, hierarchy: &'a Hierarchy, entity: Entity) -> bool {
        hierarchy.is_sibling(*self, entity)
    }

    fn is_child_of(&self, hierarchy: &'a Hierarchy, entity: Entity) -> bool {
        if *self == Entity::null() {
            return false;
        }

        if let Some(parent) = hierarchy.get_parent(*self) {
            if parent == entity {
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }

    fn is_descendant_of(&self, hierarchy: &'a Hierarchy, entity: Entity) -> bool {
        if *self == Entity::null() {
            return false;
        }

        for parent in self.parent_iter(hierarchy) {
            if parent == entity {
                return true;
            }
        }

        false
    }

    // TODO
    //fn is_descendant_of(&self, hierarchy: &'a Hierarchy, entity: Entity) -> bool {
    //    return false;

    // if let Some(parent) = hierarchy.get_parent(*self) {
    //     if parent == entity {
    //         return true;
    //     } else {
    //         return false;
    //     }
    // } else {
    //     return false;
    // }
    //}
}
