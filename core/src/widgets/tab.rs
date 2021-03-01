#![allow(dead_code)]

use crate::{EventManager, HierarchyTree, Propagation, Radio, RadioButton, RadioList, Entity, State, CheckboxEvent};

use crate::events::{BuildHandler, Event, EventHandler};

use crate::widgets::Element;

use crate::{IntoChildIterator, WindowEvent};

use crate::state::style::*;

#[derive(Clone, Debug, PartialEq)]
pub enum TabEvent {
    SwitchTab(String),
}

pub struct TabBar {
    list: RadioList,
}

impl TabBar {
    pub fn new() -> Self {
        Self { 
            list: RadioList::new(),
        }
    }
}

impl BuildHandler for TabBar {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        self.list.on_build(state, entity);

        entity.set_flex_direction(state, FlexDirection::Row);

        state.style.insert_element(entity, "tab_bar");

        entity
    }
}

impl EventHandler for TabBar {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        self.list.on_event(state, entity, event);
    }
}

pub struct Tab {
    pub name: String,
    radio: Radio,
}

impl Tab {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            radio: Radio::new().on_checked(Event::new(TabEvent::SwitchTab(name.to_string())).propagate(Propagation::Up)),
        }
    }
}

impl BuildHandler for Tab {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        entity.set_element(state, "tab")
    }
}

impl EventHandler for Tab {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        self.radio.on_event(state, entity, event);

        if let Some(tab_event) = event.message.downcast::<TabEvent>() {
            match tab_event {
                TabEvent::SwitchTab(name) => {
                    if name == &self.name && event.origin != entity {
                        state.insert_event(
                            Event::new(CheckboxEvent::Checked)
                                .target(entity)
                                .origin(entity),
                        );
                    }
                }
            }
        }
    }
}



pub struct Tabs {
    pub tab_bar: Entity,
    pub container: Entity,
}

impl Tabs {
    pub fn new() -> Self {
        Self {
            tab_bar: Entity::default(),
            container: Entity::default(),
        }
    }
}

impl BuildHandler for Tabs {
    type Ret = (Entity, Entity);
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        self.tab_bar = TabBar::new().build(state, entity, |builder| builder);

        self.container = Element::new().build(state, entity, |builder| builder.class("container"));

        entity.set_element(state, "tabs");

        (self.tab_bar, self.container)
    }
}

impl EventHandler for Tabs {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(tab_event) = event.message.downcast::<TabEvent>() {
            match tab_event {
                TabEvent::SwitchTab(name) => {
                    if event.origin.is_descendant_of(&state.hierarchy, self.tab_bar) || event.target == self.tab_bar {
                        state.insert_event(Event::new(TabEvent::SwitchTab(name.clone())).target(entity).propagate(Propagation::Fall).origin(event.origin));
                    }          
                    
                    event.consume();
                }
                _=> {}
            }
        }
    }
}


pub struct TabContainer {
    pub name: String,
}

impl TabContainer {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl BuildHandler for TabContainer {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity.set_element(state, "tab_container")
    }
}

impl EventHandler for TabContainer {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(tab_event) = event.message.downcast::<TabEvent>() {
            match tab_event {
                TabEvent::SwitchTab(name) => {
                    if name == &self.name {
                        entity.set_display(state, Display::Flexbox);
                    } else {
                        entity.set_display(state, Display::None);
                    }
                }

                _=> {}
            }
        }
    }
}