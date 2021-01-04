#![allow(dead_code)]

use crate::entity::Entity;
use crate::mouse::*;
use crate::State;
use crate::{BuildHandler, Event, EventHandler, WindowEvent};

use crate::style::{Display, Visibility};

use crate::widgets::slider::SliderEvent;
use crate::widgets::Element;

use femtovg::{
    renderer::OpenGl, Baseline, Canvas, Color, FillRule, FontId, ImageFlags, ImageId, LineCap,
    LineJoin, Paint, Path, Renderer, Solidity,
};

pub struct ControlKnob {
    sliding: bool, // Could replace this with a bool in state, maybe in mouse
    value: f32,
    temp: f32,

    mouse_down_posy: f32,
    shift_pressed: bool,

    back: Entity,
    slider: Entity,
    tick: Entity,

    min_value: f32,
    max_value: f32,

    is_log: bool,
}

impl ControlKnob {
    pub fn new(init: f32, min: f32, max: f32) -> Self {
        ControlKnob {
            sliding: false,
            value: init,
            temp: init,

            mouse_down_posy: 0.0,
            shift_pressed: false,

            back: Entity::null(),
            slider: Entity::null(),
            tick: Entity::null(),

            min_value: min,
            max_value: max,

            is_log: false,
        }
    }

    pub fn with_log_scale(mut self) -> Self {

        self.is_log = true;

        self
    }
}

impl BuildHandler for ControlKnob {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        self.back = Element::new().build(state, entity, |builder| {
            builder
                .set_hoverability(false)
                .set_display(Display::None)
                .class("back")
        });
        self.slider = Element::new().build(state, entity, |builder| {
            builder
                .set_hoverability(false)
                .set_display(Display::None)
                .class("slider")
        });
        self.tick = Element::new().build(state, entity, |builder| {
            builder
                .set_hoverability(false)
                .set_display(Display::None)
                .class("tick")
        });

        state.style.insert_element(entity, "knob");

        entity
    }
}

impl EventHandler for ControlKnob {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) -> bool {
        if let Some(slider_event) = event.message.downcast::<SliderEvent>() {
            match slider_event {
                SliderEvent::SetValue(val) => {
                    if event.target == entity {
                        if event.target == entity {
                            self.value = ((*val).min(self.max_value)).max(self.min_value);

                            state.insert_event(
                                Event::new(WindowEvent::Redraw).target(Entity::new(0, 0)),
                            );
                        }
                    }
                }

                _ => {}
            }
        }

        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::MouseDown(button) => {
                    if event.target == entity && *button == MouseButton::Left {
                        self.sliding = true;
                        self.mouse_down_posy = state.mouse.left.pos_down.1;
                        state.capture(entity);
                        state.focused = entity;
                        self.temp = self.value;
                    }
                }

                WindowEvent::MouseUp(button) => {
                    if event.target == entity && *button == MouseButton::Left {
                        self.sliding = false;
                        state.release(entity);
                        self.temp = self.value;
                    }
                }

                WindowEvent::MouseMove(_, y) => {
                    if event.target == entity {
                        if self.sliding {
                            let dy = self.mouse_down_posy - *y;

                            let normalised = if state.modifiers.shift {
                                dy / 1000.0
                            } else {
                                dy / 200.0
                            };

                            let new_val = if self.is_log {
                                let t = self.temp.log10() + (self.max_value.log10() - self.min_value.log10()) * normalised;
                                println!("norma: {}, t: {}", normalised, t);
                                10.0f32.powf((self.temp.log10() + (self.max_value.log10() - self.min_value.log10()) * normalised))
                                
                            } else {
                                self.temp + (self.max_value - self.min_value) * normalised
                            };

                            
                                

                            

                            self.value = (new_val.min(self.max_value)).max(self.min_value);

                            //println!("val: {}", normalised);

                            state.insert_event(
                                Event::new(SliderEvent::ValueChanged(self.value))
                                    .target(entity),
                            );

                            state.insert_event(
                                Event::new(WindowEvent::Redraw).target(Entity::new(0, 0)),
                            );
                        }
                    }
                }

                WindowEvent::KeyDown(input) => {
                    if let Some(virtual_keycode) = input {
                        if *virtual_keycode == crate::VirtualKeyCode::LShift {
                            if !self.shift_pressed {
                                self.shift_pressed = true;
                            }

                            self.mouse_down_posy = state.mouse.cursory;
                            self.temp = self.value;
                        }
                    }
                }

                WindowEvent::KeyUp(input) => {
                    if let Some(virtual_keycode) = input {
                        if *virtual_keycode == crate::VirtualKeyCode::LShift {
                            if self.shift_pressed {
                                self.shift_pressed = false;
                            }

                            self.mouse_down_posy = state.mouse.cursory;
                            self.temp = self.value;
                        }
                    }
                }

                _ => {}
            }
        }

        return false;
    }

    
    fn on_draw(&mut self, state: &mut State, entity: Entity, canvas: &mut Canvas<OpenGl>) {

        if state.transform.get_visibility(entity) == Visibility::Invisible {
            return;
        }

        let opacity = state.transform.get_opacity(entity);

        let mut knob_color: femtovg::Color = state
            .style
            .background_color
            .get(entity)
            .cloned()
            .unwrap_or_default()
            .into();
        knob_color.set_alphaf(knob_color.a * opacity);

        let mut back_color: femtovg::Color = state
            .style
            .background_color
            .get(self.back)
            .cloned()
            .unwrap_or_default()
            .into();
        back_color.set_alphaf(back_color.a * opacity);

        let mut slider_color: femtovg::Color = state
            .style
            .background_color
            .get(self.slider)
            .cloned()
            .unwrap_or_default()
            .into();
        slider_color.set_alphaf(slider_color.a * opacity);

        let mut tick_color: femtovg::Color = state
            .style
            .background_color
            .get(self.tick)
            .cloned()
            .unwrap_or_default()
            .into();
        tick_color.set_alphaf(tick_color.a * opacity);

        let posx = state.transform.get_posx(entity);
        let posy = state.transform.get_posy(entity);
        let width = state.transform.get_width(entity);
        let height = state.transform.get_height(entity);

        let cx = posx + 0.5 * width;
        let cy = posy + 0.5 * height;
        let r1 = width / 2.0;
        let r0 = r1 - 10.0;

        use std::f32::consts::PI;
        let start = -(PI + PI / 4.0);
        let end = PI / 4.0;

        let (min, max, value) = if self.is_log {
            (self.min_value.log10(), self.max_value.log10(), self.value.log10())
            //(self.min_value, self.max_value, self.value)
        } else {
            (self.min_value, self.max_value, self.value)
        };


        let zero_position = if self.is_log {
            start
        } else {
            (-min / (max - min)) * (end - start) + start
        };
        
        

        let normalised = (value - min) / (max - min);

        //println!("{}", zero_position);

        let current = normalised * (end - start) + start;

        canvas.save();

        // Draw outer arc background
        // let mut path = Path::new();
        // path.arc(cx, cy, r0, start, end, Solidity::Hole);
        // path.arc(cx, cy, r1, end, start, Solidity::Solid);
        // path.close();
        // let mut paint = Paint::color(back_color);
        // canvas.fill_path(&mut path, paint);


        let mut path = Path::new();
        path.arc(cx, cy, r1 - 2.5, end, start, Solidity::Solid);
        let mut paint = Paint::color(back_color);
        paint.set_line_width(5.0);
        paint.set_line_cap(LineCap::Round);
        canvas.stroke_path(&mut path, paint);

        if current != zero_position {
            let mut path = Path::new();
            if current > zero_position {
                path.arc(cx, cy, r1 - 2.5, current, zero_position, Solidity::Solid);
            } else {
                path.arc(cx, cy, r1 - 2.5, zero_position, current, Solidity::Solid);
            }
            
            let mut paint = Paint::color(slider_color);
            paint.set_line_width(5.0);
            paint.set_line_cap(LineCap::Round);
            canvas.stroke_path(&mut path, paint);
        }

        // Draw outer arc fill
        //if current != start {
            //let mut path = Path::new();
            //path.arc(cx, cy, r0, start, current, Solidity::Hole);
            //path.arc(cx, cy, r1, current, start, Solidity::Solid);
            //path.close();
            // path.arc(cx, cy, r1 - 2.5, end, start, Solidity::Solid);
            // let mut paint = Paint::color(back_color);
            // paint.set_line_width(5.0);
            // paint.set_line_cap(LineCap::Round);
            // canvas.fill_path(&mut path, paint);
        //}

        // Draw knob
        let mut path = Path::new();
        path.circle(cx, cy, r0 + 1.0);
        let mut paint = Paint::color(knob_color);
        canvas.fill_path(&mut path, paint);

        // Draw knob tick
        canvas.save();
        canvas.translate(cx, cy);
        canvas.rotate(current - PI / 2.0);

        let mut path = Path::new();
        path.circle(0.0, r0 - 2.5, 2.0);
        let mut paint = Paint::color(tick_color);
        canvas.fill_path(&mut path, paint);

        canvas.restore();
        canvas.restore();
    }
    
}
