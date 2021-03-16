#![allow(dead_code)]

use crate::{Checkbox, Element, Message, entity::Entity};
use crate::mouse::*;
use crate::{BuildHandler, Event, EventHandler, HierarchyTree, Propagation, WindowEvent};
use crate::{PropSet, State};

use crate::style::*;
use crate::widgets::*;

use crate::state::hierarchy::IntoChildIterator;

// Notes:
// When user clicks menu, the container should appear
// When container is visible, clicking on a menu item activates the item
//  Need the option to close the menu on item press

#[derive(Debug, Clone, PartialEq)]
pub enum MenuEvent {
    Open(Entity),
    Close(Entity),
    Hover(Entity),
    CloseAll(Entity),
    OpenHover(bool),
}

#[derive(Debug, Copy, Clone)]
pub enum MenuPosition {
    Auto, // TODO
    Down,
    Right,
}

pub struct Menu {
    container: Entity,
    open: bool,
}

impl Menu {
    pub fn new() -> Self {
        Menu {
            container: Entity::default(),
            open: false,
        }
    }
}

impl Widget for Menu {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity.set_flex_direction(state, FlexDirection::Column);

        self.container = Element::new().build(state, entity, |builder| {
            builder
                .set_position(Position::Absolute)
                .set_z_order(1)
                .class("container")
        });

        state.style.insert_element(entity, "menu");

        self.container
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {


        if let Some(menu_event) = event.message.downcast::<MenuEvent>() {
            match menu_event {
                MenuEvent::Open(menu) => {
                    if *menu == entity {
                        entity.set_checked(state, true);
                        state.capture(entity);
                        self.open = true;
                    }
                }

                MenuEvent::Close(menu) => {
                    if *menu == entity {
                        entity.set_checked(state, false);
                        state.release(entity);
                        self.open = false;                        
                    }
                }

                _=> {}
            }
        }

        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::MouseDown(button) => {
                    if *button == MouseButton::Left {
                        if state.hovered == entity {
                            if !self.open {
                                state.insert_event(Event::new(MenuEvent::Open(entity)).target(entity));
                            } else {
                                state.insert_event(Event::new(MenuEvent::Close(entity)).target(entity));
                            }      
                        } else {
                            if self.open {
                                if state.hovered.is_descendant_of(&state.hierarchy, entity) {
                                    state.insert_event(Event::new(WindowEvent::MouseDown(*button)).target(state.hovered));
                                    self.open = false;
                                }

                                state.insert_event(Event::new(MenuEvent::Close(entity)).target(entity));                                
                            }
                        }
                    }
                }

                WindowEvent::MouseOver => {
                    if event.target == entity {
                        state.insert_event(Event::new(MenuEvent::Hover(entity)).target(entity));
                    }
                }

                _=> {}


            }
        }

    }
}


pub struct MenuBar {
    open_menu: Entity,
}

impl MenuBar {
    pub fn new() -> Self {
        Self {
            open_menu: Entity::default(),
        }
    }
}

impl Widget for MenuBar {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(menu_event) = event.message.downcast::<MenuEvent>() {
            match menu_event {
                MenuEvent::Open(menu) => {
                    self.open_menu = *menu;
                }

                MenuEvent::Close(menu) => {
                    self.open_menu = Entity::default();
                }

                MenuEvent::Hover(menu) => {
                    if self.open_menu != Entity::default() {
                        state.insert_event(Event::new(MenuEvent::Close(self.open_menu)).target(entity).propagate(Propagation::Fall));
                        state.insert_event(Event::new(MenuEvent::Open(*menu)).target(entity).propagate(Propagation::Fall));
                        
                        self.open_menu = *menu;

                    }
                }

                _=> {}
            }
        }
    }
    
}