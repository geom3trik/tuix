use crate::{Entity, State, Tree, TreeExt};



// Calculates which layers entities should be drawn to
// 
// fn apply_layers(state: &mut State, tree: &Tree) {

//     for (_, layer) in state.layers.iter_mut() {
//         layer.count = 0;
//     }

//     for entity in tree.into_iter() {
        
//         // Skip Window
//         if entity == Entity::root() {
//             continue;
//         }

//         let parent = entity.parent(tree).unwrap();
    
//         let z_index = state.data.get_z_index(entity);
        
//         if let Some(layer) = state.layers.get(&z_index) {
//             layer.count += 1;
//         } else {

//         }
//     }
// }

// Each z-index gets its own layer
// Need a way to figure out if no entities are linked a particular z-index
//     Loop through layers and set internal count to 0
//     Loop through entities and increment count for layer
//     Remove layers which have 0 count