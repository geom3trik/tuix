#![allow(dead_code)]

use crate::state::style::*;
use crate::widgets::*;

const ICON_DOWN_OPEN: &str = "\u{e75c}";

const ICON_DOWN_DIR: &str = "\u{25be}";

#[derive(Debug, Clone, PartialEq)]
pub enum DropdownEvent {
    SetText(String),
}

pub struct DropdownItem {
    //checkbox: Entity,
    text: String,
    proxy: String,
    pressed: bool,
}

impl DropdownItem {
    pub fn new(txt: &str, proxy: &str) -> Self {
        DropdownItem {
            //checkbox: Entity::null(),
            text: txt.to_string(),
            proxy: proxy.to_string(),
            pressed: false,
        }
    }
}

impl Widget for DropdownItem {
    type Ret = Entity;
    type Data = ();
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity.set_text(state, &self.text).class(state, "item");

        //self.checkbox = Checkbox::new(false).build(state, entity, |builder| builder.set_hoverable(false));
        // Element::new().build(state, entity, |builder| {
        //     builder.set_text(&self.text).set_flex_grow(1.0).set_hoverable(false)
        // });

        entity
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
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
                                Event::new(DropdownEvent::SetText(self.text.clone()))
                                    .target(entity)
                                    .propagate(Propagation::Up),
                            );
                        }
                    }
                }

                _ => {}
            }
        }
    }
}

pub struct Dropdown {
    button: CheckButton,

    pub container: Entity,
    pub header: Entity,
    pub label: Entity,

    text: String,
}

impl Dropdown {
    pub fn new(text: &str) -> Self {
        Dropdown {
            button: CheckButton::default(),
            container: Entity::null(),
            header: Entity::null(),
            label: Entity::null(),
            text: text.to_string(),
        }
    }
}

impl Widget for Dropdown {
    type Ret = Entity;
    type Data = ();
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        // self.header = Element::new().build(state, entity, |builder| {
        //     builder
        //         //.set_background_color(Color::rgb(100,100,50))
        //         .set_hoverable(false)
        //         .set_focusable(false)
        //         .set_layout_type(LayoutType::Row)
        //         .set_width(Stretch(1.0))
        //         .set_height(Stretch(1.0))
        //         .class("header")
        // });

        self.label = Label::new(&self.text).build(state, entity, |builder| {
            builder
                //.set_background_color(Color::rgb(100,50,50))
                .set_hoverable(false)
                .set_focusable(false)
                .set_width(Stretch(1.0))
                .set_height(Stretch(1.0))
                .class("label")
        });

        // Icon
        Element::new().build(state, entity, |builder| {
            builder
                .set_font("icons")
                .set_hoverable(false)
                .set_focusable(false)
                //.set_background_color(Color::rgb(100,100,100))
                .set_text(ICON_DOWN_DIR)
                .set_width(Pixels(20.0))
                .set_height(Pixels(20.0))
                .set_top(Stretch(1.0))
                .set_bottom(Stretch(1.0))
                .set_child_space(Stretch(1.0))
                .class("icon")
        });

        self.container = Popup::new().build(state, entity, |builder| {
            builder
                .set_position_type(PositionType::SelfDirected)
                .set_top(Percentage(100.0))
                .set_width(Auto)
                .set_height(Auto)
                .set_z_order(1)
                .set_clip_widget(Entity::root())
                .class("container")
        });

        let list = List::new().build(state, self.container, |builder| 
            builder
                .set_height(Auto)
        );

        entity.set_element(state, "dropdown").set_layout_type(state, LayoutType::Row);

        // (entity, self.header, self.container)

        let container = self.container;
        self.button = CheckButton::new().on_checked(move |_, state, entity|
            state.insert_event(
                Event::new(PopupEvent::Open).target(container)
            )
        )
        .on_unchecked(move |_, state, entity|
            state.insert_event(
                Event::new(PopupEvent::Close).target(container)
            )
        );

        list
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        self.button.on_event(state, entity, event);

        if let Some(dropdown_event) = event.message.downcast::<DropdownEvent>() {
            //if event.target == entity {
            match dropdown_event {
                DropdownEvent::SetText(text) => {
                    self.label.set_text(state, text);
                }
            }
        }

        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                // WindowEvent::MouseDown(button) => match button {
                //     MouseButton::Left => {
                //         if event.target == entity || event.target == self.header {

                //         }
                //     }
                //     _ => {}
                // },

                // WindowEvent::MouseCaptureOutEvent => {

                //     self.open = false;

                //     self.header.set_disabled(state, true);

                //     state
                //         .style
                //         .opacity
                //         .play_animation(self.container, self.fade_out_animation);

                //     self.container.set_opacity(state, 0.0);
                // }

                // WindowEvent::MouseCaptureEvent => {
                //     self.open = true;

                //     self.header.set_enabled(state, true);

                //     state
                //         .style
                //         .opacity
                //         .play_animation(self.container, self.fade_in_animation);

                //     self.container.set_opacity(state, 1.0);
                //     // Shouldn't need to do this but it's required for some reason. TODO: Investigate
                //     self.container.set_z_order(state, 1);
                // }

                // WindowEvent::MouseUp(button) => match button {
                //     MouseButton::Left => {
                //         if (event.target == entity || event.target == self.header)
                //             && event.origin != entity
                //         {
                //             if state.mouse.left.pressed == state.hovered {
                //                 if !self.open {
                //                     state.capture(entity);
                //                 } else {
                //                     state.release(entity);
                //                 }

                //                 state.insert_event(
                //                     Event::new(WindowEvent::MouseUp(*button))
                //                         .target(state.hovered)
                //                         .origin(entity)
                //                         .propagate(Propagation::Direct),
                //                 );
                //             }
                //         }
                //     }

                //     _ => {}
                // },
                // WindowEvent::KeyDown(code, key) => match code {
                //     Code::Escape => {
                //         state.insert_event(
                //             Event::new(WindowEvent::KeyDown(*code, key.clone()))
                //                 .target(self.container)
                //                 .propagate(Propagation::Direct),
                //         );
                //     }
                //     _ => {}
                // },

                _ => {}
            }
        }
    }
}
