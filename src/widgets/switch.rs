#![allow(dead_code)]

use crate::entity::Entity;
use crate::mouse::*;

use crate::{BuildHandler, Event, EventHandler, WindowEvent};
use crate::{PropSet, State};

use crate::state::style::*;

use crate::widgets::Element;

#[derive(Clone)]
pub struct Switch {
    front: Entity,
    //on_press: Option<Event>,
    checked: bool,
}

impl Switch {
    pub fn new(checked: bool) -> Self {
        Switch {
            front: Entity::null(),
            //on_press: None,
            checked,
        }
    }

    // pub fn on_press(mut self, message: Event) -> Self {
    //     self.on_press = Some(message);
    //     self
    // }
}

impl BuildHandler for Switch {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        self.front = Element::new().build(state, entity, |builder| {
            builder.class("front")
            //.set_height(Length::Percentage(1.0))
            //.set_width(Length::Percentage(0.5))
            //.set_left(Length::Pixels(0.0))
            //.set_top(Length::Pixels(0.0))
        });

        state.style.insert_element(entity, "switch");

        entity
    }
}

impl EventHandler for Switch {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) -> bool {
        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            //println!("received window event: {:?}", window_event);
            match window_event {
                WindowEvent::MouseUp(button) => match button {
                    MouseButton::Left => {
                        if entity == event.target || self.front == event.target {
                            if self.checked {
                                //self.front.set_justification(state, Justification::Start);
                                //self.front.set_left(state, Length::Percentage(0.0));
                                self.checked = false;
                                entity.set_checked(state, false);
                            } else {
                                //self.front.set_justification(state, Justification::End);
                                //self.front.set_left(state, Length::Percentage(0.5));
                                self.checked = true;
                                entity.set_checked(state, true);
                            }

                            // state.insert_event(
                            //     Event::new(WindowEvent::Restyle).target(Entity::new(0, 0)),
                            // );

                            // if let Some(mut on_press) = self.on_press.clone() {
                            //     on_press.target = entity;
                            //     state.insert_event(on_press);
                            // }

                            return true;
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
