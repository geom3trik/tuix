#![allow(dead_code)]

use crate::state::style::*;
use crate::widgets::*;

#[derive(Debug, Clone, PartialEq)]
pub enum SliderEvent {
    ValueChanged(f32),
    SetValue(f32),
}

pub struct Slider {
    track: Entity,
    active: Entity,
    thumb: Entity,
    sliding: bool,
    on_change: Option<Box<dyn Fn(f32) -> Event>>,
    on_active: Option<Event>,
    on_hover: Option<Event>,

    value: f32,

    min: f32,
    max: f32,
    div: f32,
}

impl Default for Slider {
    fn default() -> Self {
        Self {
            track: Entity::default(),
            active: Entity::default(),
            thumb: Entity::default(),
            sliding: false,
            on_change: None,
            on_active: None,
            on_hover: None,
            value: 0.0,
            min: 0.0,
            max: 1.0,
            div: 0.01,
        }
    }
}

impl Slider {
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the event generating closure that will be called when the slider value changes
    pub fn on_change<F>(mut self, message: F) -> Self
    where
        F: 'static + Fn(f32) -> Event,
    {
        self.on_change = Some(Box::new(message));
        self
    }

    pub fn on_active(mut self, event: Event) -> Self {
        self.on_active = Some(event);
        self
    }

    pub fn on_over(mut self, event: Event) -> Self {
        self.on_hover = Some(event);
        self
    }

    pub fn with_initial_value(mut self, val: f32) -> Self {
        self.value = val;

        self
    }

    pub fn with_range(mut self, range: std::ops::Range<f32>) -> Self {
        self.min = range.start;
        self.max = range.end;

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

impl Widget for Slider {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
            .set_flex_direction(state, FlexDirection::Row)
            .set_child_top(state, Stretch(1.0))
            .set_child_bottom(state, Stretch(1.0));


        self.track = Element::new().build(state, entity, |builder|
            builder
                .set_width(Stretch(1.0))
                .set_height(Pixels(4.0))
                .set_bottom(Auto)
                .set_hoverability(false)
                .class("track")
        );

        self.active = Element::new().build(state, self.track, |builder| {
            builder
                //.set_position(Position::Absolute)
                //.set_position_type(PositioningType::SelfDirected)
                .set_width(Percentage(0.5))
                .set_height(Stretch(1.0))
                //.set_background_color(Color::rgb(60, 60, 200))
                .set_hoverability(false)
                .class("active")
        });

        self.thumb = Element::new().build(
            state,
            entity,
            |builder| {
                builder
                    .set_position_type(PositioningType::SelfDirected)
                    //.set_position(Position::Absolute)
                    //.set_top(Units::Pixels(-8.0))
                    //.set_width(Units::Pixels(20.0))
                    //.set_height(Units::Pixels(20.0))
                    .class("thumb")
            }, //.set_background_color(Color::rgb(80, 80, 200))
        );

        // TEMP
        self.thumb.set_left(state, Units::Percentage(0.0));
        self.active.set_width(state, Units::Percentage(self.value));

        state.style.insert_element(entity, "slider");

        entity
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::GeometryChanged(_) => {
                    if event.target == entity {
                        let width = state.data.get_width(entity);
                        let thumb_width = state.data.get_width(self.thumb);

                        let normalised_value = self.value / (self.max - self.min);

                        let mut dx = normalised_value * (width - thumb_width) + thumb_width / 2.0;

                        if dx <= thumb_width / 2.0 {
                            dx = thumb_width / 2.0;
                        }
                        if dx >= width - thumb_width / 2.0 {
                            dx = width - thumb_width / 2.0;
                        }

                        self.thumb
                            .set_left(state, Units::Percentage((dx - thumb_width / 2.0) / width));
                    }
                }

                WindowEvent::MouseOver => {
                    if event.target == entity {
                        if let Some(mut on_hover) = self.on_hover.clone() {
                            on_hover.origin = entity;

                            if on_hover.target == Entity::null() {
                                on_hover.target = entity;
                            }

                            state.insert_event(on_hover);
                        }
                    }
                }

                WindowEvent::MouseDown(button) => {
                    if *button == MouseButton::Left && event.target == entity
                        || event.target == self.thumb
                    {
                        self.sliding = true;
                        state.capture(entity);

                        entity.set_active(state, true);

                        if let Some(mut on_active) = self.on_active.clone() {
                            on_active.origin = entity;

                            if on_active.target == Entity::null() {
                                on_active.target = entity;
                            }

                            state.insert_event(on_active);
                        }


                        let width = state.data.get_width(entity);
                        let thumb_width = state.data.get_width(self.thumb);

                        let mut dx = state.mouse.left.pos_down.0 - state.data.get_posx(entity);

                        if dx <= thumb_width / 2.0 {
                            dx = thumb_width / 2.0;
                        }
                        if dx >= width - thumb_width / 2.0 {
                            dx = width - thumb_width / 2.0;
                        }

                        let nx = (dx - thumb_width / 2.0) / (width - thumb_width);

                        let v = self.min + nx * (self.max - self.min);

                        self.value = v;

                        self.active.set_width(state, Units::Percentage(nx));
                        self.thumb
                            .set_left(state, Units::Pixels(dx - thumb_width / 2.0));

                        if let Some(on_change) = &self.on_change {
                            let mut on_change_event = (on_change)(v);
                            on_change_event.origin = entity;

                            if on_change_event.target == Entity::null() {
                                on_change_event.target = entity;
                            }

                            state.insert_event(on_change_event);
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

                        let nx = (dx - thumb_width / 2.0) / (width - thumb_width);

                        let v = self.min + nx * (self.max - self.min);

                        self.active.set_width(state, Units::Percentage(nx));

                        self.thumb
                            .set_left(state, Units::Percentage((dx - thumb_width / 2.0) / width));

                        self.value = v;

                        if let Some(on_change) = &self.on_change {
                            let mut on_change_event = (on_change)(v);
                            on_change_event.origin = entity;

                            if on_change_event.target == Entity::null() {
                                on_change_event.target = entity;
                            }

                            state.insert_event(on_change_event);
                        }

                        state.insert_event(Event::new(SliderEvent::ValueChanged(v)).target(entity));
                    }
                }

                _ => {}
            }
        }
    }
}
