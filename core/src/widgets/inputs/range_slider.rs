use crate::style::*;
use crate::widgets::*;

pub struct RangeSlider {
    first: Entity,
    second: Entity,

    active: Entity,

    min: f32,
    max: f32,

    first_value: f32,
    second_value: f32,

    sliding: bool,
}

impl RangeSlider {
    pub fn new() -> Self {
        Self {
            first: Entity::null(),
            second: Entity::null(),

            active: Entity::null(),

            min: 0.0,
            max: 1.0,

            first_value: 0.0,
            second_value: 1.0,

            sliding: false,
        }
    }
}

impl Widget for RangeSlider {
    type Ret = Entity;
    fn on_build(&mut self, mut builder: Builder) -> Self::Ret {
        self.active = Element::new().build(&mut builder).class("active").entity();

        self.first = Element::new().build(&mut builder).class("first").entity();

        self.first = Element::new().build(&mut builder).class("second").entity();

        builder.entity()
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                // WindowEvent::GeometryChanged(_) => {
                //     if event.target == entity {
                //         let width = state.data.get_width(entity);
                //         let thumb_width = state.data.get_width(self.thumb);

                //         let mut dx = self.value * (width - thumb_width) + thumb_width/2.0;

                //         if dx <= thumb_width / 2.0 {
                //             dx = thumb_width / 2.0;
                //         }
                //         if dx >= width - thumb_width / 2.0 {
                //             dx = width - thumb_width / 2.0;
                //         }

                //         self.thumb
                //             .set_left(state, Length::Percentage((dx - thumb_width / 2.0) / width));
                //     }
                // }
                WindowEvent::MouseDown(button) => {
                    if *button == MouseButton::Left && event.target == entity
                        || event.target == self.first
                    {
                        self.sliding = true;
                        state.capture(entity);

                        let width = state.data.get_width(entity);
                        let thumb_width = state.data.get_width(self.first);

                        let mut dx = state.mouse.left.pos_down.0 - state.data.get_posx(entity);

                        if dx <= thumb_width / 2.0 {
                            dx = thumb_width / 2.0;
                        }
                        if dx >= width - thumb_width / 2.0 {
                            dx = width - thumb_width / 2.0;
                        }

                        let nx = (dx - thumb_width / 2.0) / (width - thumb_width);

                        let v = self.min + nx * (self.max - self.min);

                        self.active.set_width(state, Length::Percentage(nx));
                        self.first
                            .set_left(state, Length::Pixels(dx - thumb_width / 2.0));

                        // if let Some(on_change) = &self.on_change {
                        //     let mut event = (on_change)(v);
                        //     event.origin = entity;

                        //     state.insert_event(event);
                        // }

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
                        let thumb_width = state.data.get_width(self.first);

                        let mut dx = *x - state.data.get_posx(entity);

                        if dx <= thumb_width / 2.0 {
                            dx = thumb_width / 2.0;
                        }
                        if dx >= width - thumb_width / 2.0 {
                            dx = width - thumb_width / 2.0;
                        }

                        let nx = (dx - thumb_width / 2.0) / (width - thumb_width);

                        let v = self.min + nx * (self.max - self.min);

                        self.active.set_width(state, Length::Percentage(nx));

                        self.first
                            .set_left(state, Length::Percentage((dx - thumb_width / 2.0) / width));

                        //self.value = v;
                        // if let Some(on_change) = &self.on_change {
                        //     let mut event = (on_change)(v);
                        //     event.origin = entity;

                        //     state.insert_event(event);
                        // }

                        state.insert_event(Event::new(SliderEvent::ValueChanged(v)).target(entity));
                    }
                }

                _ => {}
            }
        }
    }
}
