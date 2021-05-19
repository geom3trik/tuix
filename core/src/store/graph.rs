
use image::gif::Encoder;

use crate::{Entity};

// Represents a graph of data nodes
#[derive(Clone)]
pub struct Graph {
    pub parents: Vec<Vec<Entity>>,
    pub children: Vec<Vec<Entity>>,
    pub changed: bool,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            parents: Vec::new(),
            children: Vec::new(),
            changed: false,
        }
    }

    pub fn add(&mut self, entity: Entity, parent: Entity) {
        if let Some(index) = entity.index() {
            if index >= self.parents.len() {
                self.parents.resize(index + 1, Vec::new());
                self.children.resize(index + 1, Vec::new());
            }

            if let Some(parents) = self.parents.get_mut(index) {
                if !parents.contains(&parent) {
                    parents.push(parent);
                }
            }

            if let Some(parent_index) = parent.index() {
                if let Some(children) = self.children.get_mut(parent_index) {
                    if !children.contains(&entity) {
                        children.push(entity);
                    }
                }
            }

            self.changed = true;
        }
    }

    pub fn get_parents(&self, entity: Entity) -> Option<&Vec<Entity>> {
        if let Some(index) = entity.index() {
            self.parents.get(index)
        } else {
            None
        }
    }

    pub fn get_children(&self, entity: Entity) -> Option<&Vec<Entity>> {
        if let Some(index) = entity.index() {
            self.children.get(index)
        } else {
            None
        }
    }
}