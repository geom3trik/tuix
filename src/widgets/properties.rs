#![allow(dead_code)]

use crate::component::entity::Entity;
use crate::component::hierarchy::{
    Hierarchy, IntoHIterator, IntoHierarchyIterator, IntoParentIterator,
};
use crate::component::state::WidgetState;
use crate::component::storage::Storage;
use crate::events::{EventHandler, EventQueue, WidgetEvent, WidgetList};
use crate::mouse::*;
use crate::widget::{Widget, WidgetBuilder};

use crate::component::style::display::*;
use crate::widget_system::WidgetSystem;

use crate::component::style::flexbox::*;
use crate::component::style::layout::*;
use crate::component::style::text::*;
use crate::widget::button::Button;
use crate::widget::checkbox::Checkbox;
use crate::widget::intbox::IntBox;
use crate::widget::panel::Panel;
use crate::widget::scrollbar::{Direction, ScrollBar, Slot};
use crate::widget::slider::Slider;

use crate::node::*;

pub struct Properties {
    id: Entity,
    panel_stack: Entity,
    vertical_scroll: Entity,
}

impl Properties {
    pub fn new(
        state: &mut WidgetState,
        widget_list: &mut WidgetList,
        parent: Entity,
        graph: &Graph,
    ) -> Self {
        let id = state.add(parent).unwrap();

        id.set_background(state, nanovg::Color::from_rgb(56, 56, 56));

        // TODO
        // let tool_bar = state.add(id).unwrap();
        // tool_bar
        //     .set_flex_basis(state, 40.0)
        //     .set_flex_grow(state, 0.0)
        //     .set_background(state, nanovg::Color::from_rgb(56, 56, 56));

        let panel_stack = state.add(id).unwrap();
        panel_stack
            .set_width(state, 240.0)
            .set_height(state, 1.0)
            .set_display(state, Display::Flexbox)
            .set_flex_direction(state, FlexDirection::Column)
            .set_background(state, nanovg::Color::from_rgb(56, 56, 56))
            .set_clip_widget(state, id);

        let mut panel_height_sum = 0.0;

        for node in graph.graph_data.nodes.iter() {
            let node_id = node.get_id();

            let panel = Panel::new(state, widget_list, panel_stack, graph, node_id);
            let panel = widget_list.push(panel);
            panel_height_sum += state.transform.get_local_height(panel);
        }

        //let mut dim_ratio = height / child_height_sum;

        let mut vertical_scroll = ScrollBar::new(state, id, Direction::Vertical);
        //vertical_scroll.dim_ratio = dim_ratio;
        let vertical_scroll = widget_list.push(vertical_scroll);
        vertical_scroll.set_justify(state, JustifySelf::End);

        //Set the clip widgets for the panels to be the same as the panel_stack
        //This should be done automatically bu the cascade storage but isn't working for some reason
        for child in panel_stack.into_iter(&state.hierarchy) {
            state.style.clip_widget.set(child, id);
        }

        // let resize_bar = state.add(id).unwrap();
        // resize_bar
        //     .set_width(state, 6.0)
        //     .set_height(state, 1.0)
        //     .set_background(state, nanovg::Color::from_rgb(100, 100, 100));

        Properties {
            id: id,
            panel_stack: panel_stack,
            vertical_scroll: vertical_scroll,
        }
    }

    pub fn get_entity(&self) -> Entity {
        self.id
    }
}

impl EventHandler for Properties {
    fn handle_event(
        &mut self,
        state: &mut WidgetState,
        event: &WidgetEvent,
        event_handlers: &mut Vec<Box<EventHandler>>,
        event_queue: &mut EventQueue,
    ) {
        match event {
            WidgetEvent::MouseButton(button, action, mods) => match button {
                MouseButton::Left => match action {
                    MouseButtonState::Pressed => {}

                    MouseButtonState::Released => {}
                },

                _ => {}
            },

            WidgetEvent::MouseMotion(x, y) => {
                //let posx = x - state.transform.get_global_x(self.id);

                // if state.style.visibility.get(self.id) == Visibility::Visible {
                //     if (posx >= -2.0 && posx <= 2.0) || self.resizing {
                //         self.resize_bar.set_visibility(state, Visibility::Visible);
                //     } else {
                //         self.resize_bar.set_visibility(state, Visibility::Invisible);
                //     }
                // }

                // if self.resizing {
                //     let dx = self.pressed_x - x;

                //     let mut new_width = self.resize_width + dx;
                //     if new_width < 250.0 {
                //         new_width = 250.0
                //     } else if new_width > 500.0 {
                //         new_width = 500.0
                //     }

                //     self.id.set_flex_basis(state, new_width);
                //     // Set the width of the panel stack while resizing taking into account the scrollbar
                //     if state.transform.get_local_x(self.vertical_scroll) == 0.0 {
                //         self.panel_stack.set_width(state, new_width - 10.0);
                //     } else {
                //         self.panel_stack.set_width(state, 1.0);
                //     }
                // }
            }

            WidgetEvent::WidgetSizeChanged(entity, width, height) => {
                if *entity == self.panel_stack {
                    println!("Size Changed Event: {}", height);
                    let mut child_height_sum = 0.0;
                    for child in self.panel_stack.child_iter(&state.hierarchy) {
                        let visible = state.style.visibility.get(child);
                        if visible == Visibility::Visible {
                            child_height_sum += state.transform.get_global_height(child);
                        }
                    }

                    let height = state.transform.get_global_height(self.id);

                    let mut dim_ratio = height / child_height_sum;

                    if dim_ratio > 1.0 {
                        dim_ratio = 1.0;
                    }

                    if dim_ratio < 1.0 {
                        self.panel_stack
                            .set_width(state, state.transform.get_global_width(self.id) - 10.0);
                        self.vertical_scroll.set_posx(state, 0.0);
                    } else {
                        self.panel_stack.set_width(state, 1.0);
                        self.vertical_scroll.set_posx(state, 20.0);
                    }

                    // if let Some(first_child) = state.hierarchy.get_first_child(self.panel_stack) {
                    //     let pos_ratio = state.transform.get
                    // }

                    //let posx = pos_ratio * child_height_sum;
                    // state.transform.set_local_y(self.panel_stack, -posx);

                    event_queue.push(WidgetEvent::WidgetValueChanged(
                        self.id,
                        "width".to_string(),
                        dim_ratio,
                    ));
                }
            }

            WidgetEvent::WidgetValueChanged(entity, name, value) => {
                if *entity == self.vertical_scroll {
                    println!("Properties value changed: {}", value);
                    let mut child_height_sum = 0.0;
                    for child in self.panel_stack.child_iter(&state.hierarchy) {
                        let visible = state.style.visibility.get(child);
                        if visible == Visibility::Visible {
                            child_height_sum += state.transform.get_global_height(child);
                        }
                    }

                    let height = state.transform.get_global_height(self.panel_stack);

                    let mut posx = value * (child_height_sum);
                    if posx < 0.0 {
                        posx = 0.0;
                    }

                    state.transform.set_local_y(self.panel_stack, -posx.round());

                    println!("resize pos: {}", -posx);
                }
            }

            WidgetEvent::Key(scancode, virtual_keycode, action, mods) => {
                if let Some(virtual_keycode) = *virtual_keycode {
                    if virtual_keycode == glutin::VirtualKeyCode::Left {
                        match action {
                            MouseButtonState::Pressed => {
                                println!("pos: {}", state.transform.get_global_y(self.panel_stack));
                            }

                            _ => {}
                        }
                    }
                }
            }

            _ => {}
        }
    }

    fn get_entity(&self) -> Entity {
        self.id
    }
}
