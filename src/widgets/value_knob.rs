use crate::state::{Entity, State};

use crate::events::{BuildHandler, Event, EventHandler, Propagation};

use crate::widgets::{Button, ControlKnob, Label, SliderEvent, Textbox, TextboxEvent};

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
pub struct ValueKnob {
    pub label: String,
    pub slider: Entity,
    pub value: Entity,

    init: f32,
    min_value: f32,
    max_value: f32,
}

impl ValueKnob {
    pub fn new(label: &str, init: f32, min: f32, max: f32) -> Self {
        ValueKnob {
            label: label.to_string(),
            slider: Entity::null(),
            value: Entity::null(),

            init,
            min_value: min,
            max_value: max,
        }
    }
}

impl BuildHandler for ValueKnob {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        let label = Label::new(&self.label).build(state, entity, |builder| {
            builder
                .set_height(Length::Pixels(25.0))
                .set_text_justify(Justify::Center)
        });
        self.slider = ControlKnob::new(self.init, self.min_value, self.max_value).build(
            state,
            entity,
            |builder| {
                builder
                    .set_width(Length::Pixels(50.0))
                    .set_height(Length::Pixels(50.0))
            },
        );

        let val_str = format!("{:.*}", 5, &self.init.to_string());
        self.value = Textbox::new(&val_str).build(state, entity, |builder| {
            builder
                .set_height(Length::Pixels(25.0))
                .set_margin_left(Length::Pixels(2.5))
                .set_margin_right(Length::Pixels(2.5))
                .set_flex_grow(1.0)
        });

        state.style.insert_element(entity, "value_knob");

        self.slider
    }
}

impl EventHandler for ValueKnob {
    fn on_event(&mut self, state: &mut State, _entity: Entity, event: &mut Event) -> bool {
        if let Some(slider_event) = event.message.downcast::<SliderEvent>() {
            match slider_event {
                SliderEvent::ValueChanged(_, val) => {
                    //println!("Slider Value Changed: {} {}", self.label, val);
                    if event.target == self.slider {
                        let val_str = format!("{:.*}", 5, &val.to_string());
                        println!("val_str: {} {}", self.label, val_str);
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
                    println!("Textbox Value Changed:{}", text);
                    if event.target == self.value {
                        if let Ok(value) = text.parse::<f32>() {
                            let val = (value.min(self.max_value)).max(self.min_value);

                            let val_str = format!("{:.*}", 5, &val.to_string());
                            state.insert_event(
                                Event::new(TextboxEvent::SetValue(val_str))
                                    .target(self.value)
                                    .propagate(Propagation::Direct),
                            );

                            state.insert_event(
                                Event::new(SliderEvent::SetValue(self.slider, val))
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
