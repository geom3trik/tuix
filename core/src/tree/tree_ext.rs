use crate::{Entity, Tree, WindowIterator};

use super::parent_iter::ParentIterator;
use super::child_iter::ChildIterator;
use super::branch_iter::BranchIterator;
use super::tree_iter::TreeIterator;

/// Trait which provides methods for qerying the tree.
pub trait TreeExt {
    /// Returns the parent of the entity.
    fn parent(&self, tree: &Tree) -> Option<Entity>;
    fn prev_sibling(&self, tree: &Tree) -> Option<Entity>;
    fn next_sibling(&self, tree: &Tree) -> Option<Entity>;
    /// Returns true if the entity is a sibling of the specified other entity.
    fn is_sibling_of(&self, tree: &Tree, other: Entity) -> bool;
    /// Returns true if the entity is a child of the specified other entity.
    fn is_child_of(&self, tree: &Tree, other: Entity) -> bool;
    /// Returns true if the entity is a descendant of the specified other entity.
    fn is_descendant_of(&self, tree: &Tree, other: Entity) -> bool;

    /// Returns an iterator over the ancestors of an entity.
    fn parent_iter<'a>(&self, tree: &'a Tree) -> ParentIterator<'a>;
    /// Returns an iterator over the children of an entity.
    fn child_iter<'a>(&self, tree: &'a Tree) -> ChildIterator<'a>;
    /// Returns an iterator over the tree, starting from the entity. 
    fn tree_iter<'a>(&self, tree: &'a Tree) -> TreeIterator<'a>;
    /// Returns an iterator over a branch of the tree, starting from the entity.
    fn branch_iter<'a>(&self, tree: &'a Tree) -> BranchIterator<'a>;
    /// Returns an iteraot over the entities within the same window, starting from the emtity.
    fn window_iter<'a>(&self, tree: &'a Tree) -> WindowIterator<'a>;
}

impl TreeExt for Entity {
    fn parent(&self, tree: &Tree) -> Option<Entity> {
        tree.get_parent(*self)
    }

    fn prev_sibling(&self, tree: &Tree) -> Option<Entity> {
        tree.get_prev_sibling(*self)
    }

    fn next_sibling(&self, tree: &Tree) -> Option<Entity> {
        tree.get_next_sibling(*self)
    }

    fn is_sibling_of(&self, tree: &Tree, entity: Entity) -> bool {
        tree.is_sibling(*self, entity)
    }

    fn is_child_of(&self, tree: &Tree, entity: Entity) -> bool {
        if *self == Entity::null() {
            return false;
        }

        if let Some(parent) = tree.get_parent(*self) {
            if parent == entity {
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }

    fn is_descendant_of(&self, tree: &Tree, entity: Entity) -> bool {
        if *self == Entity::null() {
            return false;
        }

        for parent in self.parent_iter(tree) {
            if parent == entity {
                return true;
            }
        }

        false
    }

    fn parent_iter<'a>(&self, tree: &'a Tree) -> ParentIterator<'a> {
        ParentIterator {
            tree,
            current: Some(*self),
        }
    }

    fn child_iter<'a>(&self, tree: &'a Tree) -> ChildIterator<'a> {
        ChildIterator {
            tree,
            current_forward: tree.get_first_child(*self),
            current_backward: tree.get_last_child(*self),
        }
    }

    fn tree_iter<'a>(&self, tree: &'a Tree) -> TreeIterator<'a> {
        TreeIterator {
            tree,
            current_node: Some(*self),
        }
    }

    fn branch_iter<'a>(&self, tree: &'a Tree) -> BranchIterator<'a> {
        BranchIterator {
            tree,
            start_node: *self,
            current_node: Some(*self),
        }
    }

    fn window_iter<'a>(&self, tree: &'a Tree) -> WindowIterator<'a> {
        WindowIterator {
            tree,
            start_node: *self,
            current_node: Some(*self),
        }
    }
}
