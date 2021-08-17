

pub(crate) mod cache;
pub(crate) use cache::*; 

pub(crate) mod node;
pub (crate) use node::*;

pub(crate) mod hierarchy;
pub (crate) use hierarchy::*;

use morphorm::{Cache, Hierarchy};
use crate::{Event, GeometryChanged, Propagation, State, Tree, WindowEvent};

pub(crate) fn geometry_changed(state: &mut State, tree: &Tree) {
    for node in tree.down_iter() {
        let geometry_changed = state.data.geometry_changed(node);

        state.insert_event(Event::new(WindowEvent::GeometryChanged(GeometryChanged {
            posx: geometry_changed.contains(morphorm::GeometryChanged::POSX_CHANGED),
            posy: geometry_changed.contains(morphorm::GeometryChanged::POSY_CHANGED),
            width: geometry_changed.contains(morphorm::GeometryChanged::WIDTH_CHANGED),
            height: geometry_changed.contains(morphorm::GeometryChanged::HEIGHT_CHANGED),
        })).target(node).propagate(Propagation::Down));

    }
} 
