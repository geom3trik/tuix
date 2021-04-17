#![allow(dead_code)]

use crate::state::style::*;
use crate::widgets::*;

#[derive(Debug, Clone, PartialEq)]
pub enum SliderEvent {
    ValueChanged(f32),
    SetValue(f32),
    SetMin(f32),
    SetMax(f32),
}

pub struct Slider {
    track: Entity,
    active: Entity,
    thumb: Entity,
    sliding: bool,
    /// Event sent when the slider value has changed
    on_changed: Option<Box<dyn Fn(f32) -> Event>>,
    /// event sent when the slider value is changing
    on_changing: Option<Box<dyn Fn(f32) -> Event>>,
    /// Event sent when the slider reaches the minimum value
    on_min: Option<Box<dyn Fn(f32) -> Event>>,
    /// Event sent when the slider reaches the maximum value
    on_max: Option<Box<dyn Fn(f32) -> Event>>,
    /// Event sent when the slider is pressed
    on_press: Option<Event>,
    /// Event sent when the slider is released
    on_release: Option<Event>,
    /// Event sent when the mouse cursor enters the slider
    on_over: Option<Event>,
    /// Event sent when the mouse cusor leaves the slider
    on_out: Option<Event>,

    value: f32,
    prev: f32,
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
            on_changed: None,
            on_changing: None,
            on_min: None,
            on_max: None,
            on_press: None,
            on_release: None,
            on_over: None,
            on_out: None,

            value: 0.0,
            prev: 0.0,
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

    /// Set the event sent when the slider value has changed
    pub fn on_changed<F>(mut self, message: F) -> Self
    where
        F: 'static + Fn(f32) -> Event,
    {
        self.on_changed = Some(Box::new(message));
        self
    }

    /// Set the event sent when the slider value is changing (dragging)
    pub fn on_changing<F>(mut self, message: F) -> Self
    where
        F: 'static + Fn(f32) -> Event,
    {
        self.on_changing = Some(Box::new(message));
        self
    }

    /// Set the event sent when the slider is pressed
    pub fn on_press(mut self, event: Event) -> Self {
        self.on_press = Some(event);
        self
    }

    /// Set the event sent when the slider is released
    pub fn on_release(mut self, event: Event) -> Self {
        self.on_release = Some(event);
        self
    }

    /// Set the event sent when the mouse cursor enters the slider
    pub fn on_over(mut self, event: Event) -> Self {
        self.on_over = Some(event);
        self
    }

    /// Set the event sent when the mouse cursor leaves the slider
    pub fn on_out(mut self, event: Event) -> Self {
        self.on_out = Some(event);
        self
    }

    /// Set the initial value of the slider
    pub fn with_initial_value(mut self, val: f32) -> Self {
        self.value = val;

        self
    }

    /// Set the range of the slider. Min and Max values are extracted from the range
    pub fn with_range(mut self, range: std::ops::Range<f32>) -> Self {
        self.min = range.start;
        self.max = range.end;

        self
    }

    /// Set the min value of the slider
    pub fn with_min(mut self, val: f32) -> Self {
        self.min = val;
        self
    }

    /// Set the max value of the slider
    pub fn with_max(mut self, val: f32) -> Self {
        self.max = val;
        self
    }

    // TODO
    // pub fn with_divisions(mut self, val: f32) -> Self {
    //     self.div = val;
    //     self
    // }

    fn update_visuals(&mut self, state: &mut State, entity: Entity) {
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

        let nx = (dx - thumb_width / 2.0) / (width - thumb_width);

        self.thumb
            .set_left(state, Units::Percentage((dx - thumb_width / 2.0) / width));

        self.active.set_width(state, Units::Percentage(nx));
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
                .set_width(Percentage(0.5))
                .set_height(Stretch(1.0))
                .set_hoverability(false)
                .class("active")
        });

        self.thumb = Element::new().build(
            state,
            entity,
            |builder| {
                builder
                    .set_position_type(PositioningType::SelfDirected)
                    .class("thumb")
            },
        );

        state.style.insert_element(entity, "slider");

        entity
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::GeometryChanged(_) => {
                    if event.target == entity {
                        self.update_visuals(state, entity);
                    }
                }

                WindowEvent::MouseOver => {
                    if event.target == entity {
                        if let Some(mut on_hover) = self.on_over.clone() {
                            on_hover.origin = entity;

                            if on_hover.target == Entity::null() {
                                on_hover.target = entity;
                            }

                            state.insert_event(on_hover);
                        }
                    }
                }

                WindowEvent::MouseOut => {
                    if event.target == entity {
                        if let Some(mut on_out) = self.on_out.clone() {
                            on_out.origin = entity;

                            if on_out.target == Entity::null() {
                                on_out.target = entity;
                            }

                            state.insert_event(on_out);
                        }
                    }
                }

                WindowEvent::MouseDown(button) => {
                    if *button == MouseButton::Left && event.target == entity
                        || event.target == self.thumb
                    {
                        self.sliding = true;
                        state.capture(entity);

                        self.prev = self.value;

                        entity.set_active(state, true);

                        if let Some(mut on_active) = self.on_press.clone() {
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

                        self.value = self.min + nx * (self.max - self.min);

                        self.active.set_width(state, Units::Percentage(nx));
                        self.thumb
                            .set_left(state, Units::Pixels(dx - thumb_width / 2.0));

                        if let Some(on_changing) = &self.on_changing {
                            let mut on_changing_event = (on_changing)(self.value);
                            on_changing_event.origin = entity;

                            if on_changing_event.target == Entity::null() {
                                on_changing_event.target = entity;
                            }

                            state.insert_event(on_changing_event);
                        }

                        state.insert_event(Event::new(SliderEvent::ValueChanged(self.value)).target(entity));
                    }
                }

                WindowEvent::MouseUp(button) => {
                    if *button == MouseButton::Left {
                        self.sliding = false;
                        state.release(entity);

                        if self.prev != self.value {
                            if let Some(on_changed) = &self.on_changed {
                                let mut on_changed_event = (on_changed)(self.value);
                                on_changed_event.origin = entity;

                                if on_changed_event.target == Entity::null() {
                                    on_changed_event.target = entity;
                                }

                                state.insert_event(on_changed_event);
                            }                            
                        }

                        if let Some(mut on_release) = self.on_release.clone() {
                            on_release.origin = entity;

                            if on_release.target == Entity::null() {
                                on_release.target = entity;
                            }

                            state.insert_event(on_release);
                        }

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

                        self.value = self.min + nx * (self.max - self.min);

                        self.active.set_width(state, Units::Percentage(nx));

                        self.thumb
                            .set_left(state, Units::Percentage((dx - thumb_width / 2.0) / width));

                        if let Some(on_changing) = &self.on_changing {
                            let mut on_changing_event = (on_changing)(self.value);
                            on_changing_event.origin = entity;

                            if on_changing_event.target == Entity::null() {
                                on_changing_event.target = entity;
                            }

                            state.insert_event(on_changing_event);
                        }

                        state.insert_event(Event::new(SliderEvent::ValueChanged(self.value)).target(entity));
                    }
                }

                _ => {}
            }
        }
    }
}
