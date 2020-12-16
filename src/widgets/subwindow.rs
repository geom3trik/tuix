#![allow(dead_code)]

use crate::component::entity::Entity;
use crate::component::hierarchy::{Hierarchy, IntoHIterator, IntoHierarchyIterator};
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
use crate::widget::scrollbar::{Direction, ScrollBar, Slot};
use crate::widget::slider::Slider;

pub struct SubwindowContainer {
    id: Entity,
    resizing: bool,
    pressed_x: f32,
    resize_widget: Entity,
    other_widget: Entity,
    resize_widget_flex: f32,
    other_flex: f32,
}

impl SubwindowContainer {
    pub fn new(state: &mut WidgetState, parent: Entity) -> Self {
        let id = state.add(parent).unwrap();

        SubwindowContainer {
            id: id,
            resizing: false,
            pressed_x: 0.0,
            resize_widget: Entity::new(),
            other_widget: Entity::new(),
            resize_widget_flex: 0.0,
            other_flex: 0.0,
        }
    }
}

impl EventHandler for SubwindowContainer {
    fn handle_event(
        &mut self,
        state: &mut WidgetState,
        event: &WidgetEvent,
        event_handlers: &mut Vec<Box<EventHandler>>,
        event_queue: &mut EventQueue,
    ) {
        match event {
            WidgetEvent::MouseButton(button, action, mods) => {
                match button {
                    MouseButton::Left => {
                        match action {
                            MouseButtonState::Pressed => {
                                for child in self.id.child_iter(&state.hierarchy) {
                                    state.style.flex_item.get_mut(child).unwrap().flex_basis =
                                        state.transform.get_global_width(child);
                                    println!(
                                        "{}",
                                        state.style.flex_item.get_mut(child).unwrap().flex_basis
                                    );
                                }

                                for child in self.id.child_iter(&state.hierarchy) {
                                    //state.style.flex_item.get_mut(child).unwrap().flex_basis = state.transform.get_global_width(child);
                                    if state.hovered == child {
                                        if state.mouse.cursorx
                                            >= state.transform.get_global_x(child)
                                                + state.transform.get_global_width(child)
                                                - 10.0
                                            && state.mouse.cursorx
                                                < state.transform.get_global_x(child)
                                                    + state.transform.get_global_width(child)
                                        {
                                            for other in self.id.child_iter(&state.hierarchy) {
                                                if state.transform.get_global_x(other)
                                                    == state.transform.get_global_x(child)
                                                        + state.transform.get_global_width(child)
                                                {
                                                    self.other_widget = other;
                                                    self.other_flex =
                                                        state.style.flex_item.get(other).flex_basis;
                                                    println!("Other: {:?}", other);
                                                }
                                            }
                                            self.resizing = true;
                                            self.pressed_x = state.mouse.cursorx;
                                            self.resize_widget = child;
                                            self.resize_widget_flex =
                                                state.style.flex_item.get(child).flex_basis;
                                        }
                                    }
                                }
                            }

                            MouseButtonState::Released => {
                                self.resizing = false;
                            }
                        }
                    }

                    _ => {}
                }
            }

            WidgetEvent::MouseMotion(x, y) => {
                if self.resizing {
                    let mut dx = x - self.pressed_x + self.resize_widget_flex;

                    if dx <= 50.0 {
                        dx = 50.0;
                    }

                    let mut x = self.resize_widget_flex + self.other_flex - dx;

                    if x <= 50.0 {
                        x = 50.0;
                        dx = self.resize_widget_flex + self.other_flex - x;
                    }

                    println!("dx: {}", dx);
                    state
                        .style
                        .flex_item
                        .get_mut(self.resize_widget)
                        .unwrap()
                        .flex_basis = dx;
                    let width = state.transform.get_global_width(self.id);
                    state
                        .style
                        .flex_item
                        .get_mut(self.other_widget)
                        .unwrap()
                        .flex_basis = x;
                }
            }

            WidgetEvent::WindowResize(w, h) => {
                //println!("Window Resize");
                for child in self.id.child_iter(&state.hierarchy) {
                    //println!("{}", state.transform.get_global_width(child));
                    //state.style.flex_item.get_mut(child).unwrap().flex_basis = state.transform.get_global_width(child);
                }
            }

            _ => {}
        }
    }

    fn get_entity(&self) -> Entity {
        self.id
    }
}

pub struct Subwindow {
    id: Entity,
    resizing: bool,
    pressed_x: f32,
    flex_basis: f32,
}

impl Subwindow {
    pub fn new(state: &mut WidgetState, parent: Entity) -> Subwindow {
        let id = state.add(parent).unwrap();
        //id.set_flex_shrink(state, 1.0);
        Subwindow {
            id: id,
            resizing: false,
            pressed_x: 0.0,
            flex_basis: 0.0,
        }
    }
}

impl EventHandler for Subwindow {
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
                    MouseButtonState::Pressed => {
                        if state.hovered == self.id {
                            if state.mouse.cursorx
                                >= state.transform.get_global_x(self.id)
                                    + state.transform.get_global_width(self.id)
                                    - 10.0
                                && state.mouse.cursorx
                                    < state.transform.get_global_x(self.id)
                                        + state.transform.get_global_width(self.id)
                            {
                                self.resizing = true;
                                self.pressed_x = state.mouse.cursorx;
                                self.flex_basis = state.style.flex_item.get(self.id).flex_basis;
                            }
                        }
                    }

                    MouseButtonState::Released => {
                        self.resizing = false;
                    }
                },

                _ => {}
            },

            WidgetEvent::MouseMotion(x, y) => {
                // if self.resizing {
                //     let dx = x - self.pressed_x + self.flex_basis;
                //     println!("dx: {}", dx);
                //     state.style.flex_item.get_mut(self.id).unwrap().flex_basis = dx;
                // }
            }

            WidgetEvent::WidgetSizeChanged(entity, w, h) => {
                if *entity == self.id {
                    println!("Entity: {:?}, width: {}, height: {}", entity, w, h);
                }
            }

            _ => {}
        }
    }

    fn get_entity(&self) -> Entity {
        self.id
    }
}
