#![allow(dead_code)]


use crate::{State, Entity, BuildHandler, EventHandler, Event, WindowEvent, MouseButton};

use crate::widgets::{Button, Checkbox};

use crate::state::style::*;

const ICON_DOWN_OPEN_BIG: &str = "\u{e75c}";
const ICON_RIGHT_OPEN_BIG: &str = "\u{e75e}";

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PanelEvent {
    Open(Entity),
    Close(Entity),
}

pub struct Panel {
    header: Entity,
    container: Entity,
    checkbox: Entity,
    collapsed: bool,
    title: String,
}

impl Panel {
    pub fn new(title: &str) -> Self {
        Panel {
            header: Entity::null(),
            container: Entity::null(),
            checkbox: Entity::null(),
            title: title.to_string(),
            collapsed: false,
        }
    }
}

impl BuildHandler for Panel {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        self.header = Button::new().build(state, entity, |builder| {
            builder
                .set_flex_direction(FlexDirection::Row)
                .class("header")
        });

        self.checkbox = Checkbox::new(true, ICON_DOWN_OPEN_BIG, ICON_RIGHT_OPEN_BIG).build(state, self.header, |builder| {
            builder
                .set_width(Length::Pixels(20.0))
                .set_height(Length::Percentage(1.0))
                .set_hoverability(false)
        });        

        Button::new().build(state, self.header, |builder| builder.set_text(&self.title).set_flex_grow(1.0).set_hoverability(false));

        self.container = Button::new().build(state, entity, |builder| builder.class("container"));

        //state.style.checked.set(entity, true);
        entity.set_checked(state, true);

        state.style.insert_element(entity, "panel");

        self.container
    }
}

impl EventHandler for Panel {


    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) -> bool {
        if event.target == self.header {
            if let Some(window_event) = event.message.downcast::<WindowEvent>() {
                match window_event {
                    WindowEvent::MouseUp(button, mods) => {
                        if *button == MouseButton::Left {
                            if self.collapsed {
                                //self.container.set_visibility(state, Visibility::Visible);
                                self.checkbox.set_text(state, ICON_DOWN_OPEN_BIG);
                                self.collapsed = false;
                                //state.style.checked.set(entity, true);
                                entity.set_checked(state, true);
                                state.insert_event(Event::new(PanelEvent::Open(entity)).target(entity));
                            } else {
                                //self.container.set_visibility(state, Visibility::Invisible);
                                self.checkbox.set_text(state, ICON_RIGHT_OPEN_BIG);
                                self.collapsed = true;
                                //state.style.checked.set(entity, false);
                                entity.set_checked(state, false);
                                state.insert_event(Event::new(PanelEvent::Close(entity)).target(entity));
                            }
                        }

                        println!("Draw");
                        state.insert_event(Event::new(WindowEvent::Restyle));
                        //state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
                        state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
                        state.insert_event(Event::new(WindowEvent::Redraw));

                        return true;
                    }

                    _=> {}
                }
            }
        }

        false
    }
}