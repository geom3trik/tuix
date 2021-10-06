use crate::{Tree, Entity, GenerationalId};

/// Iterator for iterating through the tree from top to bottom in depth first order
pub struct TreeIterator<'a> {
    pub tree: &'a Tree,
    pub current_node: Option<Entity>,
    //current_back: Option<Entity>,
}

impl<'a> TreeIterator<'a> {
    /// Skip to next branch
    pub fn next_branch(&mut self) -> Option<Entity> {
        let r = self.current_node;
        if let Some(current) = self.current_node {
            let mut temp = Some(current);
            while temp.is_some() {
                if let Some(sibling) = self.tree.next_sibling[temp.unwrap().index()]
                {
                    self.current_node = Some(sibling);
                    return r;
                } else {
                    temp = self.tree.parent[temp.unwrap().index()];
                }
            }
        } else {
            self.current_node = None;
        }

        return None;
    }
}

impl<'a> Iterator for TreeIterator<'a> {
    type Item = Entity;
    fn next(&mut self) -> Option<Entity> {
        let r = self.current_node;

        if let Some(current) = self.current_node {
            if let Some(child) = self.tree.first_child[current.index()] {
                self.current_node = Some(child);
            } else {
                let mut temp = Some(current);
                while temp.is_some() {
                    if let Some(sibling) =
                        self.tree.next_sibling[temp.unwrap().index()]
                    {
                        self.current_node = Some(sibling);
                        return r;
                    } else {
                        temp = self.tree.parent[temp.unwrap().index()];
                    }
                }

                self.current_node = None;
            }
        }

        return r;
    }
}

#[cfg(test)]
mod tests {
    use crate::{Entity, GenerationalId, Tree, TreeExt, entity, id::IdManager};

    #[test]
    fn create() {
        let mut entity_manager = IdManager::<Entity>::new();
        entity_manager.create();
        let mut tree = Tree::new();
        let root = Entity::root();
        assert_eq!(root.index(), 0);
        assert_eq!(root.is_window(), true);
        let one = entity_manager.create();
        tree.add(one, root);
        let two = entity_manager.create();
        tree.add(two, one);
        let three = entity_manager.create();
        tree.add(three, one);
        let four = entity_manager.create();
        tree.add(four, two);
        let five = entity_manager.create();
        tree.add(five, two);
        let six = entity_manager.create();
        tree.add(six, three);

        println!("Tree: {:?}", tree);

        // println!("Prime: {}", prime);
        let mut iter = one.tree_iter(&tree);
        let a = iter.next();
        println!("A: {:?}", a);
        println!("B: {:?}", iter.next());
        println!("C: {:?}", iter.next());
        println!("D: {:?}", iter.next());
        println!("E: {:?}", iter.next());
        println!("F: {:?}", iter.next());
        println!("G: {:?}", iter.next());

        for entity in one.tree_iter(&tree) {
            println!("E: {:?}", entity);
        }


    }
}