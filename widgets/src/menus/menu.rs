

use crate::{Popup, PopupEvent, common::*};

// Notes:
// When user clicks menu, the container should appear
// When container is visible, clicking on a menu item activates the item
//  Need the option to close the menu on item press

#[derive(Debug, Clone)]
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

pub struct MenuData {
    //open: bool,
}

pub struct Menu {
    container: Entity,
    open: bool,
    text: String,
}

impl Menu {
    pub fn new(text: &str) -> Self {
        Menu {
            container: Entity::default(),
            open: false,
            text: text.to_string(),
        }
    }
}

impl Widget for Menu {
    type Ret = Entity;
    type Data = MenuData;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        self.container = Popup::new().build(state, entity, |builder| {
            builder
                .set_position_type(PositionType::SelfDirected)
                .set_top(Percentage(100.0))
                // .set_width(Auto)
                // .set_height(Auto)
                .set_width(Pixels(100.0))
                .set_height(Pixels(300.0))
                .set_background_color(Color::red())
                .set_z_order(1)
                .set_clip_widget(Entity::root())
                .class("container")
        });

        entity
            .set_text(state, &self.text)
            .set_element(state, "menu");

        self.container
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        // if let Some(menu_event) = event.message.downcast::<MenuEvent>() {
        //     match menu_event {
        //         MenuEvent::Open(menu) => {
        //             if *menu == entity {
        //                 entity.set_checked(state, true);
        //                 state.capture(entity);
        //                 self.open = true;
        //             }
        //         }

        //         MenuEvent::Close(menu) => {
        //             if *menu == entity {
        //                 entity.set_checked(state, false);
        //                 state.release(entity);
        //                 self.open = false;
        //             }
        //         }

        //         _ => {}
        //     }
        // }

        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::MouseDown(button) => {
                    if *button == MouseButton::Left {
                        if state.hovered == entity {
                            if !self.open {
                                state.insert_event(
                                    Event::new(PopupEvent::Open).target(self.container).propagate(Propagation::Direct),
                                );

                                event.consume();
                                //entity.emit(state, PopupEvent::Switch);
                            } else {
                                // state.insert_event(
                                //     Event::new(MenuEvent::Close(entity)).target(entity),
                                // );
                                //entity.emit(state, PopupEvent::Close);
                            }
                        }
                        // } else {
                        //     if self.open {
                        //         if state.hovered.is_descendant_of(&state.tree, entity) {
                        //             state.insert_event(
                        //                 Event::new(WindowEvent::MouseDown(*button))
                        //                     .target(state.hovered),
                        //             );
                        //             self.open = false;
                        //         }

                        //         state.insert_event(
                        //             Event::new(MenuEvent::Close(entity)).target(entity),
                        //         );
                        //     }
                        // }
                    }
                }

                // WindowEvent::MouseOver => {
                //     if event.target == entity {
                //         state.insert_event(Event::new(MenuEvent::Hover(entity)).target(entity));
                //     }
                // }

                _ => {}
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
    type Data = ();
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity.set_element(state, "menu_bar")
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(menu_event) = event.message.downcast::<MenuEvent>() {
            match menu_event {
                MenuEvent::Open(menu) => {
                    self.open_menu = *menu;
                }

                MenuEvent::Close(_) => {
                    self.open_menu = Entity::default();
                }

                MenuEvent::Hover(menu) => {
                    if self.open_menu != Entity::default() {
                        state.insert_event(
                            Event::new(MenuEvent::Close(self.open_menu))
                                .target(entity)
                                .propagate(Propagation::Fall),
                        );
                        state.insert_event(
                            Event::new(MenuEvent::Open(*menu))
                                .target(entity)
                                .propagate(Propagation::Fall),
                        );

                        self.open_menu = *menu;
                    }
                }

                _ => {}
            }
        }
    }
}
