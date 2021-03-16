

use crate::{Entity, State, Event, WindowEvent, Length, Visibility, Display, PropGet};

// Determines the next and previous focus entities for each entity in the hierarchy
// Only needs to be called when the hierarchy changes or the set_next_focus or set_prev_focus methods are called
// pub fn apply_focus(state: &mut State) {

//     let mut current = Entity::root();

//     for entity in state.hierarchy.into_iter() {
//         let next_focus = 
//     }
// }