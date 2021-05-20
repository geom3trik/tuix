

use crate::{Entity, EventManager, Graph, Node, State, Update};

use fnv::FnvHashMap;

pub struct DataManager {
    pub nodes: FnvHashMap<Entity, Box<dyn Node>>,
    pub graph: Graph,
    pub update_queue: Vec<Update>,
}

impl DataManager {

    pub fn new() -> Self {
        Self {
            nodes: FnvHashMap::default(),
            graph: Graph::new(),
            update_queue: Vec::new(),
        }
    }
    // Loop through the mutations and propagate them to the data nodes
    pub fn flush_updates(&mut self, state: &mut State, event_manager: &mut EventManager) {

        if state.data_graph.changed {
            self.graph = state.data_graph.clone();
            state.data_graph.changed = false;
        }

        self.update_queue.clear();

        self.nodes.extend(state.data_nodes.drain());

        let update_queue = state.update_queue.clone();
        self.update_queue = update_queue.into_iter().collect::<Vec<Update>>();

        state.update_queue.clear();

        let mut mutated_nodes = Vec::new();

        // Apply mutations
        self.apply_mutations(&mut mutated_nodes);


        // Update bound widgets
        self.apply_updates(&mut mutated_nodes, state, event_manager);
    }

    fn apply_mutations(&mut self, mutated_nodes: &mut Vec<Entity>) {
        for update in self.update_queue.iter_mut() {
            //let target = update.target;
            if update.target != Entity::null() {
                if let Some(data_node) = self.nodes.get_mut(&update.target) {

                    if update.mutator.borrow_mut()(&mut **data_node) {
                        // TODO: change detection
                        mutated_nodes.push(update.target);
                    }
                }
            } else {
                if let Some(parents) = self.graph.get_parents(update.origin) {
                    for parent in parents.iter() {
                        if let Some(data_node) = self.nodes.get_mut(parent) {

                            if update.mutator.borrow_mut()(&mut **data_node) {
                                // TODO: change detection
                                mutated_nodes.push(*parent);
                            }
                            
                            
                        }                    
                    }
                }                
            }

        }
    }

    fn apply_updates(&mut self, mutated_nodes: &mut Vec<Entity>, state: &mut State, event_manager: &mut EventManager) {
        for mutated_node in mutated_nodes.iter() {
            if let Some(data_node) = self.nodes.get(&mutated_node) {
                if let Some(children) = self.graph.get_children(*mutated_node) {
                    //println!("Loop Over Children");
                    for child in children.iter() {
                        //println!("Child: {}", child);
                        if let Some(event_handler) = event_manager.event_handlers.get_mut(child) {
                            event_handler.on_update(state, *child, data_node, &self.nodes);
                        }
                    }                       
                }
             
            }

        } 
    }
}