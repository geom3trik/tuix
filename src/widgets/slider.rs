#![allow(dead_code)]

use crate::entity::Entity;
use crate::mouse::*;
use crate::{BuildHandler, Event, EventHandler, Length, WindowEvent};
use crate::{PropSet, State};

use crate::state::style::*;

use crate::widgets::Button;

#[derive(Debug, Clone, PartialEq)]
pub enum SliderEvent {
    ValueChanged(Entity, f32),
    SetValue(Entity, f32),
}

//impl Message for SliderEvent {}

#[derive(Clone)]
pub struct Slider {
    front: Entity,
    on_press: Option<Event>,
    value: f32,
    temp: f32,
    sliding: bool,
    pressed_x: f32,
}

impl Slider {
    pub fn new() -> Self {
        Slider {
            front: Entity::null(),
            on_press: None,
            value: 0.5,
            temp: 0.5,
            sliding: false,
            pressed_x: 0.0,
        }
    }

    pub fn on_press(mut self, message: Event) -> Self {
        self.on_press = Some(message);
        self
    }
}

impl BuildHandler for Slider {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity.set_flex_direction(state, FlexDirection::Row);

        self.front = Button::new().build(state, entity, |builder| {
            builder.set_width(Length::Percentage(0.5)).class("front")
        });

        state.style.insert_element(entity, "slider");

        entity
    }
}

impl EventHandler for Slider {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) -> bool {
        if let Some(slider_event) = event.message.downcast::<SliderEvent>() {
            match slider_event {
                SliderEvent::SetValue(id, val) => {
                    if *id == entity {
                        let mut val = *val;

                        if val <= 0.0 {
                            val = 0.0;
                        }
                        if val >= 1.0 {
                            val = 1.0;
                        }

                        self.value = val;
                        self.temp = val;

                        self.front.set_width(state, Length::Percentage(self.value));

                        state.insert_event(
                            Event::new(WindowEvent::Restyle).target(Entity::new(0, 0)),
                        );
                    }
                }

                _ => {}
            }
        }

        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::MouseDown(button) => match button {
                    MouseButton::Left => {
                        if entity == event.target || self.front == event.target {
                            self.sliding = true;
                            self.pressed_x = state.mouse.cursorx;
                            //state.captured = entity;
                            state.capture(entity);
                            state.focused = entity;

                            let dx = (self.pressed_x - state.transform.get_posx(entity))
                                / state.transform.get_width(entity);

                            let mut v = dx;

                            if v <= 0.0 {
                                v = 0.0;
                            }
                            if v >= 1.0 {
                                v = 1.0;
                            }

                            self.value = (v * 1000.0).round() / 1000.0;
                            self.temp = (v * 1000.0).round() / 1000.0;

                            self.front.set_width(state, Length::Percentage(self.value));

                            state.insert_event(
                                Event::new(SliderEvent::SetValue(entity, self.value))
                                    .target(entity),
                            );

                            // state.insert_event(
                            //     Event::new(StyleEvent::Restyle)
                            //         .target(Entity::new(0, 0)),
                            // );

                            state.insert_event(
                                Event::new(SliderEvent::ValueChanged(entity, self.value))
                                    .target(entity),
                            );
                        }
                    }

                    _ => {}
                },

                WindowEvent::MouseUp(button) => match button {
                    MouseButton::Left => {
                        //println!("Not Sliding");
                        self.temp = self.value;
                        self.sliding = false;
                        //state.captured = Entity::null();
                        state.release(entity);
                        state.insert_event(
                            Event::new(WindowEvent::Restyle).target(Entity::new(0, 0)),
                        );
                    }

                    _ => {}
                },

                // Slider needs to capture mouse events
                WindowEvent::MouseMove(x, _) => {
                    //println!("Mouse Move");
                    if self.sliding {
                        //let dx = self.pressed_x - x;
                        let dx = (*x - state.transform.get_posx(entity))
                            / state.transform.get_width(entity);
                        //let mut v = self.temp - dx * 0.01;
                        let mut v = dx;

                        if v <= 0.0 {
                            v = 0.0;
                        }
                        if v >= 1.0 {
                            v = 1.0;
                        }

                        self.value = (v * 1000.0).round() / 1000.0;

                        //let back_width = state.transform.get_width(entity);

                        //println!("{}", back_width);
                        self.front.set_width(state, Length::Percentage(self.value));

                        state.insert_event(
                            Event::new(SliderEvent::ValueChanged(entity, self.value))
                                .target(entity),
                        );

                        state.insert_event(
                            Event::new(WindowEvent::Restyle).target(Entity::new(0, 0)),
                        );
                        //self.textbox.set_text(state, v.to_string());
                    }
                }

                WindowEvent::MouseScroll(_, y) => {
                    if event.target == entity || self.front == event.target {
                        self.value += *y * 0.1;

                        if self.value <= 0.0 {
                            self.value = 0.0;
                        }
                        if self.value >= 1.0 {
                            self.value = 1.0;
                        }

                        self.front.set_width(state, Length::Percentage(self.value));

                        state.insert_event(
                            Event::new(SliderEvent::ValueChanged(entity, self.value))
                                .target(entity),
                        );

                        state.insert_event(
                            Event::new(WindowEvent::Restyle).target(Entity::new(0, 0)),
                        );
                    }
                }

                _ => {}
            }
        }

        false
    }
}

// #![allow(dead_code)]

// use crate::component::storage::Storage;
// use crate::component::style::text::*;
// use crate::entity::Entity;
// use crate::events::*;
// use crate::mouse::*;
// use crate::widget::Widget;
// use crate::State;

// pub struct Slider {
//     front: Entity,
//     pub back: Entity,
//     textbox: Entity,

//     pressed_x: f32,
//     sliding: bool,

//     value: f32,
//     temp: f32,
// }

// impl Slider {
//     pub fn new(state: &mut State, widget_list: &mut WidgetList, parent: Entity) -> Self {
//         let back = state.add(parent);
//         let front = state.add(back);

//         back.set_width(state, 100.0)
//             .set_height(state, 30.0)
//             .set_background_color(state, nanovg::Color::from_rgb(46, 46, 46));
//         front
//             .set_width(state, 50.0)
//             .set_height(state, 1.0)
//             .set_background_color(state, nanovg::Color::from_rgb(73, 73, 73));

//         let textbox = state.add(back);

//         textbox
//             .set_width(state, 1.0)
//             .set_height(state, 1.0)
//             .set_background_color(state, nanovg::Color::from_rgba(0, 0, 0, 0))
//             .set_text_horizontal_align(state, HorizontalAlign::Center)
//             .set_text(state, "0.5".to_string());

//         Slider {
//             front: front,
//             back: back,

//             textbox: textbox,

//             pressed_x: 0.0,
//             sliding: false,

//             value: 0.5,
//             temp: 0.5,
//         }
//     }

//     pub fn get_entity(&self) -> Entity {
//         self.back
//     }
// }

// impl EventHandler for Slider {
//     fn handle_event(
//         &mut self,
//         state: &mut State,
//         event: &WidgetEvent,
//         event_handlers: &mut Vec<Box<EventHandler>>,
//         event_queue: &mut EventQueue,
//     ) {
//         match event {
//             WidgetEvent::MouseButton(button, action, mods) => match button {
//                 MouseButton::Left => match action {
//                     MouseButtonState::Pressed => {
//                         if state.hovered == self.front
//                             || state.hovered == self.back
//                             || state.hovered == self.textbox
//                         {
//                             println!("Slider Pressed");
//                             self.sliding = true;
//                             self.pressed_x = state.mouse.cursorx;
//                         }
//                     }

//                     MouseButtonState::Released => {
//                         self.sliding = false;
//                         self.temp = self.value;
//                     }
//                 },

//                 _ => {}
//             },

//             WidgetEvent::MouseMotion(x, y) => {
//                 if self.sliding {
//                     let dx = self.pressed_x - x;
//                     let mut v = self.temp - dx * 0.01;

//                     if v <= 0.0 {
//                         v = 0.0;
//                     }
//                     if v >= 1.0 {
//                         v = 1.0;
//                     }

//                     self.value = v;

//                     let back_width = state.transform.get_width(self.back);

//                     self.front.set_width(state, self.value);
//                     self.textbox.set_text(state, v.to_string());
//                 }
//             }

//             _ => {}
//         }
//     }

//     fn get_entity(&self) -> Entity {
//         self.back
//     }
// }
