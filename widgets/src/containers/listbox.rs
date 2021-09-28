use std::marker::PhantomData;

use crate::{CheckButton, common::*};
use crate::{CheckboxEvent};
use tuix_core::{Node, Lens, IntoChildIterator};

use tuix_core::TreeExt;

#[derive(Debug, Clone, PartialEq)]
pub enum ListEvent {
    CheckAll,
}

#[derive(Default)]
pub struct List {
    checked_entity: Entity,
    single: bool,
    selected_index: usize,
}

impl List {
    pub fn new() -> Self {
        Self {
            checked_entity: Entity::null(),
            single: true,
            selected_index: 0,
        }
    }

    pub fn set_multi(mut self) -> Self {
        self.single = false;
        self
    }
}

impl Widget for List {
    type Ret = Entity;
    type Data = ();
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        state.focused = entity;
        entity.set_focusable(state, false);
        entity.set_element(state, "list")
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::KeyDown(_, key) => match key {
                    Some(Key::ArrowDown) | Some(Key::ArrowRight) => {
                        if let Some(next_entity) =
                            state.tree.get_next_sibling(self.checked_entity)
                        {
                            self.selected_index += 1;

                            //TODO
                            // if state.data.get_checkable(next_sibling)

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
                            state.tree.get_prev_sibling(self.checked_entity)
                        {

                            // TODO - prevent underflow
                            self.selected_index -= 1;

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

                CheckboxEvent::Check => {
                    if self.single {
                        if event.target.is_descendant_of(&state.tree, entity) {
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
                                    Event::new(CheckboxEvent::Check)
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

                CheckboxEvent::Checked => {
                    if self.single {
                        if event.target.is_descendant_of(&state.tree, entity) {
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
                _ => {}
            }
        }
    }
}


pub struct ListItem<W> {
    widget: W,
}

impl<W: Widget> ListItem<W> {
    pub fn new(widget: W) -> Self {
        Self {
            widget,
        }
    }
}

impl<W: Widget> Widget for ListItem<W> {
    type Ret = <W as Widget>::Ret;
    type Data = <W as Widget>::Data;

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        self.widget.on_build(state, entity)
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        self.widget.on_event(state, entity, event)
    }

    fn on_update(&mut self, state: &mut State, entity: Entity, data: &Self::Data) {

        for child in entity.child_iter(&state.tree.clone()) {
            if let Some(mut event_handler) = state.event_handlers.remove(&child) {
                event_handler.on_update_(state, child, data);

                state.event_handlers.insert(child, event_handler);
            }
        }

        self.widget.on_update(state, entity, data)
    }
}


pub struct ListView<T, W> {
    single: bool,
    pub selected: usize,

    on_change: Option<Box<dyn Fn(&mut Self, &mut State, Entity)>>,

    template: Option<Box<dyn Fn(&T, usize) -> W>>,
}

impl<T: std::fmt::Debug + Node> ListView<T, CheckButton> {
    pub fn new() -> Self {
        Self {
            single: true,
            selected: 0,
            on_change: None,
            template: None,
        }
    }
}

impl<T: std::fmt::Debug + Node, W: Widget> ListView<T, W> {
    // pub fn new() -> Self 
    // where W: Widget,
    // {
    //     ListView::<T, CheckButton>::with_template(|item, index| CheckButton::with_label(&format!("{:?}", item)))
    // }

    pub fn with_template<F>(template: F) -> Self 
    where F: 'static + Fn(&T, usize) -> W,
    {
        Self {
            single: true,
            selected: 0,
            on_change: None,
            template: Some(Box::new(template)),
        }
    }

    pub fn on_change<F>(mut self, callback: F) -> Self 
    where F: 'static + Fn(&mut Self, &mut State, Entity)
    {
        self.on_change = Some(Box::new(callback));

        self
    }



    // pub fn default() -> Self {

    //     let creator = |item: &T| CheckButton::with_label(&item.to_string());

    //     Self {
    //         checked_entity: Entity::null(),
    //         single: true,
    //         selected_index: 0,
    //         creator: Box::new(creator),
    //         t: PhantomData::default(),
    //     }
    // }

    pub fn set_multi(mut self) -> Self {
        self.single = false;
        self
    }
    
    // This method will be part of a trait (maybe the Widget trait)
    pub fn bind<L: Lens, F>(self, lens: L, converter: F) -> Wrapper<L, Self> 
    where F: 'static + Fn(&<L as Lens>::Target) -> <Self as Widget>::Data
    {
        Wrapper::new(self, lens, converter)
    }
}

impl<T: std::fmt::Debug + Node, W: Widget> Widget for ListView<T, W> {
    type Ret = Entity;
    type Data = Vec<T>;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        state.focused = entity;
        entity.set_focusable(state, false);
        entity.set_element(state, "list")
    }

    fn on_update(&mut self, state: &mut State, entity: Entity, data: &Vec<T>) {

        if state.tree.get_num_children(entity).unwrap() as usize != data.len() {
            
            // Currently rebuilds the entire list of child widgets but would be good to just add the specific ones here
            for child in entity.child_iter(&state.tree.clone()) {
                state.remove(child);
            }

            for (index, item) in data.iter().enumerate() {
                println!("Item: {:?}", item);
                // let item = CheckButton::new()
                //     .set_checked(true)
                //     .build(state, entity, |builder| 
                //         builder
                //             .set_color(Color::black())
                //             .set_child_space(Pixels(0.0))
                //     );

                if let Some(template) = &self.template {
                    ListItem::new((template)(item, index)).build(state, entity, |builder| 
                        builder
                    );
                } else {
                    ListItem::new(CheckButton::with_label(&format!("{:?}", item))).build(state, entity, |builder| 
                        builder
                    );
                }
             
            }            
        }

        for (index, child) in entity.child_iter(&state.tree.clone()).enumerate() {
            
            if let Some(mut event_handler) = state.event_handlers.remove(&child) {
                event_handler.on_update_(state, child, &data[index]);

                state.event_handlers.insert(child, event_handler);
            }
        }
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        
        if let Some(bind_event) = event.message.downcast() {
            match bind_event {
                BindEvent::Bind(target, _) => {
                    if target.is_child_of(&state.tree, entity) {
                        if *target != entity {
                            event.consume();
                        }                        
                    }

                }

                _=> {}
            }
        }
        
        
        // if let Some(window_event) = event.message.downcast::<WindowEvent>() {
        //     match window_event {
        //         WindowEvent::KeyDown(_, key) => match key {
        //             Some(Key::ArrowDown) | Some(Key::ArrowRight) => {
        //                 if let Some(next_entity) =
        //                     state.tree.get_next_sibling(self.checked_entity)
        //                 {
        //                     self.selected += 1;

        //                     state.insert_event(
        //                         Event::new(CheckboxEvent::Unchecked)
        //                             .target(self.checked_entity)
        //                             .origin(entity)
        //                             .propagate(Propagation::Direct),
        //                     );
        //                     state.insert_event(
        //                         Event::new(CheckboxEvent::Checked)
        //                             .target(next_entity)
        //                             .origin(entity)
        //                             .propagate(Propagation::Direct),
        //                     );
        //                     self.checked_entity = next_entity;

        //                     if let Some(callback) = self.on_change.take() {
        //                         (callback)(self, state, entity);

        //                         self.on_change = Some(callback);
        //                     } 

        //                     event.consume();
        //                 }
        //             }

        //             Some(Key::ArrowUp) | Some(Key::ArrowLeft) => {
        //                 if let Some(prev_entity) =
        //                     state.tree.get_prev_sibling(self.checked_entity)
        //                 {

        //                     self.selected -= 1;

        //                     if let Some(callback) = self.on_change.take() {
        //                         (callback)(self, state, entity);

        //                         self.on_change = Some(callback);
        //                     } 

        //                     state.insert_event(
        //                         Event::new(CheckboxEvent::Unchecked)
        //                             .target(self.checked_entity)
        //                             .origin(entity)
        //                             .propagate(Propagation::Direct),
        //                     );
        //                     state.insert_event(
        //                         Event::new(CheckboxEvent::Checked)
        //                             .target(prev_entity)
        //                             .origin(entity)
        //                             .propagate(Propagation::Direct),
        //                     );
        //                     self.checked_entity = prev_entity;

        //                     event.consume();
        //                 }
        //             }

        //             _ => {}
        //         },

        //         _ => {}
        //     }
        // }

        // if let Some(checkbox_event) = event.message.downcast::<CheckboxEvent>() {
        //     match checkbox_event {
        //         CheckboxEvent::Unchecked => {
        //             if self.single {
        //                 if event.target != entity {
        //                     event.consume();
        //                 }
        //             }
        //         }

        //         CheckboxEvent::Checked => {
        //             if self.single {
        //                 if event.target.is_descendant_of(&state.tree, entity) {
        //                     if event.target != entity && event.origin != entity {
        //                         state.insert_event(
        //                             Event::new(CheckboxEvent::Unchecked)
        //                                 .target(entity)
        //                                 .origin(event.target)
        //                                 .propagate(Propagation::Fall),
        //                         );

        //                         event.consume();
        //                     }

        //                     if event.target != entity && event.origin != entity {
        //                         state.insert_event(
        //                             Event::new(CheckboxEvent::Checked)
        //                                 .target(event.target)
        //                                 .origin(entity)
        //                                 .propagate(Propagation::Direct),
        //                         );

        //                         event.consume();
        //                     }

        //                     self.checked_entity = event.target;
        //                 }
        //             }
        //         }
        //         _ => {}
        //     }
        // }
    
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
