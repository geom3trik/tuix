

pub(crate) mod cache;
pub use cache::*;

pub use morphorm::{Cache, GeometryChanged};

pub(crate) mod node;

pub(crate) mod hierarchy;

use morphorm::{Hierarchy};
use crate::{Event, Propagation, State, Tree, WindowEvent};

pub(crate) fn geometry_changed(state: &mut State, tree: &Tree) {
    for node in tree.down_iter() {
        let geometry_changed = state.data.geometry_changed(node);
        if !geometry_changed.is_empty() {
            state.insert_event(Event::new(WindowEvent::GeometryChanged(geometry_changed)).target(node).propagate(Propagation::Down));
        }

        state.data.set_geo_changed(node, morphorm::GeometryChanged::POSX_CHANGED, false);
        state.data.set_geo_changed(node, morphorm::GeometryChanged::POSY_CHANGED, false);
        state.data.set_geo_changed(node, morphorm::GeometryChanged::WIDTH_CHANGED, false);
        state.data.set_geo_changed(node, morphorm::GeometryChanged::HEIGHT_CHANGED, false);
    }
}

pub fn reset_geometry_changed(state: &mut State, tree: &Tree) {
    for node in tree.down_iter() {

        state.data.set_geo_changed(node, morphorm::GeometryChanged::POSX_CHANGED, false);
        state.data.set_geo_changed(node, morphorm::GeometryChanged::POSY_CHANGED, false);
        state.data.set_geo_changed(node, morphorm::GeometryChanged::WIDTH_CHANGED, false);
        state.data.set_geo_changed(node, morphorm::GeometryChanged::HEIGHT_CHANGED, false);
    }
}

