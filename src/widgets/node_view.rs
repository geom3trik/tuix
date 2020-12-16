use crate::component::entity::Entity;
use crate::component::state::WidgetState;
use crate::component::storage::Storage;
use crate::events::{EventHandler, EventQueue, WidgetEvent, WidgetList};
use crate::mouse::*;
use crate::widget::{Widget, WidgetBuilder};

use crate::component::style::display::*;
use crate::widget_system::WidgetSystem;

use crate::component::style::flexbox::*;
use crate::component::style::text::*;
use crate::widget::audio_viewer::AudioViewer;
use crate::widget::button::Button;
use crate::widget::checkbox::Checkbox;
use crate::widget::intbox::IntBox;
use crate::widget::nodegraph::{NodeGraph, NodeObj};
use crate::widget::panel::Panel;
use crate::widget::properties::Properties;
use crate::widget::scrollbar::{Direction, ScrollBar, Slot};
use crate::widget::slider::Slider;

use crate::node::*;

pub struct NodeView {
    id: Entity,
}

impl NodeView {
    pub fn new(state: &mut WidgetState, widget_list: &mut WidgetList, parent: Entity) -> Self {
        let id = state.add(parent).unwrap();

        id.set_display(state, Display::Flexbox)
            .set_background(state, nanovg::Color::from_rgb(38, 38, 38))
            .set_display(state, Display::Flexbox)
            .set_flex_direction(state, FlexDirection::Column);

        let tool_bar = state.add(id).unwrap();
        tool_bar
            .set_flex_basis(state, 40.0)
            .set_flex_grow(state, 0.0)
            .set_background(state, nanovg::Color::from_rgb(56, 56, 56))
            .set_margin_bottom(state, 1.0);

        // Nodegraph

        let nodegraph = NodeGraph::new(state, widget_list, id);
        nodegraph
            .get_entity()
            .set_background(state, nanovg::Color::from_rgb(38, 38, 38));

        widget_list.push(nodegraph);

        //let properties = Panel::new(state, widget_list, id, graph, 0);
        //let side_bar = state.add(id).unwrap();

        NodeView { id: id }
    }

    pub fn get_entity(&self) -> Entity {
        self.id
    }
}
