#![allow(dead_code)]

use crate::style::*;
use crate::widgets::*;

use crate::widgets::slider::SliderEvent;

use femtovg::{renderer::OpenGl, Canvas, LineCap, Paint, Path, Solidity};

use std::sync::{Arc, Mutex};

pub struct ControlKnob {
    sliding: bool, // Could replace this with a bool in state, maybe in mouse
    pub value: f32,
    temp: f32,

    mouse_down_posy: f32,
    shift_pressed: bool,

    pub back: Entity,
    pub slider: Entity,
    pub tick: Entity,

    min: f32,
    max: f32,

    pub is_log: bool,

    pub on_change: Option<Arc<Mutex<dyn Fn(f32) -> Event>>>,
    pub on_changing: Option<Arc<dyn Fn(&Self,&mut State, Entity)>>,
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

            min,
            max,

            is_log: false,

            on_change: None,
            on_changing: None,
        }
    }

    pub fn with_log_scale(mut self) -> Self {
        self.is_log = true;

        self
    }

    pub fn on_change<F>(mut self, message: F) -> Self
    where
        F: Fn(f32) -> Event,
        F: 'static,
    {
        self.on_change = Some(Arc::new(Mutex::new(message)));
        self
    }

    pub fn on_changing<F>(mut self, message: F) -> Self
    where
        F: Fn(&Self, &mut State, Entity),
        F: 'static,
    {
        self.on_changing = Some(Arc::new(message));
        self
    }
}

impl Widget for ControlKnob {
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

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(slider_event) = event.message.downcast::<SliderEvent>() {
            match slider_event {
                SliderEvent::SetValue(val) => {
                    if event.target == entity {
                        if event.target == entity {
                            self.value = ((*val).min(self.max)).max(self.min);

                            state.insert_event(
                                Event::new(WindowEvent::Redraw).target(Entity::root()),
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
                                let _t = self.temp.log10()
                                    + (self.max.log10() - self.min.log10()) * normalised;
                                10.0f32.powf(
                                    self.temp.log10()
                                        + (self.max.log10() - self.min.log10()) * normalised,
                                )
                            } else {
                                self.temp + (self.max - self.min) * normalised
                            };

                            self.value = (new_val.min(self.max)).max(self.min);

                            //println!("val: {}", normalised);

                            if let Some(on_change) = &self.on_change {
                                let mut event = (on_change.lock().unwrap())(self.value);
                                if event.target == Entity::null() {
                                    event.target = entity;
                                }

                                event.origin = entity;
                                state.insert_event(event);
                            }

                            if let Some(on_changing) = &self.on_changing {
                                (on_changing)(self, state, entity);
                            }

                            state.insert_event(
                                Event::new(SliderEvent::ValueChanged(self.value)).target(entity),
                            );

                            state.insert_event(
                                Event::new(WindowEvent::Redraw).target(Entity::root()),
                            );
                        }
                    }
                }

                WindowEvent::KeyDown(keycode, _) => {
                    if *keycode == keyboard_types::Code::ShiftLeft {
                        if !self.shift_pressed {
                            self.shift_pressed = true;
                        }

                        self.mouse_down_posy = state.mouse.cursory;
                        self.temp = self.value;
                    }
                }

                WindowEvent::KeyUp(keycode, _) => {
                    if *keycode == keyboard_types::Code::ShiftLeft {
                        if self.shift_pressed {
                            self.shift_pressed = false;
                        }

                        self.mouse_down_posy = state.mouse.cursory;
                        self.temp = self.value;
                    }
                }

                _ => {}
            }
        }
    }

    fn on_draw(&mut self, state: &mut State, entity: Entity, canvas: &mut Canvas<OpenGl>) {
        if state.data.get_visibility(entity) == Visibility::Invisible {
            return;
        }

        let opacity = state.data.get_opacity(entity);

        let mut knob_color: femtovg::Color = entity.get_background_color(state).into();
        knob_color.set_alphaf(knob_color.a * opacity);

        let mut back_color: femtovg::Color = self.back.get_background_color(state).into();    
        back_color.set_alphaf(back_color.a * opacity);

        let mut slider_color: femtovg::Color = self.slider.get_background_color(state).into();
        slider_color.set_alphaf(slider_color.a * opacity);

        let mut tick_color: femtovg::Color = self.tick.get_background_color(state).into();
        tick_color.set_alphaf(tick_color.a * opacity);

        let posx = state.data.get_posx(entity);
        let posy = state.data.get_posy(entity);
        let width = state.data.get_width(entity);
        let height = state.data.get_height(entity);

        let cx = posx + 0.5 * width;
        let cy = posy + 0.5 * height;
        let r1 = width / 2.0;
        // This property determines the thickness of the track, currently hardcoded to be 10 pixels
        let r0 = r1 - 10.0;

        use std::f32::consts::PI;
        let start = -(PI + PI / 4.0);
        let end = PI / 4.0;

        let (min, max, value) = if self.is_log {
            (self.min.log10(), self.max.log10(), self.value.log10())
        } else {
            (self.min, self.max, self.value)
        };

        let zero_position = if self.is_log {
            start
        } else {
            (-min / (max - min)) * (end - start) + start
        };

        let normalised = (value - min) / (max - min);

        let current = normalised * (end - start) + start;

        canvas.save();

        // Draw the background of the slider
        let mut path = Path::new();
        path.arc(cx, cy, r1 - 2.5, end, start, Solidity::Solid);
        let mut paint = Paint::color(back_color);
        paint.set_line_width(5.0);
        paint.set_line_cap(LineCap::Butt);
        canvas.stroke_path(&mut path, paint);

        // Draw the filled part of the slider
        if current != zero_position {
            let mut path = Path::new();
            if current > zero_position {
                path.arc(cx, cy, r1 - 2.5, current, zero_position, Solidity::Solid);
            } else {
                path.arc(cx, cy, r1 - 2.5, zero_position, current, Solidity::Solid);
            }

            let mut paint = Paint::color(slider_color);
            paint.set_line_width(5.0);
            paint.set_line_cap(LineCap::Butt);
            canvas.stroke_path(&mut path, paint);
        }

        // Draw the inner knob
        let mut path = Path::new();
        path.circle(cx, cy, r0 + 1.0);
        let paint = Paint::color(knob_color);
        canvas.fill_path(&mut path, paint);

        // Draw knob tick mark
        canvas.save();
        canvas.translate(cx, cy);
        canvas.rotate(current - PI / 2.0);

        let mut path = Path::new();
        path.circle(0.0, r0 - 2.5, 2.0);
        let paint = Paint::color(tick_color);
        canvas.fill_path(&mut path, paint);

        canvas.restore();
        
        canvas.restore();
    }
}
