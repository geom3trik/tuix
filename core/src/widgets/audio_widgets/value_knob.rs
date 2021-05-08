use crate::widgets::*;
use crate::widgets::{ControlKnob, Label, SliderEvent, Textbox, TextboxEvent};

use crate::state::style::*;

use std::sync::{Arc, Mutex};
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

pub struct FreqValue(pub f32);

impl std::fmt::Display for FreqValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.0.abs() < 10.0 {
            write!(f, "{:.2}", self.0)
        } else if self.0.abs() >= 10.0 && self.0.abs() < 100.0 {
            write!(f, "{:.1}", self.0)
        } else if self.0.abs() >= 100.0 && self.0.abs() < 1000.0 {
            write!(f, "{:.0}", self.0)
        } else if self.0.abs() >= 1000.0 && self.0.abs() < 10000.0 {
            write!(f, "{:.2}", self.0 / 1000.0)
        } else if self.0.abs() >= 10000.0 && self.0.abs() < 100000.0 {
            write!(f, "{:.1}", self.0 / 1000.0)
        } else {
            write!(f, "{}", self.0)
        }
    }
}

impl From<f32> for FreqValue {
    fn from(src: f32) -> FreqValue {
        FreqValue(src)
    }
}

pub enum UnitsType {
    None,
    Hertz,
    dB,
}

pub struct ValueKnob {
    pub label: String,
    pub knob: Entity,
    pub textbox: Entity,

    units: UnitsType,

    init: f32,
    min: f32,
    max: f32,

    is_log: bool,

    pub on_change: Option<Arc<Mutex<dyn Fn(f32) -> Event>>>,

    pub on_changing: Option<Arc<dyn Fn(&ControlKnob, &mut State, Entity)>>,
}

impl ValueKnob {
    pub fn new(label: &str, init: f32, min: f32, max: f32) -> Self {
        ValueKnob {
            label: label.to_string(),
            knob: Entity::null(),
            textbox: Entity::null(),

            units: UnitsType::None,

            init,
            min,
            max,

            is_log: false,

            on_change: None,
            on_changing: None,
        }
    }

    pub fn with_log_scale(mut self) -> Self {
        self.is_log = true;

        self
    }

    pub fn with_units(mut self, units: UnitsType) -> Self {
        self.units = units;
        self
    }

    pub fn with_minium(mut self, min: f32) -> Self {
        self.min = min;

        self
    }

    pub fn with_maximum(mut self, min: f32) -> Self {
        self.min = min;

        self
    }

    pub fn on_change<F>(mut self, message: F) -> Self
    where
        F: Fn(f32) -> Event,
        F: 'static,
    {
        self.on_change = Some(Arc::new(Mutex::new(message)));
        self
    }

    pub fn on_changing<F>(mut self, message: F) -> Self
    where
        F: Fn(&ControlKnob, &mut State, Entity),
        F: 'static,
    {
        self.on_changing = Some(Arc::new(message));
        self
    }
}

impl Widget for ValueKnob {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {        
        Label::new(&self.label).build(state, entity, |builder| {
            builder
                .set_height(Units::Pixels(25.0))
                .set_child_space(Stretch(1.0))
                //.set_background_color(Color::green())
                .set_bottom(Pixels(5.0))
        });

        let mut knob = ControlKnob::new(self.init, self.min, self.max);

        knob.on_change = self.on_change.clone();
        knob.on_changing = self.on_changing.clone();
        knob.is_log = self.is_log;

        self.knob = knob.build(state, entity, |builder| {
            builder
                .set_width(Pixels(50.0))
                .set_height(Pixels(50.0))
                .set_left(Stretch(1.0))
                .set_right(Stretch(1.0))
        });

        //let val_str = format!("{:3}!", self.init);
        let freq_val: FreqValue = self.init.into();

        let units = match self.units {
            UnitsType::Hertz => {
                if self.init < 1000.0 {
                    " Hz"
                } else {
                    " kHz"
                }
            },

            UnitsType::dB => " dB",

            _=> ""
        };

        self.textbox = Textbox::new(&(freq_val.to_string() + units)).build(state, entity, |builder| {
            builder
                .set_height(Pixels(25.0))
                .set_left(Pixels(2.5))
                .set_right(Pixels(2.5))
                .set_child_space(Stretch(1.0))
        });

        entity
            .set_height(state, Units::Auto)
            .set_element(state, "value_knob");
            //.set_background_color(state, Color::blue());

        self.knob
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(slider_event) = event.message.downcast::<SliderEvent>() {
            match slider_event {
                SliderEvent::ValueChanged(val) => {
                    //println!("Slider Value Changed: {} {}", self.label, val);
                    if event.target == self.knob {
                        // let val_str = format!("{:3}!", val);
                        let freq_val: FreqValue = (*val).into();

                        let units = match self.units {
                            UnitsType::Hertz => {
                                if self.init < 1000.0 {
                                    " Hz"
                                } else {
                                    " kHz"
                                }
                            },
                
                            UnitsType::dB => " dB",
                
                            _=> ""
                        };

                        let new_string = freq_val.to_string() + units;

                        //println!("val_str: {} {}", self.label, val_str);
                        state.insert_event(
                            Event::new(TextboxEvent::SetValue(new_string))
                                .target(self.textbox)
                                .propagate(Propagation::Direct),
                        );
                    }
                }

                SliderEvent::SetValue(val) => {
                    if event.target == entity || event.target == self.knob {
                        let freq_val: FreqValue = (*val).into();
                        //println!("val_str: {} {}", self.label, val_str);

                        let units = match self.units {
                            UnitsType::Hertz => {
                                if self.init < 1000.0 {
                                    " Hz"
                                } else {
                                    " kHz"
                                }
                            },
                
                            UnitsType::dB => " dB",
                
                            _=> ""
                        };

                        let new_string = freq_val.to_string() + units;

                        state.insert_event(
                            Event::new(TextboxEvent::SetValue(new_string))
                                .target(self.textbox)
                                .propagate(Propagation::Direct),
                        );

                        state.insert_event(
                            Event::new(SliderEvent::SetValue(*val))
                                .target(self.knob)
                                .propagate(Propagation::Direct),
                        );
                    }
                }

                _ => {}
            }
        }

        /*
        if let Some(textbox_event) = event.message.downcast::<TextboxEvent>() {
            match textbox_event {
                TextboxEvent::ValueChanged(text) => {
                    //println!("Textbox Value Changed:{}", text);
                    if event.target == self.value {
                        if let Ok(value) = text.parse::<f32>() {
                            let val = (value.min(self.max_value)).max(self.min_value);

                            let val_str = format!("{:3}!", val);
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
        */
    }
}
