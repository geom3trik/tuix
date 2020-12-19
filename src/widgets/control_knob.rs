#![allow(dead_code)]

use crate::entity::Entity;
use crate::mouse::*;
use crate::{BuildHandler, Event, EventHandler, WindowEvent};
use crate::{State};


use crate::widgets::slider::SliderEvent;
use crate::widgets::button::Button;

use femtovg::{
    renderer::OpenGl,
    Baseline,
    Canvas,
    Color,
    FillRule,
    FontId,
    ImageFlags,
    ImageId,
    LineCap,
    LineJoin,
    Paint,
    Path,
    Renderer,
    Solidity,
};

pub struct ControlKnob {
    sliding: bool,  // Could replace this with a bool in state, maybe in mouse
    value: f32,
    temp: f32,

    mouse_down_posy: f32,
    shift_pressed: bool,

    back: Entity,
    slider: Entity,
    tick: Entity,
}

impl ControlKnob {
    pub fn new() -> Self {
        ControlKnob {
            sliding: false,
            value: 0.0,
            temp: 0.0,

            mouse_down_posy: 0.0,
            shift_pressed: false,

            back: Entity::null(),
            slider: Entity::null(),
            tick: Entity::null(),

        }
    }
}

impl BuildHandler for ControlKnob {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        
        self.back = Button::new().build(state, entity, |builder| builder.set_hoverability(false).class("back"));
        self.slider = Button::new().build(state, entity, |builder| builder.set_hoverability(false).class("slider"));
        self.tick = Button::new().build(state, entity, |builder| builder.set_hoverability(false).class("tick"));

        state.style.insert_element(entity, "knob");
        
        entity
    }
}

impl EventHandler for ControlKnob {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) -> bool {
        

        if let Some(slider_event) = event.message.downcast::<SliderEvent>() {
            match slider_event {
                SliderEvent::SetValue(id, val) => {
                    if *id == entity {


                        self.value = ((*val).min(1.0)).max(0.0);

                        state.insert_event(
                            Event::new(WindowEvent::Redraw).target(Entity::new(0, 0)),
                        );
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

                WindowEvent::MouseMove(_,y) => {
                    if event.target == entity {
                        if self.sliding {
                            let dy = self.mouse_down_posy - *y;
                            
                            let normalised = if state.modifiers.shift {
                                dy/1000.0
                            } else {
                                dy/200.0
                            };
                            
                            self.value = ((self.temp + normalised).min(1.0)).max(0.0);

                            state.insert_event(
                                Event::new(SliderEvent::ValueChanged(entity, self.value))
                                    .target(entity),
                            );

                            state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::new(0, 0)));
                        }
                    }
                }

                WindowEvent::KeyInput(input) => {
                    if let Some(virtual_keycode) = input.virtual_keycode {
                        if virtual_keycode == crate::VirtualKeyCode::LShift {
                            
                            if input.state == MouseButtonState::Pressed {
                                if !self.shift_pressed {
                                    self.shift_pressed = true;
                                }
                            } else {
                                if self.shift_pressed {
                                    self.shift_pressed = false; 
                                }
                            }

                            self.mouse_down_posy = state.mouse.cursory;
                            self.temp = self.value;

                            
                        }


                    }
                }

                _=> {}
            }
        }
        
        return false;
    }

    fn on_draw(&mut self, state: &mut State, entity: Entity, canvas: &mut Canvas<OpenGl>) {


        let opacity = state.transform.get_opacity(entity);

        let mut knob_color: femtovg::Color = state.style.background_color.get(entity).cloned().unwrap_or_default().into();
        knob_color.set_alphaf(knob_color.a * opacity);

        let mut back_color: femtovg::Color = state.style.background_color.get(self.back).cloned().unwrap_or_default().into();
        back_color.set_alphaf(back_color.a * opacity);

        let mut slider_color: femtovg::Color = state.style.background_color.get(self.slider).cloned().unwrap_or_default().into(); 
        slider_color.set_alphaf(slider_color.a * opacity);

        let mut tick_color: femtovg::Color = state.style.background_color.get(self.tick).cloned().unwrap_or_default().into(); 
        tick_color.set_alphaf(tick_color.a * opacity);


        let posx = state.transform.get_posx(entity);
        let posy = state.transform.get_posy(entity);
        let width = state.transform.get_width(entity);
        let height = state.transform.get_height(entity);

        let cx = posx + 0.5 * width;
        let cy = posy + 0.5 * height;
        let r1 = width/2.0;
        let r0 = r1 - 10.0;

        let PI = 3.141592f32;

        let start = -(PI + PI/4.0);
        let end = PI/4.0;

        let current = self.value * (end - start) + start;



        canvas.save();

        // Draw outer arc background
        let mut path = Path::new();
        path.arc(cx, cy, r0, start, end, Solidity::Hole);
        path.arc(cx, cy, r1, end, start, Solidity::Solid);
        path.close();
        let mut paint = Paint::color(back_color);
        canvas.fill_path(&mut path, paint);

        //Draw outer arc fill
        if current != start {
            let mut path = Path::new();
            path.arc(cx, cy, r0, start, current, Solidity::Hole);
            path.arc(cx, cy, r1, current, start, Solidity::Solid);
            path.close();
            let mut paint = Paint::color(slider_color);
            canvas.fill_path(&mut path, paint);            
        }


        // Draw knob    
        let mut path = Path::new();
        path.circle(cx, cy, r0+1.0);
        let mut paint = Paint::color(knob_color);
        canvas.fill_path(&mut path, paint);

        // Draw knob tick
        canvas.save();
        canvas.translate(cx, cy);
        canvas.rotate(current - PI/2.0);

        let mut path = Path::new();
        path.circle(0.0, r0 - 5.0, 2.0);
        let mut paint = Paint::color(tick_color);
        canvas.fill_path(&mut path, paint);

        canvas.restore();
        canvas.restore();



        
    }
    
}