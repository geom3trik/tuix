/// Iterates over the entities in a particular window

use crate::{Tree, Entity, GenerationalId};

/// Iterator for iterating through the tree from top to bottom in depth first order
// pub struct WindowIterator<'a> {
//     pub tree: &'a Tree,
//     pub current_node: Option<Entity>,
//     //current_back: Option<Entity>,
// }

// impl<'a> WindowIterator<'a> {
//     /// Skip to next branch
//     pub fn next_branch(&mut self) -> Option<Entity> {
//         let r = self.current_node;
//         if let Some(current) = self.current_node {
//             let mut temp = Some(current);
//             while temp.is_some() {
//                 if let Some(sibling) = self.tree.next_sibling[temp.unwrap().index()]
//                 {
//                     self.current_node = Some(sibling);
//                     return r;
//                 } else {
//                     temp = self.tree.parent[temp.unwrap().index()];
//                 }
//             }
//         } else {
//             self.current_node = None;
//         }

//         return None;
//     }
// }

// impl<'a> Iterator for WindowIterator<'a> {
//     type Item = Entity;
//     fn next(&mut self) -> Option<Entity> {
//         let r = self.current_node;

//         if let Some(current) = self.current_node {
//             if let Some(child) = self.tree.first_child[current.index()] {
//                 // if child.is_window() {
//                 //     self.next_branch();
//                 // }
//                 self.current_node = Some(child);
//             } else {
//                 let mut temp = Some(current);

//                 self.current_node = None;

//                 while temp.is_some() {
//                     if let Some(sibling) =
//                         self.tree.next_sibling[temp.unwrap().index()]
//                     {
//                         // if sibling.is_window() {
//                         //     self.next_branch();
//                         // }
//                         self.current_node = Some(sibling);
//                         break;
//                     } else {
//                         temp = self.tree.parent[temp.unwrap().index()];
//                     }
//                 }

                
//             }
//         }

//         //println!("This: {:?}", self.current_node);

//         if let Some(current) = self.current_node {
//             if current.is_window() {
//                 self.next_branch();
//             }
//         }

//         return r;
//     }
// }

/// An iterator for a branch of the tree tree
pub struct WindowIterator<'a> {
    pub(crate) tree: &'a Tree,
    pub(crate) start_node: Entity,
    pub(crate) current_node: Option<Entity>,
}

impl<'a> WindowIterator<'a> {
    /// Skip to next branch
    pub fn next_branch(&mut self) -> Option<Entity> {
        //println!("Next Branch");
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

        self.current_node = None;
        return None;
    }
}

impl<'a> Iterator for WindowIterator<'a> {
    type Item = Entity;
    fn next(&mut self) -> Option<Entity> {
        let r = self.current_node;

        if let Some(current) = self.current_node {
            if let Some(child) = self.tree.first_child[current.index()] {
                self.current_node = Some(child);
            } else {
                if self.current_node != Some(self.start_node) {

                    self.current_node = None;
                    let mut temp = Some(current);
                    while temp.is_some() {
                        if let Some(sibling) =
                            self.tree.next_sibling[temp.unwrap().index()]
                        {
                            self.current_node = Some(sibling);
                            break;
                        } else {
                            temp = self.tree.parent[temp.unwrap().index()];
                            if Some(self.start_node) == temp {
                                self.current_node = None;
                                temp = None;
                            }
                        }
                    }
                }

                
            }
        }

        if self.current_node == Some(self.start_node) {
            self.current_node = None;
        }

        loop {
            if let Some(current) = self.current_node {
                if current.is_window() {
                    self.next_branch();
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        // if let Some(current) = self.current_node {
        //     if current.is_window() {
        //         self.next_branch();
        //     }
        // }

        return r;
    }
}

#[cfg(test)]
mod tests {
    use crate::{Entity, GenerationalId, Tree, TreeExt, id::IdManager};

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
        //assert_eq!(two.is_window(), true);
        tree.add(two, one);
        let three = entity_manager.create().set_window();
        tree.add(three, one);
        let four = entity_manager.create();
        tree.add(four, three);
        let five = entity_manager.create().set_window();
        tree.add(five, one);
        let six = entity_manager.create();
        tree.add(six, five);

        println!("Tree: {:?}", tree);

        // println!("Prime: {}", prime);
        // let mut iter = one.window_iter(&tree);
        // let a = iter.next();
        // println!("A: {:?}", a);
        // //iter.next_branch();

        // println!("B: {:?}", iter.next());
        // println!("C: {:?}", iter.next());
        // println!("D: {:?}", iter.next());
        // println!("E: {:?}", iter.next());
        // println!("F: {:?}", iter.next());
        // println!("G: {:?}", iter.next());

        for entity in five.window_iter(&tree) {
            println!("E: {:?}", entity);
        }


    }
}