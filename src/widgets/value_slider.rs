use crate::state::{Entity, State};

use crate::events::{BuildHandler, Event, EventHandler, Propagation};

use crate::widgets::{Button, Slider, SliderEvent, Textbox, TextboxEvent};

use crate::state::style::*;

// const VALUE_SLIDER_STYLE: &str = r#"

//     slider
//     {
//         background-color: #2E2E2E;
//     }

//     slider > .front {
//         background-color: #494949;
//     }

//     slider > .front:hover {
//         background-color: #6D6D6D;
//     }

//     slider:active > .front {
//         background-color: #6D6D6D;
//     }

//     slider:hover > .front {
//         background-color: #6D6D6D;
//     }
// "#;

#[derive(Clone)]
pub struct ValueSlider {
    pub slider: Entity,
    pub value: Entity,
    pub label: Entity,
    label_text: String,
}

impl ValueSlider {
    pub fn new(label_txt: &str) -> Self {
        ValueSlider {
            slider: Entity::null(),
            value: Entity::null(),
            label: Entity::null(),
            label_text: label_txt.to_string(),
        }
    }
}

impl BuildHandler for ValueSlider {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
            .set_display(state, Display::Flexbox)
            .set_flex_direction(state, FlexDirection::Row);

        self.slider = Slider::new().build(state, entity, |builder| builder.set_flex_grow(1.0));
        self.value = Textbox::new("0.5").build(state, entity, |builder| {
            builder
                .set_flex_basis(50.0)
                .set_margin_left(Length::Pixels(5.0))
        });
        self.label = Button::new().build(state, self.slider, |builder| {
            builder
                .set_width(Length::Percentage(1.0))
                .set_height(Length::Percentage(1.0))
                .set_position(Position::Absolute)
                .set_text(&self.label_text)
                .set_padding_left(Length::Pixels(5.0))
                .set_hoverability(false)
        });

        state.style.insert_element(entity, "value_slider");

        entity
    }
}

impl EventHandler for ValueSlider {
    fn on_event(&mut self, state: &mut State, _entity: Entity, event: &mut Event) -> bool {
        if let Some(slider_event) = event.message.downcast::<SliderEvent>() {
            match slider_event {
                SliderEvent::ValueChanged(val) => {
                    if event.target == self.slider {
                        let val_str = format!("{:.*}", 5, &val.to_string());
                        state.insert_event(
                            Event::new(TextboxEvent::SetValue(val_str))
                                .target(self.value)
                                .propagate(Propagation::Direct),
                        );
                    }
                }

                _ => {}
            }
        }

        if let Some(textbox_event) = event.message.downcast::<TextboxEvent>() {
            match textbox_event {
                TextboxEvent::ValueChanged(text) => {
                    if event.target == self.value {
                        if let Ok(value) = text.parse::<f32>() {
                            let mut val = value;
                            if val <= 0.0 {
                                val = 0.0;
                            }
                            if val >= 1.0 {
                                val = 1.0;
                            }

                            let val_str = format!("{:.*}", 5, &val.to_string());
                            state.insert_event(
                                Event::new(TextboxEvent::SetValue(val_str))
                                    .target(self.value)
                                    .propagate(Propagation::Direct),
                            );

                            state.insert_event(
                                Event::new(SliderEvent::SetValue(val))
                                    .target(self.slider)
                                    .propagate(Propagation::Direct),
                            );
                        } else {
                            state.insert_event(
                                Event::new(TextboxEvent::ResetValue)
                                    .target(self.value)
                                    .propagate(Propagation::Direct),
                            );
                        }
                    }
                }

                _ => {}
            }
        }

        false
    }
}
