

use crate::{Entity, EventManager, Hierarchy, IntoChildIterator, Node, State, Update, data};

use fnv::FnvHashMap;

pub struct DataManager {
    pub nodes: FnvHashMap<Entity, Box<dyn Node>>,
    pub hierarchy: Hierarchy,
    pub update_queue: Vec<Update>,
}

impl DataManager {

    pub fn new() -> Self {
        Self {
            nodes: FnvHashMap::default(),
            hierarchy: Hierarchy::new(),
            update_queue: Vec::new(),
        }
    }
    // Loop through the mutations and propagate them to the data nodes
    pub fn flush_updates(&mut self, state: &mut State, event_manager: &mut EventManager) {

        if state.data_hierarchy.changed {
            self.hierarchy = state.data_hierarchy.clone();
            state.data_hierarchy.changed = false;
        }

        self.update_queue.clear();

        self.nodes.extend(state.data_nodes.drain());

        let update_queue = state.update_queue.clone();
        self.update_queue = update_queue.into_iter().collect::<Vec<Update>>();

        state.update_queue.clear();

        let mut mutated_nodes = Vec::new();

        // Apply mutations
        for update in self.update_queue.iter_mut() {
            let origin = update.origin;
            if let Some(parent) = self.hierarchy.get_parent(origin) {
                if let Some(data_node) = self.nodes.get_mut(&parent) {
                    update.mutator.borrow_mut()(&mut **data_node);
                    
                    // TODO: change detection
                    mutated_nodes.push(parent);
                }
            }
        }

        // Update bound widgets
        for mutated_node in mutated_nodes.iter() {
            if let Some(data_node) = self.nodes.get(&mutated_node) {
                for child in mutated_node.child_iter(&self.hierarchy) {
                    if let Some(event_handler) = event_manager.event_handlers.get_mut(&child) {
                        event_handler.on_update(state, child, data_node);
                    }
                }                
            }

        } 
    }
}