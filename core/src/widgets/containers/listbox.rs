
use crate::widgets::*;

use crate::HierarchyTree;

#[derive(Debug, Clone, PartialEq)]
pub enum ListEvent {
    CheckAll,
}

#[derive(Default)]
pub struct List {
    checked_entity: Entity,
    single: bool,
}

impl List {
    pub fn new() -> Self {
        Self {
            checked_entity: Entity::null(),
            single: true,
        }
    }

    pub fn set_multi(mut self) -> Self {
        self.single = false;
        self
    }
}

impl Widget for List {
    type Ret = Entity;
    fn on_build(&mut self, builder: Builder) -> Self::Ret {
        //state.focused = entity;
        builder
            .set_focusability(false)
            .set_element("list")
            .entity()
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::KeyDown(_, key) => match key {
                    Some(Key::ArrowDown) | Some(Key::ArrowRight) => {
                        if let Some(next_entity) =
                            state.hierarchy.get_next_sibling(self.checked_entity)
                        {
                            state.insert_event(
                                Event::new(CheckboxEvent::Unchecked)
                                    .target(self.checked_entity)
                                    .origin(entity)
                                    .propagate(Propagation::Direct),
                            );
                            state.insert_event(
                                Event::new(CheckboxEvent::Checked)
                                    .target(next_entity)
                                    .origin(entity)
                                    .propagate(Propagation::Direct),
                            );
                            self.checked_entity = next_entity;

                            event.consume();
                        }
                    }

                    Some(Key::ArrowUp) | Some(Key::ArrowLeft) => {
                        if let Some(prev_entity) =
                            state.hierarchy.get_prev_sibling(self.checked_entity)
                        {
                            state.insert_event(
                                Event::new(CheckboxEvent::Unchecked)
                                    .target(self.checked_entity)
                                    .origin(entity)
                                    .propagate(Propagation::Direct),
                            );
                            state.insert_event(
                                Event::new(CheckboxEvent::Checked)
                                    .target(prev_entity)
                                    .origin(entity)
                                    .propagate(Propagation::Direct),
                            );
                            self.checked_entity = prev_entity;

                            event.consume();
                        }
                    }

                    _ => {}
                },

                _ => {}
            }
        }

        if let Some(checkbox_event) = event.message.downcast::<CheckboxEvent>() {
            match checkbox_event {
                CheckboxEvent::Unchecked => {
                    if self.single {
                        if event.target != entity {
                            event.consume();
                        }
                    }
                }

                CheckboxEvent::Checked => {
                    if self.single {
                        if event.target.is_descendant_of(&state.hierarchy, entity) {
                            if event.target != entity && event.origin != entity {
                                state.insert_event(
                                    Event::new(CheckboxEvent::Unchecked)
                                        .target(entity)
                                        .origin(event.target)
                                        .propagate(Propagation::Fall),
                                );

                                event.consume();
                            }

                            if event.target != entity && event.origin != entity {
                                state.insert_event(
                                    Event::new(CheckboxEvent::Checked)
                                        .target(event.target)
                                        .origin(entity)
                                        .propagate(Propagation::Direct),
                                );

                                event.consume();
                            }

                            self.checked_entity = event.target;
                        }
                    }
                }

                // CheckboxEvent::Check => {
                //     if event.target != entity {
                //         event.consume();
                //     }

                //     if event.target.is_descendant_of(&state.hierarchy, entity) {
                //         if event.target != entity && event.origin != entity {
                //             state.insert_event(
                //                 Event::new(CheckboxEvent::Uncheck)
                //                     .target(entity)
                //                     .origin(event.target)
                //                     .propagate(Propagation::Fall),
                //             );

                //             event.consume();
                //         }

                //         if event.target != entity && event.origin != entity {
                //             state.insert_event(
                //                 Event::new(CheckboxEvent::Check)
                //                     .target(event.target)
                //                     .origin(entity)
                //                     .propagate(Propagation::Direct),
                //             );

                //             event.consume();
                //         }
                //     }
                // }

                // CheckboxEvent::Uncheck => {
                //     if event.target != entity {
                //         event.consume();
                //     }
                // }
                _ => {}
            }
        }
    }
}

// #[derive(Debug, Clone, PartialEq)]
// pub enum ListboxEvent {
//     Next(Entity),
//     Prev(Entity),
// }

// pub struct ListboxItem {}

// impl ListboxItem {}

// pub struct Listbox {}

// impl Listbox {
//     pub fn new() -> Self {
//         Self {}
//     }
// }

// impl BuildHandler for Listbox {
//     type Ret = Entity;
//     fn on_build(&mut self, _state: &mut State, entity: Entity) -> Self::Ret {
//         entity
//     }
// }

// impl EventHandler for Listbox {
//     fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
//         if let Some(window_event) = event.message.downcast::<WindowEvent>() {
//             match window_event {
//                 WindowEvent::KeyDown(_, key) => match key {
//                     Some(Key::ArrowDown) => {

//                     }

//                     Some(Key::ArrowUp) => {

//                     }

//                     _ => {}
//                 },

//                 _ => {}
//             }
//         }
//     }
// }
