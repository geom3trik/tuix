use crate::state::style::*;
use crate::widgets::*;

#[derive(Debug, Clone, PartialEq)]
pub enum SliderEvent {
    // TODO - Remove this
    ValueChanged(f32),
    SetValue(f32),
    SetMin(f32),
    SetMax(f32),
}

pub struct Slider {
    // The track that the thumb slides along
    track: Entity,
    // An overlay on the track to indicate the value
    active: Entity,
    // A marker used to indicate the value by its position along the track
    thumb: Entity,

    // Event sent when the slider value has changed
    on_change: Option<Box<dyn Fn(&mut Self, &mut State, Entity)>>,
    // event sent when the slider value is changing
    on_changing: Option<Box<dyn Fn(&mut Self, &mut State, Entity)>>,
    // Event sent when the slider reaches the minimum value
    on_min: Option<Box<dyn Fn(&mut Self, &mut State, Entity)>>,
    // Event sent when the slider reaches the maximum value
    on_max: Option<Box<dyn Fn(&mut Self, &mut State, Entity)>>,
    // Event sent when the slider is pressed
    on_press: Option<Box<dyn Fn(&mut Self, &mut State, Entity)>>,
    // Event sent when the slider is released
    on_release: Option<Box<dyn Fn(&mut Self, &mut State, Entity)>>,
    // Event sent when the mouse cursor enters the slider
    on_over: Option<Box<dyn Fn(&mut Self, &mut State, Entity)>>,
    // Event sent when the mouse cusor leaves the slider
    on_out: Option<Box<dyn Fn(&mut Self, &mut State, Entity)>>,

    pub value: f32,
    prev: f32,
    min: f32,
    max: f32,

    is_min: bool,
    is_max: bool,
}

impl Default for Slider {
    fn default() -> Self {
        Self {
            track: Entity::default(),
            active: Entity::default(),
            thumb: Entity::default(),

            on_change: None,
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

            is_min: true,
            is_max: false,
        }
    }
}

impl Slider {
    /// Create a new slider widget with default values (min: 0.0, max: 1.0, val: 0.0).
    ///
    /// # Example
    /// 
    /// ```
    /// Slider::new().build(state, parent, |builder| builder);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the initial value of the slider.
    ///
    /// # Example
    /// 
    /// ```
    /// Slider::new()
    ///    .with_init(0.5)
    ///    .build(state, parent, |builder| builder)
    /// ```
    pub fn with_init(mut self, val: f32) -> Self {
        self.value = val;

        self
    }

    /// Set the range of the slider. Min and Max values are extracted from the range.
    ///
    /// # Example
    /// 
    /// ```
    /// Slider::new()
    ///     .with_range(0.0..5.0)
    ///     .build(state, parent, |builder| builder)
    /// ```
    pub fn with_range(mut self, range: std::ops::Range<f32>) -> Self {
        self.min = range.start;
        self.max = range.end;

        self
    }

    /// Set the minimum value of the slider.
    ///
    /// # Example
    /// 
    /// ```
    /// Slider::new()
    ///     .with_min(0.2)
    ///     .build(state, parent, |builder| builder)
    /// ```
    pub fn with_min(mut self, val: f32) -> Self {
        self.min = val;
        self
    }

    /// Set the maximum value of the slider.
    ///
    /// # Example
    /// 
    /// ```
    /// Slider::new()
    ///     .with_max()
    ///     .build(state, parent, |builder| builder)
    /// ```
    pub fn with_max(mut self, val: f32) -> Self {
        self.max = val;
        self
    }

    /// Set the callback triggered when the slider value has changed.
    ///
    /// Takes a closure which provides the current value and returns an event to be sent when the slider
    /// value has changed after releasing the slider. If the slider thumb is pressed but not moved, and thus
    /// the value is not changed, then the event will not be sent.
    ///
    /// # Example
    /// 
    /// ```
    /// Slider::new()
    ///     .on_change(|slider, state, entity| {
    ///         entity.emit(WindowEvent::Debug(format!("Slider on_change: {}", slider.value)));
    ///     })
    ///     .build(state, parent, |builder| builder);
    /// ```
    pub fn on_change<F>(mut self, callback: F) -> Self
    where
        F: 'static + Fn(&mut Self, &mut State, Entity),
    {
        self.on_change = Some(Box::new(callback));
        self
    }

    /// Set the callback triggered when the slider value is changing (dragging).
    ///
    /// Takes a closure which triggers when the slider value is changing, 
    /// either by pressing the track or dragging the thumb along the track.
    ///
    /// # Example
    /// 
    /// ```
    /// Slider::new()
    ///     .on_changing(|slider, state, entity| {
    ///         entity.emit(WindowEvent::Debug(format!("Slider on_changing: {}", slider.value)));
    ///     })
    ///     .build(state, parent, |builder| builder);
    /// ```
    pub fn on_changing<F>(mut self, callback: F) -> Self
    where
        F: 'static + Fn(&mut Self, &mut State, Entity),
    {
        self.on_changing = Some(Box::new(callback));
        self
    }

    /// Set the callback triggered when the slider value reaches the minimum.
    ///
    /// Takes a closure which triggers when the slider reaches the minimum value, 
    /// either by pressing the track at the start or dragging the thumb to the start
    /// of the track. The event is sent once for each time the value reaches the minimum.
    ///
    /// # Example
    /// 
    /// ```
    /// Slider::new()
    ///     .on_min(|slider, state, entity| {
    ///         entity.emit(WindowEvent::Debug(format!("Slider on_min: {}", slider.value)));
    ///     })
    ///     .build(state, parent, |builder| builder);
    /// ```
    pub fn on_min<F>(mut self, callback: F) -> Self
    where
        F: 'static + Fn(&mut Self, &mut State, Entity),
    {
        self.on_min = Some(Box::new(callback));
        self
    }

    /// Set the callback triggered when the slider value reaches the maximum.
    ///
    /// Takes a closure which triggers when the slider reaches the maximum value, 
    /// either by pressing the track at the end or dragging the thumb to the end
    /// of the track. The event is sent once for each time the value reaches the maximum.
    ///
    /// # Example
    /// 
    /// ```
    /// Slider::new()
    ///     .on_max(|slider, state, entity| {
    ///         entity.emit(WindowEvent::Debug(format!("Slider on_min: {}", slider.value)));
    ///     })
    ///     .build(state, parent, |builder| builder);
    /// ```
    pub fn on_max<F>(mut self, callback: F) -> Self
    where
        F: 'static + Fn(&mut Self, &mut State, Entity),
    {
        self.on_max = Some(Box::new(callback));
        self
    }

    /// Set the event sent when the slider is pressed.
    ///
    /// The event is sent when the left mouse button is pressed on any part of the slider.
    ///
    /// # Example
    /// 
    /// ```
    /// Slider::new()
    ///     .on_max(|slider, state, entity| {
    ///         entity.emit(WindowEvent::Debug(format!("Slider on_min: {}", slider.value)));
    ///     })
    ///     .build(state, parent, |builder| builder);
    /// ```
    // pub fn on_press<F>(mut self, callback: F) -> Self 
    // where
    //     F: 'static + Fn(&mut Self, &mut State, Entity),
    // {
    //     self.on_press = Some(Box::new(callback));
    //     self
    // }

    /// Set the event sent when the slider is released.
    ///
    /// The event is sent when the left mouse button is released after being pressed on any part of the slider.
    ///
    /// # Example
    /// 
    /// ```
    /// Slider::new()
    ///     .on_max(|slider, state, entity| {
    ///         entity.emit(WindowEvent::Debug(format!("Slider on_min: {}", slider.value)));
    ///     })
    ///     .build(state, parent, |builder| builder);
    /// ```
    pub fn on_release<F>(mut self, callback: F) -> Self 
    where
        F: 'static + Fn(&mut Self, &mut State, Entity),
    {
        self.on_release = Some(Box::new(callback));
        self
    }

    /// Set the event sent when the mouse cursor enters the slider.
    ///
    /// The event is sent when the mouse cursor enters the bounding box of the slider.
    ///
    /// # Example
    /// 
    /// ```
    /// Slider::new()
    ///     .on_max(|slider, state, entity| {
    ///         entity.emit(WindowEvent::Debug(format!("Slider on_min: {}", slider.value)));
    ///     })
    ///     .build(state, parent, |builder| builder);
    /// ```
    pub fn on_over<F>(mut self, callback: F) -> Self 
    where
        F: 'static + Fn(&mut Self, &mut State, Entity),
    {
        self.on_over = Some(Box::new(callback));
        self
    }

    /// Set the event sent when the mouse cursor leaves the slider
    ///
    /// The event is sent when the mouse cursor leaves the bounding box of the slider.
    ///
    /// # Example
    /// 
    /// ```
    /// Slider::new()
    ///     .on_max(|slider, state, entity| {
    ///         entity.emit(WindowEvent::Debug(format!("Slider on_min: {}", slider.value)));
    ///     })
    ///     .build(state, parent, |builder| builder);
    /// ```
    pub fn on_out<F>(mut self, callback: F) -> Self 
    where
        F: 'static + Fn(&mut Self, &mut State, Entity),
    {
        self.on_out = Some(Box::new(callback));
        self
    }

    // Private helper functions

    // Update the active size and thumb position
    fn update_value(&mut self, state: &mut State, entity: Entity, mut dx: f32) {
        let width = state.data.get_width(entity);
        let thumb_width = state.data.get_width(self.thumb);

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

        self.value = self.min + nx * (self.max - self.min);

        if self.value == self.min {
            if !self.is_min {
                self.is_min = true;
                //self.send_value_event(state, entity, &self.on_min);
                if let Some(callback) = self.on_min.take() {
                    (callback)(self, state, entity);
                    self.on_min = Some(callback);
                }
            }
        } else {
            self.is_min = false;
        }

        if self.value == self.max {
            if !self.is_max {
                self.is_max = true;
                if let Some(callback) = self.on_max.take() {
                    (callback)(self, state, entity);
                    self.on_max = Some(callback);
                }
            }
        } else {
            self.is_max = false;
        }
    }

    fn update_visuals(&mut self, state: &mut State, entity: Entity) {
        let normalised_value = (self.value - self.min) / (self.max - self.min);

        let width = state.data.get_width(entity);
        let thumb_width = state.data.get_width(self.thumb);

        let dx = normalised_value * (width - thumb_width) + thumb_width / 2.0;

        self.update_value(state, entity, dx);
    }

    fn clamp_value(&mut self) {
        self.value = self.value.clamp(self.min, self.max);
    }
}

impl Widget for Slider {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        if self.min > self.max {
            panic!("minimum value must be less than maximum value")
        }

        self.clamp_value();

        self.is_min = self.value == self.min;
        self.is_max = self.value == self.max;

        entity
            .set_layout_type(state, LayoutType::Row)
            .set_child_top(state, Stretch(1.0))
            .set_child_bottom(state, Stretch(1.0));

        // Track
        self.track = Element::new().build(state, entity, |builder| {
            builder
                .set_width(Stretch(1.0))
                .set_height(Pixels(4.0))
                .set_bottom(Auto)
                .set_hoverability(false)
                .class("track")
        });

        // Active
        self.active = Element::new().build(state, self.track, |builder| {
            builder
                .set_width(Percentage(0.5))
                .set_height(Stretch(1.0))
                .set_hoverability(false)
                .class("active")
        });

        // Thumb
        self.thumb = Element::new().build(state, entity, |builder| {
            builder
                .set_position_type(PositionType::SelfDirected)
                .set_hoverability(false)
                .class("thumb")
        });

        state.style.insert_element(entity, "slider");

        entity
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        // Handle window events
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::GeometryChanged(_) if event.target == entity => {
                    self.update_visuals(state, entity);
                }

                WindowEvent::MouseOver if event.target == entity => {
                    if let Some(callback) = self.on_over.take() {
                        (callback)(self, state, entity);
                        self.on_over = Some(callback);
                    }
                }

                WindowEvent::MouseOut if event.target == entity => {
                    if let Some(callback) = self.on_out.take() {
                        (callback)(self, state, entity);
                        self.on_out = Some(callback);
                    }
                }

                WindowEvent::MouseDown(button) if event.target == entity => {
                    if *button == MouseButton::Left {
                        state.capture(entity);

                        self.prev = self.value;

                        entity.set_active(state, true);

                        if let Some(callback) = self.on_press.take() {
                            (callback)(self, state, entity);
                            self.on_press = Some(callback);
                        }

                        let dx = state.mouse.left.pos_down.0 - state.data.get_posx(entity);

                        self.update_value(state, entity, dx);

                        if let Some(callback) = self.on_changing.take() {
                            (callback)(self, state, entity);
                            self.on_changing = Some(callback);
                        }

                        state.insert_event(
                            Event::new(SliderEvent::ValueChanged(self.value)).target(entity),
                        );
                    }
                }

                WindowEvent::MouseUp(button) if event.target == entity => {
                    if *button == MouseButton::Left {
                        state.release(entity);

                        entity.set_active(state, false);

                        if self.prev != self.value {
                            //self.send_value_event(state, entity, &self.on_change);
                            if let Some(callback) = self.on_change.take() {
                                (callback)(self, state, entity);
                                self.on_change = Some(callback);
                            }

                        }

                        if let Some(callback) = self.on_release.take() {
                            (callback)(self, state, entity);
                            self.on_release = Some(callback);
                        }
                    }
                }

                WindowEvent::MouseMove(x, _) if event.target == entity => {
                    if entity.is_active(state) {
                        let dx = *x - state.data.get_posx(entity);

                        self.update_value(state, entity, dx);
                        
                        if let Some(callback) = self.on_changing.take() {
                            (callback)(self, state, entity);
                            self.on_changing = Some(callback);
                        }
                    }
                }

                // TODO - Add keyboard control
                _ => {}
            }
        }

        // Handle slider events
        if let Some(slider_event) = event.message.downcast() {
            match slider_event {
                SliderEvent::SetMin(val) => {
                    self.min = *val;
                    self.min = self.min.min(self.max);
                    self.clamp_value();

                    self.update_visuals(state, entity);
                }

                SliderEvent::SetMax(val) => {
                    self.max = *val;
                    self.max = self.max.max(self.min);
                    self.clamp_value();

                    self.update_visuals(state, entity);
                }

                SliderEvent::SetValue(val) => {
                    self.value = *val;
                    self.clamp_value();

                    self.update_visuals(state, entity);
                }

                _ => {}
            }
        }
    }
}
