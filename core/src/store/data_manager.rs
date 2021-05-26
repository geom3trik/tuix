

use crate::{Entity, EventManager, Graph, Node, State, Update, Event};

use fnv::FnvHashMap;

pub struct DataManager {
    pub nodes: FnvHashMap<Entity, Box<dyn Node>>,
    pub graph: Graph,
    pub update_queue: Vec<Event>,
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
        self.update_queue = update_queue.into_iter().collect::<Vec<Event>>();

        state.update_queue.clear();

        let mut mutated_nodes = Vec::new();

        // Apply mutations
        for update in self.update_queue.iter_mut() {
            apply_mutations(state, &self.graph, &mut self.nodes, &mut mutated_nodes, update, update.origin);
        }


        // Update bound widgets
        for mutated_node in mutated_nodes.into_iter().rev() {
            apply_updates(&self.graph, &mut self.nodes, state, event_manager, mutated_node);
        }
        
    }




    fn apply_updates(&mut self, mutated_nodes: &mut Vec<Entity>, state: &mut State, event_manager: &mut EventManager) {
        for mutated_node in mutated_nodes.iter() {
            
            if let Some(data_node) = self.nodes.remove(&mutated_node) {
                let data = data_node.get_data().unwrap();
                if let Some(children) = self.graph.get_children(*mutated_node) {
                    //println!("Loop Over Children");
                    for child in children.iter() {
                        //println!("Child: {}", child);
                        if let Some(event_handler) = event_manager.event_handlers.get_mut(child) {
                            event_handler.on_update(state, *child, data, &self.nodes);
                        } else if let Some(mut event_handler) = self.nodes.remove(child) {
                            event_handler.on_update(state, *child, data, &self.nodes);
                            self.nodes.insert(*child, event_handler);
                        }
                    }                       
                }

                self.nodes.insert(*mutated_node, data_node);
            }
        } 
    }
} 

fn apply_mutations(state: &mut State, graph: &Graph, nodes: &mut FnvHashMap<Entity, Box<dyn Node>>, mutated_nodes: &mut Vec<Entity>, update: &mut Event, id: Entity) {     
    println!("Apply Mutations");
    if let Some(parents) = graph.get_parents(id) {
        for parent in parents.iter() {
            println!("Parent: {}", parent);

            if let Some(data_node) = nodes.get_mut(parent) {
                (*data_node).on_event(state,*parent, update);
                mutated_nodes.push(*parent);
            }

            apply_mutations(state, graph, nodes, mutated_nodes, update, *parent);
            
            
        }
    }
}

fn apply_updates(graph: &Graph, nodes: &mut FnvHashMap<Entity, Box<dyn Node>>, state: &mut State, event_manager: &mut EventManager, mutated_node: Entity) {

        if let Some(data_node) = nodes.remove(&mutated_node) {
            let data = data_node.get_data().unwrap();
            if let Some(children) = graph.get_children(mutated_node) {
                //println!("Loop Over Children");
                for child in children.iter() {
                    //println!("Child: {}", child);
                    if let Some(event_handler) = event_manager.event_handlers.get_mut(child) {
                        event_handler.on_update(state, *child, data, &nodes);
                    } else if let Some(mut event_handler) = nodes.remove(child) {
                        event_handler.on_update(state, *child, data, &nodes);
                        nodes.insert(*child, event_handler);
                    }

                    apply_updates(graph, nodes, state, event_manager, *child);

                }                       
            }

            nodes.insert(mutated_node, data_node);
        }

        
}