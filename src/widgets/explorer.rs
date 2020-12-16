#![allow(dead_code)]

use crate::entity::Entity;
use crate::state::hierarchy::*;
use crate::state::style::*;
use crate::{Builder, Event, EventHandler, EventManager, Message};
use crate::{PropSet, State};

use crate::button::Button;
use crate::checkbox::Checkbox;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum ExplorerMessage {
    ToggleDirectory(Entity),
    ViewDirectory,
    AddDirectory(Entity, u32),
}

impl Message for ExplorerMessage {}

pub struct ExplorerItem {
    pub expanded: bool,
    pub level: u32,
}

impl ExplorerItem {
    pub fn new(lvl: u32) -> Self {
        ExplorerItem {
            expanded: false,
            level: lvl,
        }
    }
}

impl EventHandler for ExplorerItem {
    fn build<'a>(self, state: &'a mut State, parent: Entity) -> Builder<'a>
    where
        Self: std::marker::Sized + 'static,
    {
        let id = state.add(parent);

        id
            //.set_background_color(state, Color::rgb(100, 50, 50))
            .set_display(state, Display::Flexbox);

        // Button2::new("arrow")
        //     .on_press(Event::message(ExplorerMessage::ToggleDirectory(id)))
        //     .build(state, id, event_manager)
        //     .selector(Selector::from("button").class("arrow"))
        //     .set_flex_basis(30.0 * self.level as f32);

        Checkbox::new()
            .on_press(Event::new(ExplorerMessage::ToggleDirectory(id)))
            .build(state, id)
            .element("button")
            .class("arrow")
            //.selector(Selector::from("button").class("arrow"))
            .set_flex_basis(30.0 * self.level as f32);
        //.set_margin_left(30.0 * self.level as f32);

        Button::new()
            .on_press(Event::new(ExplorerMessage::AddDirectory(id, self.level)))
            .build(state, id)
            .element("button")
            .class("icon");
        //.selector(Selector::from("button").class("icon"));

        Button::new()
            .on_press(Event::new(ExplorerMessage::ViewDirectory))
            .build(state, id)
            .element("button")
            .class("text")
            //.selector(Selector::from("button").class("text"))
            .entity()
            .set_text(state, &id.index().to_string());

        state.build(id, self)
        //widget_list.insert(id, self);

        //id.state(state);
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &Event) -> bool {
        match event.message.downcast::<ExplorerMessage>() {
            Some(explorer_message) => {
                match explorer_message {
                    ExplorerMessage::ToggleDirectory(_) => {
                        //println!("ToggleDirector: {:?}", entity);
                        // if *entity == id {
                        //     if self.expanded == true {
                        //         self.expanded = false;
                        //     } else {
                        //         self.expanded = true;
                        //     }
                        // }
                    }

                    _ => {}
                }
            }

            _ => {}
        }

        /*
        match &event.event_type {
            EventType::Message(ref message) => {
                match message.downcast::<ExplorerMessage>() {
                    Some(explorer_message) => {
                        match explorer_message {
                            ExplorerMessage::ToggleDirectory(entity) => {
                                //println!("ToggleDirector: {:?}", entity);
                                // if *entity == id {
                                //     if self.expanded == true {
                                //         self.expanded = false;
                                //     } else {
                                //         self.expanded = true;
                                //     }
                                // }
                            }

                            _ => {}
                        }
                    }

                    _ => {}
                }
            }

            _ => {}
        }
        */

        false
    }
}

#[derive(Clone)]
pub struct Entry {
    pub name: String,
    pub expanded: bool,
}

#[derive(Clone)]
pub struct Explorer {
    hierarchy: Hierarchy,
    pub directories: HashMap<Entity, Entry>,
}

impl Explorer {
    pub fn new() -> Self {
        Explorer {
            hierarchy: Hierarchy::new(),
            directories: HashMap::new(),
        }
    }
}

impl EventHandler for Explorer {
    fn build<'a>(mut self, state: &'a mut State, parent: Entity) -> Builder<'a>
    where
        Self: std::marker::Sized + 'static,
    {
        let id = state.add(parent);

        id.set_display(state, Display::Flexbox)
            .set_flex_direction(state, FlexDirection::Column);

        let root = ExplorerItem::new(1)
            .build(state, id)
            .class("explorer_item")
            //.selector(Selector::new().class("explorer_item"))
            .entity();

        println!("Add root: {:?}", root);

        self.hierarchy.add(root, None);
        self.directories.insert(
            root,
            Entry {
                name: "test".to_string(),
                expanded: true,
            },
        );

        state.build(id, self)
        //id.state(state);
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &Event) -> bool {
        match event.message.downcast::<ExplorerMessage>() {
            Some(explorer_message) => match explorer_message {
                ExplorerMessage::ToggleDirectory(directory) => {
                    if let Some(entry) = self.directories.get_mut(directory) {
                        if entry.expanded {
                            println!("ToggleDirector: {:?}", directory);
                            entry.expanded = false;
                        } else {
                            entry.expanded = true;
                        }
                    }

                    for item in directory.into_iter(&self.hierarchy) {
                        if let Some(parent) = self.hierarchy.get_parent(item) {
                            if let Some(entry) = self.directories.get(&parent) {
                                if entry.expanded
                                    && state.transform.get_visibility(parent) == Visibility::Visible
                                {
                                    state.transform.set_visibility(item, Visibility::Visible);
                                } else if !entry.expanded
                                    || state.transform.get_visibility(parent)
                                        == Visibility::Invisible
                                {
                                    state.transform.set_visibility(item, Visibility::Invisible);
                                }
                            }
                        }
                    }
                }

                ExplorerMessage::ViewDirectory => {
                    println!("View Directory");
                }

                ExplorerMessage::AddDirectory(entity, level) => {
                    let child = ExplorerItem::new(level + 1)
                        .build(state, id)
                        .with_sibling(*entity)
                        .class("explorer_item")
                        //.selector(Selector::new().class("explorer_item"))
                        .entity();
                    self.hierarchy.add(child, Some(*entity));
                    self.directories.insert(
                        child,
                        Entry {
                            name: "test".to_string(),
                            expanded: true,
                        },
                    );

                    println!("Add Directory: {:?}", entity);
                }
            },

            _ => {}
        }

        /*
        match &event.event_type {
            EventType::Message(ref message) => match message.downcast::<ExplorerMessage>() {
                Some(explorer_message) => match explorer_message {
                    ExplorerMessage::ToggleDirectory(directory) => {
                        if let Some(entry) = self.directories.get_mut(directory) {
                            if entry.expanded {
                                println!("ToggleDirector: {:?}", directory);
                                entry.expanded = false;
                            } else {
                                entry.expanded = true;
                            }
                        }

                        for item in directory.into_iter(&self.hierarchy) {
                            if let Some(parent) = self.hierarchy.get_parent(item) {
                                if let Some(entry) = self.directories.get(&parent) {
                                    if entry.expanded
                                        && state.transform.get_visibility(parent)
                                            == Visibility::Visible
                                    {
                                        state.transform.set_visibility(item, Visibility::Visible);
                                    } else if !entry.expanded
                                        || state.transform.get_visibility(parent)
                                            == Visibility::Invisible
                                    {
                                        state.transform.set_visibility(item, Visibility::Invisible);
                                    }
                                }
                            }
                        }
                    }

                    ExplorerMessage::ViewDirectory => {
                        println!("View Directory");
                    }

                    ExplorerMessage::AddDirectory(entity, level) => {
                        let child = ExplorerItem::new(level + 1)
                            .build(state, id, event_manager)
                            .with_sibling(*entity)
                            .selector(Selector::new().class("explorer_item"))
                            .entity();
                        self.hierarchy.add(child, Some(*entity));
                        self.directories.insert(
                            child,
                            Entry {
                                name: "test".to_string(),
                                expanded: true,
                            },
                        );

                        println!("Add Directory: {:?}", entity);
                    }

                    _ => {}
                },

                _ => {}
            },

            _ => {}
        }
        */

        false
    }
}

/*
pub struct ExplorerEventHandler {
    hierarchy: Hierarchy,
    items: Vec<(Entity, bool)>,
    explorer: Entity,
}

impl ExplorerEventHandler {
    pub fn new(widget: Entity) -> Self {
        let mut hierarchy = Hierarchy::new();
        hierarchy.add(widget, None);
        ExplorerEventHandler {
            hierarchy: hierarchy,
            items: Vec::new(),
            explorer: widget,
        }
    }

    pub fn add(&mut self, gui: &mut WidgetSystem, parent: Entity) -> Entity {
        let folder = gui.add(Some(self.explorer)).unwrap();
        self.hierarchy.add(folder, Some(parent));
        gui.state.transform.set_local_height(folder, 30.0);
        gui.state.style.flex_item.get_mut(folder).unwrap().flex_grow = 0.0;
        gui.state
            .style
            .flex_item
            .get_mut(folder)
            .unwrap()
            .flex_basis = 0.0;
        self.items.push((folder, true));
        return folder;
    }
}

impl EventHandler for ExplorerEventHandler {
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
                                for item in &mut self.items {
                                    if state.hovered == item.0 {
                                        if item.1 == false {
                                            item.1 = true;
                                        } else {
                                            item.1 = false;
                                        }
                                    }
                                }

                                // for item in &self.items {
                                //     if item.0 == state.hovered {
                                //         match item.1 {
                                //             true => {
                                //                 for child in item.0.into_iter(&self.hierarchy) {
                                //                     state.style.visibility.set(child, Visibility::Visible);
                                //                 }
                                //             }

                                //             false => {
                                //                 for child in item.0.into_iter(&self.hierarchy) {
                                //                     state.style.visibility.set(child, Visibility::Invisible);
                                //                 }
                                //             }
                                //         }
                                //     }
                                // }

                                for widget in self.hierarchy.into_iter() {
                                    println!("WIDGET: {:?}", widget);
                                    println!("{:?}", self.hierarchy.parent);
                                    for item in &self.items {
                                        if widget == item.0 {
                                            match item.1 {
                                                true => {
                                                    for child in item.0.child_iter(&self.hierarchy)
                                                    {
                                                        state
                                                            .style
                                                            .visibility
                                                            .set(child, Visibility::Visible);
                                                    }
                                                }

                                                false => {
                                                    for child in item.0.child_iter(&self.hierarchy)
                                                    {
                                                        state
                                                            .style
                                                            .visibility
                                                            .set(child, Visibility::Invisible);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }

                                for widget in self.hierarchy.into_iter() {
                                    let vis = state.style.visibility.get(widget);
                                    if vis == Visibility::Invisible {
                                        for child in widget.child_iter(&self.hierarchy) {
                                            state
                                                .style
                                                .visibility
                                                .set(child, Visibility::Invisible);
                                        }
                                    }
                                }
                            }

                            MouseButtonState::Released => {}
                        }
                    }

                    _ => {}
                }
            }

            _ => {}
        }
    }

    fn get_entity(&self) -> Entity {
        self.explorer
    }
}
*/
