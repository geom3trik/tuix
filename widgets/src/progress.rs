use crate::common::*;

pub struct ProgressBar {
    front: Entity,
    value: f32,
}

impl ProgressBar {
    pub fn new() -> Self {
        Self {
            front: Entity::default(),
            value: 0.0,
        }
    }

    pub fn with_value(mut self, val: f32) -> Self {
        self.value = val.clamp(0.0, 1.0);

        self
    }
}

impl Widget for ProgressBar {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity.set_layout_type(state, LayoutType::Row);

        self.front = Element::new().build(state, entity, |builder| {
            builder.set_width(Units::Percentage(0.5)).class("front")
        });

        entity.set_element(state, "progress_bar")
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(slider_event) = event.message.downcast::<SliderEvent>() {
            match slider_event {
                SliderEvent::SetValue(val) => {
                    if event.target == entity {
                        self.value = val.clamp(0.0, 1.0);

                        self.front.set_width(state, Units::Percentage(self.value));
                    }
                }

                _ => {}
            }
        }
    }
}
