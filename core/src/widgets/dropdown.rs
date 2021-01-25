#![allow(dead_code)]

use crate::entity::Entity;
use crate::mouse::*;
use crate::{AnimationState, BuildHandler, Event, EventHandler, Propagation, WindowEvent};
use crate::{PropSet, State};

use crate::state::style::*;
use crate::widgets::{Button, Checkbox, CheckboxEvent, Element, HBox, Label, RadioList};

use crate::state::hierarchy::HierarchyTree;

const ICON_DOWN_OPEN: &str = "\u{e75c}";

const ICON_DOWN_DIR: &str = "\u{25be}";

#[derive(Debug, Clone, PartialEq)]
pub enum DropdownEvent {
    SetText(String, String),
}

pub struct Item {
    //checkbox: Entity,
    text: String,
    proxy: String,
    pressed: bool,
}

impl Item {
    pub fn new(txt: &str, proxy: &str) -> Self {
        Item {
            //checkbox: Entity::null(),
            text: txt.to_string(),
            proxy: proxy.to_string(),
            pressed: false,
        }
    }
}

impl BuildHandler for Item {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
            .set_flex_grow(state, 1.0)
            .set_text(state, &self.text)
            .class(state, "item");

        //self.checkbox = Checkbox::new(false).build(state, entity, |builder| builder.set_hoverability(false));
        // Element::new().build(state, entity, |builder| {
        //     builder.set_text(&self.text).set_flex_grow(1.0).set_hoverability(false)
        // });

        entity
    }
}

impl EventHandler for Item {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) -> bool {
        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::MouseDown(button) => {
                    if *button == MouseButton::Left {
                        if entity == event.target {
                            self.pressed = true;
                        }
                    }
                }

                WindowEvent::MouseUp(button) => {
                    if *button == MouseButton::Left {
                        if self.pressed {
                            self.pressed = false;
                            //self.checkbox.set_checked(state, true);
                            // state.insert_event(
                            //     Event::new(CheckboxEvent::Switch)
                            //         .target(self.checkbox)
                            //         .propagate(Propagation::Direct),
                            // );
                            state.insert_event(
                                Event::new(DropdownEvent::SetText(
                                    self.text.clone(),
                                    self.proxy.clone(),
                                ))
                                .target(entity)
                                .propagate(Propagation::Up),
                            );
                        }
                    }
                }

                _ => {}
            }
        }

        false
    }
}

pub struct Dropdown {
    container: Entity,
    header: Entity,
    label: Entity,
    //options: Vec<(Entity, String, String)>,
    text: String,

    open: bool,

    //other_container: Entity,
    expand_animation: usize,
    fade_in_animation: usize,

    collapse_animation: usize,
    fade_out_animation: usize,
    //container_height: f32,
}

impl Dropdown {
    pub fn new(text: &str) -> Self {
        Dropdown {
            container: Entity::null(),
            header: Entity::null(),
            label: Entity::null(),
            //options: Vec::new(),
            text: text.to_string(),
            open: false,
            //other_container: Entity::null(),
            expand_animation: std::usize::MAX,
            fade_in_animation: std::usize::MAX,
            collapse_animation: std::usize::MAX,
            fade_out_animation: std::usize::MAX,
            //container_height: 0.0,
        }
    }

    // pub fn add_item(mut self, name: &str, proxy: &str) -> Self {
    //     self.options.push((Entity::null(), name.to_string(), proxy.to_string()));

    //     // self.options.insert(name.to_string(), v: V)
    //     self
    // }
}

impl BuildHandler for Dropdown {
    type Ret = (Entity, Entity, Entity);
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        self.header = Element::new().build(state, entity, |builder| {
            builder
                //.set_background_color(Color::rgb(100,100,50))
                .set_flex_direction(FlexDirection::Row)
                .set_flex_grow(1.0)
                .class("header")
        });

        self.label = Label::new(&self.text).build(state, self.header, |builder| {
            builder
                //.set_background_color(Color::rgb(100,50,50))
                .set_hoverability(false)
                .set_flex_grow(1.0)
        });

        let icon = Element::new().build(state, self.header, |builder| {
            builder
                .set_hoverability(false)
                //.set_background_color(Color::rgb(100,100,100))
                .set_text(ICON_DOWN_DIR)
                .set_width(Length::Pixels(20.0))
                .set_text_justify(Justify::Center)
                .class("icon")
        });

        self.container = Element::new().build(state, entity, |builder| {
            builder
                .set_position(Position::Absolute)
                .set_top(Length::Percentage(1.0))
                //.set_width(Length::Percentage(1.0))
                //.set_height(Length::Pixels(0.0))
                .set_opacity(0.0)
                .set_z_order(1)
                .set_clip_widget(Entity::new(0, 0))
                //.set_visibility(Visibility::Invisible)
                //.set_background_color(Color::rgb(100, 50, 50))
                .class("container")
        });

        //self.other_container = Button::new().build(state, self.container, |builder| builder.set_flex_grow(1.0).set_opacity(0.0).class("other"));

        // self.container = RadioList::new("").build(state, entity, |builder| {
        //     builder
        //         .set_position(Position::Absolute)
        //         .set_top(Length::Percentage(1.0))
        //         //.set_flex_grow(1.0)
        //         //.set_width(Length::Percentage(1.0))
        //         //.set_height(Length::Pixels(0.0))
        //         .set_opacity(0.0)
        //         .set_z_order(1)
        //         .set_clip_widget(Entity::new(0, 0))
        //         //.set_background_color(Color::rgb(100, 50, 50))
        //         .class("container")
        // });

        // for (id, name, proxy) in self.options.iter_mut() {
        //     *id = Item::new(name, proxy).build(state, self.other_container, |builder| builder.set_flex_direction(FlexDirection::Row).class("item").class("other"));
        // }

        state.style.insert_element(entity, "dropdown");

        // let container_expand_animation = AnimationState::new()
        //     .with_duration(std::time::Duration::from_millis(100))
        //     .with_keyframe((0.0, Length::Pixels(0.0)))
        //     .with_keyframe((1.0, Length::Pixels(200.0)));

        // self.expand_animation = state.style.height.insert_animation(container_expand_animation);

        // let container_collapse_animation = AnimationState::new()
        //     .with_duration(std::time::Duration::from_millis(100))
        //     .with_delay(std::time::Duration::from_millis(100))
        //     .with_keyframe((0.0, Length::Pixels(200.0)))
        //     .with_keyframe((1.0, Length::Pixels(0.0)));

        // self.collapse_animation = state.style.height.insert_animation(container_collapse_animation);

        let container_fade_in_animation = AnimationState::new()
            .with_duration(std::time::Duration::from_millis(100))
            //.with_delay(std::time::Duration::from_millis(100))
            .with_keyframe((0.0, Opacity(0.0)))
            .with_keyframe((1.0, Opacity(1.0)));

        self.fade_in_animation = state
            .style
            .opacity
            .insert_animation(container_fade_in_animation);

        let container_fade_out_animation = AnimationState::new()
            .with_duration(std::time::Duration::from_millis(100))
            .with_keyframe((0.0, Opacity(1.0)))
            .with_keyframe((1.0, Opacity(0.0)));

        self.fade_out_animation = state
            .style
            .opacity
            .insert_animation(container_fade_out_animation);

        // (entity, self.header, self.container)

        (entity, self.header, self.container)
    }
}

impl EventHandler for Dropdown {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) -> bool {
        if let Some(dropdown_event) = event.message.downcast::<DropdownEvent>() {
            //if event.target == entity {
            match dropdown_event {
                DropdownEvent::SetText(text, proxy) => {
                    //println!("Set Text");
                    //Check here if it's an event from a child (TODO)
                    self.label.set_text(state, proxy);
                    //self.container.set_visibility(state, Visibility::Invisible);
                    self.open = false;
                    //state.style.height.play_animation(self.container, self.collapse_animation);
                    //state.style.opacity.play_animation(self.other_container, self.fade_out_animation);
                    // Temp until persistent animations work

                    //self.container.set_height(state, Length::Pixels(0.0));
                    //self.other_container.set_opacity(state, 0.0);
                    //println!("Dropedown release");
                    //state.release(entity);
                    //state.insert_event(Event::new(WindowEvent::Restyle));
                    //state.insert_event(Event::new(WindowEvent::Redraw));

                    //return true;
                }

                _ => {}
            }
            //}
        }

        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::MouseDown(button) => match button {
                    MouseButton::Left => {
                        if event.target == entity || event.target == self.header {
                            if state.hovered.is_child_of(&state.hierarchy, self.container) {
                                state.insert_event(
                                    Event::new(WindowEvent::MouseDown(*button))
                                        .target(state.hovered)
                                        .propagate(Propagation::Direct),
                                );
                            }

                            return true;
                        }
                    }
                    _ => {}
                },

                WindowEvent::MouseCaptureOutEvent => {
                    self.open = false;

                    self.header.set_disabled(state, true);

                    state
                        .style
                        .opacity
                        .play_animation(self.container, self.fade_out_animation);

                    self.container.set_opacity(state, 0.0);
                }

                WindowEvent::MouseCaptureEvent => {
                    self.open = true;

                    self.header.set_enabled(state, true);

                    state
                        .style
                        .opacity
                        .play_animation(self.container, self.fade_in_animation);

                    self.container.set_opacity(state, 1.0);
                    // Shouldn't need to do this but it's required for some reason. TODO: Investigate
                    self.container.set_z_order(state, 1);
                }

                WindowEvent::MouseUp(button) => match button {
                    MouseButton::Left => {
                        if event.target == entity || event.target == self.header {
                            if state.mouse.left.pressed == state.hovered {
                                if !self.open {
                                    state.capture(entity);
                                } else {
                                    state.release(entity);
                                }

                                state.insert_event(
                                    Event::new(WindowEvent::MouseUp(*button))
                                        .target(state.hovered)
                                        .propagate(Propagation::Direct),
                                );

                                return true;
                            }
                        }
                    }

                    _ => {}
                },

                _ => {}
            }
        }

        false
    }
}
