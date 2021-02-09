#![allow(dead_code)]

use crate::entity::Entity;
use crate::mouse::*;
use crate::{BuildHandler, Event, EventHandler, Length, WindowEvent};
use crate::{PropSet, State};

use crate::state::style::*;

use crate::widgets::{Button, Element};

use crate::event::Message;

#[derive(Debug, Clone, PartialEq)]
pub enum SliderEvent {
    ValueChanged(f32),
    SetValue(f32),
}

pub struct ProgressBar {
    front: Entity,
    on_change: Option<Box<dyn Fn(f32) -> Event + Send>>,
    value: f32,
    temp: f32,
    sliding: bool,
    pressed_x: f32,
}

impl ProgressBar {
    pub fn new() -> Self {
        Self {
            front: Entity::null(),
            on_change: None,
            value: 0.5,
            temp: 0.5,
            sliding: false,
            pressed_x: 0.0,
        }
    }

    pub fn on_change<F>(mut self, message: F) -> Self
    where
        F: 'static + Send + Fn(f32) -> Event,
    {
        self.on_change = Some(Box::new(message));
        self
    }
}

impl BuildHandler for ProgressBar {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity.set_flex_direction(state, FlexDirection::Row);

        self.front = Element::new().build(state, entity, |builder| {
            builder.set_width(Length::Percentage(0.5)).class("front")
        });

        state.style.insert_element(entity, "progress_bar");

        entity
    }
}

impl EventHandler for ProgressBar {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(slider_event) = event.message.downcast::<SliderEvent>() {
            match slider_event {
                SliderEvent::SetValue(val) => {
                    if event.target == entity {
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

                        // state.insert_event(
                        //     Event::new(WindowEvent::Restyle).target(Entity::new(0, 0)),
                        // );
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

                            let dx = (self.pressed_x - state.data.get_posx(entity))
                                / state.data.get_width(entity);

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
                                Event::new(SliderEvent::SetValue(self.value)).target(entity),
                            );

                            // state.insert_event(
                            //     Event::new(StyleEvent::Restyle)
                            //         .target(Entity::new(0, 0)),
                            // );

                            state.insert_event(
                                Event::new(SliderEvent::ValueChanged(self.value)).target(entity),
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
                        // state.insert_event(
                        //     Event::new(WindowEvent::Restyle).target(Entity::new(0, 0)),
                        // );
                    }

                    _ => {}
                },

                // Slider needs to capture mouse events
                WindowEvent::MouseMove(x, _) => {
                    //println!("Mouse Move");
                    if self.sliding {
                        //let dx = self.pressed_x - x;
                        let dx = (*x - state.data.get_posx(entity))
                            / state.data.get_width(entity);
                        //let mut v = self.temp - dx * 0.01;
                        let mut v = dx;

                        if v <= 0.0 {
                            v = 0.0;
                        }
                        if v >= 1.0 {
                            v = 1.0;
                        }

                        self.value = (v * 1000.0).round() / 1000.0;

                        //let back_width = state.data.get_width(entity);

                        //println!("{}", back_width);
                        self.front.set_width(state, Length::Percentage(self.value));

                        state.insert_event(
                            Event::new(SliderEvent::ValueChanged(self.value)).target(entity),
                        );

                        // state.insert_event(
                        //     Event::new(WindowEvent::Restyle).target(Entity::new(0, 0)),
                        // );
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
                            Event::new(SliderEvent::ValueChanged(self.value)).target(entity),
                        );

                        // state.insert_event(
                        //     Event::new(WindowEvent::Restyle).target(Entity::new(0, 0)),
                        // );
                    }
                }

                _ => {}
            }
        }
    }
}

pub struct Slider {
    thumb: Entity,
    active: Entity,
    sliding: bool,
    on_change: Option<Box<dyn Fn(f32) -> Event + Send>>,

    value: f32,

    min: f32,
    max: f32,
    div: f32,
}

impl Slider {
    pub fn new() -> Self
    {
        Self {
            thumb: Entity::null(),
            active: Entity::null(),
            sliding: false,
            on_change: None,

            min: 0.0,
            max: 1.0,
            div: 0.0,
            value: 0.0,
        }
    }

    pub fn on_change<F>(mut self, message: F) -> Self
    where F: 'static + Fn(f32) -> Event + Send
    {
        self.on_change = Some(Box::new(message));
        self
    }

    pub fn with_initial_value(mut self, val: f32) -> Self {
        self.value = val;

        self
    }

    pub fn with_min(mut self, val: f32) -> Self {
        self.min = val;
        self
    }

    pub fn with_max(mut self, val: f32) -> Self {
        self.max = val;
        self
    }

    pub fn with_divisions(mut self, val: f32) -> Self {
        self.div = val;
        self
    }
}

impl BuildHandler for Slider {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
            .set_width(state, Length::Pixels(100.0))
            .set_height(state, Length::Pixels(4.0));
        //.set_align_items(state, AlignItems::Center)
        //.set_background_color(state, Color::rgb(200, 80, 80));

        self.active = Element::new().build(state, entity, |builder| {
            builder
                .set_width(Length::Percentage(0.0))
                .set_height(Length::Percentage(1.0))
                //.set_background_color(Color::rgb(60, 60, 200))
                .set_hoverability(false)
                .class("active")
        });

        self.thumb = Element::new().build(
            state,
            entity,
            |builder| {
                builder
                    .set_position(Position::Absolute)
                    .set_top(Length::Pixels(-8.0))
                    .set_width(Length::Pixels(20.0))
                    .set_height(Length::Pixels(20.0))
                    .class("thumb")
            }, //.set_background_color(Color::rgb(80, 80, 200))
        );
        
        // TEMP
        self.thumb.set_left(state, Length::Pixels(80.0));
        self.active.set_width(state, Length::Percentage(1.0));

        state.style.insert_element(entity, "slider");

        entity
    }
}

impl EventHandler for Slider {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::MouseDown(button) => {
                    if *button == MouseButton::Left && event.target == entity
                        || event.target == self.thumb
                    {
                        self.sliding = true;
                        state.capture(entity);

                        let width = state.data.get_width(entity);
                        let thumb_width = state.data.get_width(self.thumb);

                        let mut dx =
                            (state.mouse.left.pos_down.0 - state.data.get_posx(entity));

                        if dx <= thumb_width / 2.0 {
                            dx = thumb_width / 2.0;
                        }
                        if dx >= width - thumb_width / 2.0 {
                            dx = width - thumb_width / 2.0;
                        }

                        let nx = (dx - thumb_width / 2.0) / (width - thumb_width);

                        let v = self.min + nx * (self.max - self.min);

                        self.active.set_width(state, Length::Percentage(nx));
                        self.thumb
                            .set_left(state, Length::Pixels(dx - thumb_width / 2.0));

                        if let Some(on_change) = &self.on_change {
                            let mut event = (on_change)(v);
                            event.origin = entity;

                            state.insert_event(event);                            
                        }

                        state.insert_event(Event::new(SliderEvent::ValueChanged(v)).target(entity));
                    }
                }

                WindowEvent::MouseUp(button) => {
                    if *button == MouseButton::Left {
                        self.sliding = false;
                        state.release(entity);
                    }
                }

                WindowEvent::MouseMove(x, _) => {
                    if self.sliding {
                        let width = state.data.get_width(entity);
                        let thumb_width = state.data.get_width(self.thumb);

                        let mut dx = *x - state.data.get_posx(entity);

                        if dx <= thumb_width / 2.0 {
                            dx = thumb_width / 2.0;
                        }
                        if dx >= width - thumb_width / 2.0 {
                            dx = width - thumb_width / 2.0;
                        }

                        // if dx <= 0.0 {
                        //     dx = 0.0;
                        // } if dx >= width - thumb_width {
                        //     dx = width - thumb_width;
                        // }

                        // let nx = (dx - thumb_width/2.0) / (width - thumb_width);
                        let nx = (dx - thumb_width / 2.0) / (width - thumb_width);

                        let v = self.min + nx * (self.max - self.min);

                        self.active.set_width(state, Length::Percentage(nx));
                        //self.thumb.set_left(state, Length::Pixels(dx - thumb_width/2.0));
                        self.thumb
                            .set_left(state, Length::Percentage((dx - thumb_width / 2.0) / width));
                        
                        if let Some(on_change) = &self.on_change {
                            let mut event = (on_change)(v);
                            event.origin = entity;

                            state.insert_event(event);                            
                        }


                        state.insert_event(
                            Event::new(SliderEvent::ValueChanged(v))
                                .target(entity),
                        );
                    }
                }

                _ => {}
            }
        }
    }
}
