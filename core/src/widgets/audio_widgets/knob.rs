
use crate::style::*;
use crate::widgets::*;
use femtovg::*;

use super::NormalizedMap;

use std::f32::consts::PI;

static DEFAULT_DRAG_SCALAR: f32 = 0.0042;
static DEFAULT_WHEEL_SCALAR: f32 = 0.005;
static DEFAULT_MODIFIER_SCALAR: f32 = 0.04;

pub struct ArcTrack {
    angle_start: f32,
    angle_end: f32,
    radius: Units,
    span: Units,

    front: Entity,

    normalized_value: f32,
}

impl ArcTrack {
    pub fn new(normalized_value: f32) -> Self {
        Self {
            angle_start: -150.0,
            angle_end: 150.0,
            radius: Units::Pixels(30.0),
            span: Units::Pixels(5.0),

            front: Entity::null(),

            normalized_value: normalized_value.clamp(0.0, 1.0),
        }
    }
}

impl Widget for ArcTrack {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        // Non-displayed element used for setting the color of the active arc
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

        // Clalculate arc center
        let cx = posx + 0.5 * width;
        let cy = posy + 0.5 * height;

        // Convert start and end angles to radians and rotate origin direction to be upwards instead of to the right
        let start = self.angle_start.to_radians() - PI/2.0;
        let end = self.angle_end.to_radians() - PI/2.0;

        let parent = entity.get_parent(state).unwrap();

        let parent_width = state.data.get_width(parent);

        // Convert radius and span into screen coordinates
        let radius = self.radius.get_value(parent_width);
        let span = self.span.get_value(parent_width);
        
        // Draw the track arc
        let mut path = Path::new();
        path.arc(cx, cy, radius - span/2.0, end, start, Solidity::Solid);
        let mut paint = Paint::color(background_color);
        paint.set_line_width(span);
        paint.set_line_cap(LineCap::Butt);
        canvas.stroke_path(&mut path, paint);

        let current = self.normalized_value * (end - start) + start;

        // Draw the active arc
        let mut path = Path::new();
        path.arc(cx, cy, radius - span/2.0, current, start, Solidity::Solid);
        let mut paint = Paint::color(foreground_color);
        paint.set_line_width(span);
        paint.set_line_cap(LineCap::Butt);
        canvas.stroke_path(&mut path, paint);
    }
}

pub struct Knob<T: NormalizedMap> {
    thumb: Entity,
    value_track: Entity,
    mod_track: Entity,
    tick: Entity,

    normalized_value: f32,
    default_normal: f32,

    is_dragging: bool,
    prev_drag_y: f32,
    continuous_normal: f32,
    
    drag_scalar: f32,
    wheel_scalar: f32,
    modifier_scalar: f32,

    map: T,
}

impl<T: NormalizedMap> Knob<T> {
    pub fn new(map: T, normalized_default: f32) -> Self {
        let normalized_default = normalized_default.clamp(0.0, 1.0);

        Self {
            thumb: Default::default(),
            value_track: Default::default(),
            mod_track: Default::default(),
            tick: Default::default(),

            normalized_value: normalized_default,
            default_normal: normalized_default,

            is_dragging: false,
            prev_drag_y: 0.0,
            continuous_normal: normalized_default,

            drag_scalar: DEFAULT_DRAG_SCALAR,
            wheel_scalar: DEFAULT_WHEEL_SCALAR,
            modifier_scalar: DEFAULT_MODIFIER_SCALAR,

            map,
        }
    }

    pub fn map(&self) -> &T {
        &self.map
    }
}

impl<T: NormalizedMap> Widget for Knob<T> {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        self.value_track = ArcTrack::new(self.normalized_value).build(state, entity, |builder| 
            builder
                .set_position_type(PositionType::SelfDirected)
                .set_hoverability(false)
                .class("value_track")
        );
        
        self.mod_track = ArcTrack::new(self.normalized_value).build(state, entity, |builder| 
            builder
                .set_position_type(PositionType::SelfDirected)
                .set_hoverability(false)
                .class("mod_track")
            
        );

        entity.set_element(state, "knob")
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        let move_virtual_slider = |self_ref: &mut Self, state: &mut State, new_normal: f32| {
            self_ref.continuous_normal = new_normal.clamp(0.0, 1.0);

            // This will cause the knob to "snap" when using an `IntMap`.
            self_ref.normalized_value = self_ref.map.snap(self_ref.continuous_normal);
            
            // TODO - Remove when done
            println!("Normalized: {}, Display: {}", self_ref.normalized_value, self_ref.map.normalized_to_display(self_ref.normalized_value));

            if let Some(track) = state.query::<ArcTrack>(self_ref.value_track) {
                track.normalized_value = self_ref.normalized_value;
            }

            state.insert_event(
                Event::new(WindowEvent::Redraw).target(Entity::root()),
            );
        };

        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::MouseDown(button) => {
                    if event.target == entity && *button == MouseButton::Left {
                        self.is_dragging = true;
                        self.prev_drag_y = state.mouse.left.pos_down.1;

                        state.capture(entity);
                        state.focused = entity;
                    }
                }

                WindowEvent::MouseUp(button) => {
                    if *button == MouseButton::Left {
                        self.is_dragging = false;
                        self.continuous_normal = self.normalized_value;

                        state.release(entity);
                    }
                }

                WindowEvent::MouseMove(_, y) => {
                    if event.target == entity {
                        if self.is_dragging {
                            let mut delta_normal = (*y - self.prev_drag_y) * self.drag_scalar;

                            self.prev_drag_y = *y;

                            if state.modifiers.shift {
                                delta_normal *= self.modifier_scalar;
                            }
                
                            let new_normal = self.continuous_normal - delta_normal;

                            move_virtual_slider(self, state, new_normal);
                        }
                    }
                }

                WindowEvent::MouseScroll(_, y) => {
                    if *y != 0.0 {
                        let delta_normal = -*y * self.wheel_scalar;

                        let new_normal = self.continuous_normal - delta_normal;

                        move_virtual_slider(self, state, new_normal);
                    }
                }

                WindowEvent::MouseDoubleClick(button) => {
                    if event.target == entity && *button == MouseButton::Left {
                        self.is_dragging = false;

                        move_virtual_slider(self, state, self.default_normal);
                    }
                }

                _ => {}
            }
        }        
    }
}