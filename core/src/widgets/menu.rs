#![allow(dead_code)]

use crate::entity::Entity;
use crate::mouse::*;
use crate::{
    BuildHandler, Event, EventHandler, HierarchyTree, Propagation, WidgetEvent, WindowEvent,
};
use crate::{PropSet, State};

use crate::state::style::*;
use crate::widgets::Button;

use crate::state::hierarchy::IntoChildIterator;

// Notes:
// When user clicks menu, the container should appear
// When container is visible, clicking on a menu item activates the item
//  Need the option to close the menu on item press

#[derive(Debug, Clone, PartialEq)]
pub enum MenuEvent {
    Open(Entity),
    Close(Entity),
    CloseAll(Entity),
    OpenHover(bool),
}

//impl Message for MenuEvent {}

#[derive(Debug, Copy, Clone)]
pub enum MenuPosition {
    Auto, // TODO
    Down,
    Right,
}

#[derive(Clone)]
pub struct Menu {
    //entity: Entity,
    container: Entity,
    //options: Vec<(String, Option<Event>)>,
    text: String,
    menu_position: MenuPosition,
    open_on_hover: bool,
    open: bool,
}

impl Menu {
    pub fn new(text: &str, menu_position: MenuPosition) -> Self {
        Menu {
            //entity: state.add(state.root),
            container: Entity::null(),
            //options: Vec::new(),
            text: text.to_string(),
            menu_position: menu_position,
            open_on_hover: false,
            open: false,
        }
    }

    // pub fn add_item(mut self, name: &str, event: Option<Event>) -> Self {
    //     self.options.push((name.to_string(), event));

    //     // self.options.insert(name.to_string(), v: V)
    //     self
    // }
}

impl BuildHandler for Menu {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
            .set_text(state, &self.text)
            .set_flex_direction(state, FlexDirection::Column);

        self.container = Button::new().build(state, entity, |builder| {
            builder
                .set_flex_direction(FlexDirection::Column)
                .set_position(Position::Absolute)
                .class("container")
        });

        match self.menu_position {
            MenuPosition::Down => {
                self.container.set_top(state, Length::Percentage(1.0));
            }

            MenuPosition::Right => {
                self.container.set_left(state, Length::Percentage(1.0));
            }

            _ => {}
        }

        //state.style.checked.set(entity, false);
        entity.set_checked(state, false);

        self.container.set_z_order(state, 1);

        //self.container.set_visibility(state, Visibility::Invisible);

        state.style.insert_element(entity, "menu");

        self.container
    }
}

impl EventHandler for Menu {
    // fn add_child(&mut self, child: Entity, state: &mut State) {
    //     if child == self.container {
    //         return;
    //     } else {
    //         //println!("Add Child Event - Parent: {:?}  Child: {:?}", parent, child);

    //         let height = state
    //             .style
    //             .height
    //             .get(self.container)
    //             .cloned()
    //             .unwrap_or_default();
    //         //self.container.set_height(state, height + 30.0);

    //         state.hierarchy.set_parent(child, self.container);
    //         state.style.z_order.set(child, 1);
    //     }
    // }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) -> bool {
        if let Some(menu_event) = event.message.downcast::<MenuEvent>() {
            match menu_event {
                MenuEvent::Open(_id) => {
                    if event.target == entity {
                        state.capture(entity);
                        entity.set_checked(state, true);
                        self.open = true;
                    } else {
                        self.open_on_hover = true;
                    }
                }

                MenuEvent::Close(id) => {
                    if *id == entity {
                        if entity.parent(&state.hierarchy).unwrap() == state.root {
                            state.release(entity);
                        }
                        //state.style.checked.set(entity, false);
                        entity.set_checked(state, false);
                        self.open = false;
                    }
                    // else {
                    //     state.capture(entity);
                    // }
                }

                MenuEvent::CloseAll(_entity) => {
                    if let Some(_visibility) = state.style.visibility.get(self.container) {
                        //self.container.set_visibility(state, Visibility::Invisible);
                    }

                    //state.style.checked.set(entity, false);
                    entity.set_checked(state, false);
                    self.open = false;

                    state.release(entity);
                }

                MenuEvent::OpenHover(val) => {
                    self.open_on_hover = *val;
                }
            }

            state.insert_event(Event::new(WindowEvent::Restyle));
        }

        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::MouseDown(button) => match button {
                    MouseButton::Left => {
                        if state.hovered == entity {
                            if self.open {
                                state.insert_event(
                                    Event::new(MenuEvent::Close(entity))
                                        .target(entity)
                                        .propagate(Propagation::Direct),
                                );
                            } else {
                                state.insert_event(
                                    Event::new(MenuEvent::Open(entity))
                                        .target(entity)
                                        .propagate(Propagation::Fall),
                                );
                            }

                            // if let Some(visibility) = state.style.visibility.get(self.container) {
                            //     match visibility {
                            //         Visibility::Invisible => {
                            //             state.insert_event(
                            //                 Event::new(MenuEvent::Open(entity))
                            //                     .target(entity)
                            //                     .propagate(Propagation::None),
                            //             );
                            //         }

                            //         Visibility::Visible => {
                            //             state.insert_event(
                            //                 Event::new(MenuEvent::Close(entity))
                            //                     .target(entity)
                            //                     .propagate(Propagation::None),
                            //             );
                            //         }
                            //     }
                            // }

                            state.insert_event(Event::new(WindowEvent::Restyle));

                        //return true;
                        } else {
                            // state.insert_event(
                            //     Event::new(WindowEvent::MouseDown(*button, *mods))
                            //         .target(state.hovered)
                            //         .propagate(false),
                            // );

                            if event.target == entity {
                                for entity in self.container.child_iter(&state.hierarchy) {
                                    if entity == state.hovered {
                                        // Forward the mouse down event to the option that's hovered
                                        state.insert_event(
                                            Event::new(WindowEvent::MouseDown(*button))
                                                .target(state.hovered)
                                                .propagate(Propagation::Direct),
                                        );

                                        return false;
                                    }
                                }

                                state.insert_event(
                                    Event::new(MenuEvent::CloseAll(entity)).target(entity),
                                );
                                state.insert_event(Event::new(WindowEvent::Restyle));
                            }
                        }
                    }
                    _ => {}
                },

                WindowEvent::MouseUp(button) => {
                    match button {
                        MouseButton::Left => {
                            for entity in self.container.child_iter(&state.hierarchy) {
                                if entity == state.hovered {
                                    // Forward the mouse down event to the option that's hovered
                                    state.insert_event(
                                        Event::new(WindowEvent::MouseUp(*button))
                                            .target(state.hovered)
                                            .propagate(Propagation::Direct),
                                    );
                                    //state.insert_event(Event::new(StyleEvent::Restyle));
                                    //state.insert_event(Event::new(MenuEvent::Close(entity)).target(entity));
                                    //state.captured = Entity::null();

                                    state.insert_event(
                                        Event::new(MenuEvent::CloseAll(entity)).target(entity),
                                    );

                                    return false;
                                }
                            }

                            // for entity in self.container.child_iter(&state.hierarchy) {
                            //     if entity == state.hovered {
                            //         state.captured = Entity::null();
                            //         self.container.set_visibility(state, Visibility::Invisible);
                            //         //state.insert_event(Event::new(WindowEvent::MouseUp(*button, *mods)).target(state.hovered).propagate(false));
                            //         state.insert_event(Event::new(StyleEvent::Restyle));
                            //         return false;
                            //     }
                            // }
                        }

                        _ => {}
                    }
                }

                WindowEvent::MouseOver => {
                    if event.origin == Entity::null() {
                        state.insert_event(
                            Event::new(WindowEvent::MouseOver)
                                .origin(event.target)
                                .target(self.container)
                                .propagate(Propagation::Fall),
                        );

                        return true;
                    }

                    if event.origin == entity {
                        if self.open_on_hover {
                            state.insert_event(
                                Event::new(MenuEvent::Open(entity))
                                    .target(entity)
                                    .propagate(Propagation::Fall),
                            );

                            return true;
                        }
                    } else if event.origin.is_descendant_of(&state.hierarchy, entity) {

                        //if event.target != self.container {
                        // state.insert_event(
                        //     Event::new(WindowEvent::MouseOver)
                        //         .origin(event.target)
                        //         .target(self.container)
                        //         .propagate(Propagation::Fall),
                        // );

                        //return true;
                        //}

                        state.insert_event(
                            Event::new(MenuEvent::Open(entity))
                                .target(entity)
                                .propagate(Propagation::Fall),
                        );

                    //return true;

                    //Do nothing
                    } else {
                        state.insert_event(
                            Event::new(MenuEvent::Close(entity))
                                .target(entity)
                                .propagate(Propagation::Fall),
                        );

                        return true;
                    }

                    //return true;

                    // if event.target.is_child_of(&state.hierarchy, entity) {
                    //     state.insert_event(
                    //         Event::new(MenuEvent::CloseAll(entity))
                    //             .target(self.container)
                    //             .propagate(Propagation::Fall),
                    //     );
                    // }

                    //println!("Mouse over menu");
                }

                WindowEvent::MouseOut => {
                    //println!("Mouse over menu");
                    // state.insert_event(
                    //     Event::new(MenuEvent::Close(entity))
                    //         .target(entity)
                    //         .propagate(Propagation::Direct),
                    // );
                }

                _ => {}
            }
        }

        if let Some(wentityget_event) = event.message.downcast::<WidgetEvent>() {
            match wentityget_event {
                WidgetEvent::MouseEnter(id) => {
                    if *id == entity {
                        // state.insert_event(
                        //     Event::new(MenuEvent::Open(entity))
                        //         .target(entity)
                        //         .propagate(Propagation::None),
                        // );

                        // if let Some(visibility) = state.style.visibility.get(self.container) {
                        //     match visibility {
                        //         Visibility::Invisible => {
                        //             state.insert_event(
                        //                 Event::new(MenuEvent::Open(entity)).target(entity).propagate(false),
                        //             );
                        //         }

                        //         Visibility::Visible => {
                        //             state.insert_event(
                        //                 Event::new(MenuEvent::Close(entity)).target(entity).propagate(false),
                        //             );
                        //         }
                        //     }
                        // }

                        state.insert_event(Event::new(WindowEvent::Restyle));
                    }
                }

                WidgetEvent::MouseLeave(id) => {
                    if *id == entity {
                        // for child in self.container.child_iter(&state.hierarchy) {
                        //     if child == state.hovered {
                        //         // Forward the mouse down event to the option that's hovered
                        //         //state.insert_event(Event::new(WindowEvent::MouseDown(*button, *mods)).target(state.hovered).propagate(false));
                        //         //state.insert_event(Event::new(StyleEvent::Restyle));
                        //         //state.insert_event(Event::new(MenuEvent::Close(entity)).target(entity));
                        //         //state.captured = Entity::null();

                        //         return false;
                        //     }
                        // }

                        //state.insert_event(Event::new(MenuEvent::Close(entity)).target(entity).propagate(false));
                    }

                    state.insert_event(Event::new(WindowEvent::Restyle));
                }

                _ => {}
            }
        }

        false
    }
}
