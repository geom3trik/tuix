#![allow(dead_code)]

use crate::state::{Entity, State};

use crate::events::{BuildHandler, Event, EventHandler};

use crate::widgets::Element;

use crate::{IntoChildIterator, WindowEvent};

use crate::state::style::*;

#[derive(Clone, Debug, PartialEq)]
pub enum TabEvent {
    SwitchTab(usize),
}

//impl Message for TabEvent {}

pub struct TabBar {
    tabs: Vec<(Entity, Entity)>,
}

impl TabBar {
    pub fn new() -> Self {
        TabBar { tabs: Vec::new() }
    }

    // pub fn add_tab(&mut self, state: &mut State, name: &str, view: Entity) -> Entity {
    //     let tab = state.add(self.entity);
    //     tab.set_flex_basis(state, 100.0)
    //         .set_background_color(state, nanovg::Color::from_rgb(46, 46, 46))
    //         .set_flex_grow(state, 0.0)
    //         .set_text(state, name)
    //         .set_text_horizontal_align(state, HorizontalAlign::Center)
    //         .set_margin_right(state, 1.0)
    //         .set_margin_bottom(state, 1.0);

    //     state.style.visibility.set(view, Visibility::Invisible);

    //     for child in view.into_iter(&state.hierarchy) {
    //         //println!("Entity: {:?}", child);
    //         state.style.visibility.set(child, Visibility::Invisible);
    //     }

    //     self.tabs.push((tab, view));
    //     return tab;
    // }
}

impl BuildHandler for TabBar {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
            .set_display(state, Display::Flexbox)
            .set_flex_direction(state, FlexDirection::Row);

        // let first = Button::new()
        //     .on_press(Event::new(TabEvent::SwitchTab(0)))
        //     .build(state, entity, |builder| builder.class("tab"));
        // Button::new()
        //     .on_press(Event::new(TabEvent::SwitchTab(1)))
        //     .build(state, entity, |builder| builder.class("tab"));
        // Button::new()
        //     .on_press(Event::new(TabEvent::SwitchTab(2)))
        //     .build(state, entity, |builder| builder.class("tab"));
        // Button::new()
        //     .on_press(Event::new(TabEvent::SwitchTab(3)))
        //     .build(state, entity, |builder| builder.class("tab"));

        // state.style.checked.set(first, true);
        state.style.insert_element(entity, "tab_bar");

        entity
    }
}

impl EventHandler for TabBar {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) -> bool {
        if let Some(tab_event) = event.message.downcast::<TabEvent>() {
            match tab_event {
                TabEvent::SwitchTab(tab_index) => {
                    if let Some(tab) = state.hierarchy.get_child(entity, *tab_index) {
                        for c in entity.child_iter(&state.hierarchy.clone()) {
                            //state.style.checked.set(c, false);
                            c.set_checked(state, false);
                        }

                        //state.style.checked.set(child, true);
                        tab.set_checked(state, true);

                        state.insert_event(Event::new(WindowEvent::Restyle));
                        state.insert_event(Event::new(WindowEvent::Relayout));
                    }
                }
            }
        }

        false
    }
}

pub struct TabContainer {
    pub tab_bar: Entity,
    pub container: Entity,

    tabs: Vec<String>,
}

impl TabContainer {
    pub fn new() -> Self {
        TabContainer {
            tab_bar: Entity::null(),
            container: Entity::null(),
            tabs: Vec::new(),
        }
    }

    pub fn add_tab(mut self, name: &str) -> Self {
        self.tabs.push(name.to_string());

        self
    }
}

impl BuildHandler for TabContainer {
    type Ret = (Entity, Entity);
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        //entity.set_background_color(state, Color::rgb(50, 70, 90));

        self.tab_bar = TabBar::new().build(state, entity, |builder| {
            builder
                .set_flex_direction(FlexDirection::Row)
                .set_flex_basis(30.0)
                .class("tab_bar")
        });

        // for (i, tab) in self.tabs.iter().enumerate() {
        //     let t = Button::with_label(tab).on_press(Event::new(TabEvent::SwitchTab(i as u32))).build(state, self.tab_bar, |builder| builder.class("tab"));
        //     if i == 0 {
        //         //state.style.checked.set(t, true);
        //         t.set_checked(state, true);
        //     }

        // }

        self.container = Element::new().build(state, entity, |builder| builder.class("container"));

        state.style.insert_element(entity, "tab_container");

        (self.tab_bar, self.container)
    }
}

impl EventHandler for TabContainer {
    // fn on_build(&mut self, state: &mut State, entity: Entity) -> Entity {
    //     entity
    //         .set_display(state, Display::Flexbox)
    //         .set_flex_direction(state, FlexDirection::Column);
    //         //.set_flex_grow(state, 1.0);

    //     self.tab_bar = TabBar::new().build(state, entity, |builder| builder.set_flex_basis(30.0));

    //     for (i, tab) in self.tabs.iter().enumerate() {
    //         let t = Button::with_label(tab).on_press(Event::new(TabEvent::SwitchTab(i))).build(state, self.tab_bar, |builder| builder.class("tab"));
    //         if i == 0 {
    //             //state.style.checked.set(t, true);
    //             t.set_checked(state, true);
    //         }

    //     }

    //     self.container = Button::new().build(state, entity, |builder| {
    //         builder.set_flex_grow(1.0).set_padding_top(Length::Pixels(0.0))
    //     });

    //     state.style.insert_element(entity, "tab_container");

    //     self.container
    // }

    fn on_event(&mut self, state: &mut State, _entity: Entity, event: &mut Event) -> bool {
        if let Some(tab_event) = event.message.downcast::<TabEvent>() {
            match tab_event {
                TabEvent::SwitchTab(tab_index) => {
                    //println!("Tab: {}", tab);
                    if let Some(tab) = state.hierarchy.get_child(self.container, *tab_index) {
                        //println!("Child: {:?}", child);
                        for child in self.container.child_iter(&state.hierarchy) {
                            state.style.display.insert(child, Display::None);
                        }

                        tab.set_display(state, Display::Flexbox);
                    }
                }
            }
        }

        false
    }
}
