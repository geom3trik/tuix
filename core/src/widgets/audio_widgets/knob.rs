
use crate::style::*;
use crate::widgets::*;
use femtovg::*;

const PI: f32 = std::f32::consts::PI;

pub struct ArcTrack {
    angle_start: f32,
    angle_end: f32,
    radius: Units,
    span: Units,

    front: Entity,

    value: f32,
}

impl ArcTrack {
    pub fn new() -> Self {
        Self {
            angle_start: -150.0,
            angle_end: 150.0,
            radius: Units::Pixels(30.0),
            span: Units::Pixels(5.0),

            front: Entity::null(),

            value: 0.5,
        }
    }
}

impl Widget for ArcTrack {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        self.front = Element::new().build(state, entity, |builder|
            builder
                .set_hoverability(false)
                .set_display(Display::None)
                .class("active")
        );

        entity.set_element(state, "arc")
    }

    fn on_style(&mut self, state: &mut State, entity: Entity, property: (String, PropType)) {
        if property.0 == "radius" {
            match property.1 {
                PropType::Units(value) => {
                    self.radius = value;
                }
                _=> {}
            }
        }

        if property.0 == "span" {
            match property.1 {
                PropType::Units(value) => {
                    self.span = value;
                }
                _=> {}
            }
        }

        if property.0 == "angle-start" {
            match property.1 {
                PropType::Units(value) => {
                    self.angle_start = value.get_value(0.0);
                    println!("Start: {}", self.angle_start);
                }
                _=> {}
            }
        }

        if property.0 == "angle-end" {
            match property.1 {
                PropType::Units(value) => {
                    self.angle_end = value.get_value(0.0);
                    println!("End: {}", self.angle_end);
                }
                _=> {}
            }
        }
    }

    fn on_draw(&mut self, state: &mut State, entity: Entity, canvas: &mut crate::widget::Canvas) {
        
        let opacity = state.data.get_opacity(entity);
        
        let mut background_color: femtovg::Color = entity.get_background_color(state).into();    
        background_color.set_alphaf(background_color.a * opacity);

        let mut foreground_color: femtovg::Color = self.front.get_background_color(state).into();    
        foreground_color.set_alphaf(foreground_color.a * opacity);


        let posx = state.data.get_posx(entity);
        let posy = state.data.get_posy(entity);
        let width = state.data.get_width(entity);
        let height = state.data.get_height(entity);

        let cx = posx + 0.5 * width;
        let cy = posy + 0.5 * height;

        let start = self.angle_start.to_radians() - PI/2.0;
        let end = self.angle_end.to_radians() - PI/2.0;

        let parent = entity.get_parent(state).unwrap();

        let parent_width = state.data.get_width(parent);
        let parent_height = state.data.get_height(parent);

        let radius = self.radius.get_value(parent_width);
        let span = self.span.get_value(parent_width);
        
        let mut path = Path::new();
        path.arc(cx, cy, radius - span/2.0, end, start, Solidity::Solid);
        let mut paint = Paint::color(background_color);
        paint.set_line_width(span);
        paint.set_line_cap(LineCap::Butt);
        canvas.stroke_path(&mut path, paint);

        let current = self.value * (end - start) + start;

        let mut path = Path::new();
        path.arc(cx, cy, radius - span/2.0, current, start, Solidity::Solid);
        let mut paint = Paint::color(foreground_color);
        paint.set_line_width(span);
        paint.set_line_cap(LineCap::Butt);
        canvas.stroke_path(&mut path, paint);
    }
}

#[derive(Default)]
pub struct Knob {
    thumb: Entity,
    value_track: Entity,
    mod_track: Entity,
    tick: Entity,

    mouse_down_posy: f32,

    sliding: bool,
    value: f32,
    temp: f32,
}



impl Widget for Knob {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        self.value_track = ArcTrack::new().build(state, entity, |builder| 
            builder
                .set_position_type(PositionType::SelfDirected)
                .set_hoverability(false)
                .class("value_track")
        );
        
        self.mod_track = ArcTrack::new().build(state, entity, |builder| 
            builder
                .set_position_type(PositionType::SelfDirected)
                .set_hoverability(false)
                .class("mod_track")
            
        );

        entity.set_element(state, "knob")
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
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

                            let new_val = self.temp + normalised;
                            

                            self.value = new_val.clamp(0.0, 1.0);

                            println!("Value: {}", self.value);

                            if let Some(track) = state.query::<ArcTrack>(self.value_track) {
                                track.value = self.value;
                            }

                            state.insert_event(
                                Event::new(WindowEvent::Redraw).target(Entity::root()),
                            );
                        }
                    }
                }

                WindowEvent::MouseScroll(x, y) => {
                    
                }

                _ => {}
            }
        }        
    }
}