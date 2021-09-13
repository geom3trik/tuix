
use crate::{CheckButton, Label, PopupEvent, Slider, SliderEvent, common::*};
use crate::{Dropdown, DropdownEvent, Textbox, TextboxEvent, CheckboxEvent};


#[derive(PartialEq)]
pub enum LengthBoxEvent {
    SetType(Units),
    SetValue(f32, bool),
    Reset(bool),
}


pub struct LengthBox {

    name: String,

    slider: Entity,
    textbox: Entity,
    dropdown: Entity,
    stretch: f32,
    percentage: f32,
    pixels: f32,
    units: Units,

    pixels_max: f32,

    // Dropdown Check Buttons
    check_auto: Entity,
    check_stretch: Entity,
    check_percentage: Entity,
    check_pixels: Entity,

    // Callbacks
    on_changed: Option<Box<dyn Fn(&mut Self, &mut State, Entity)>>,
}

impl LengthBox {
    pub fn new(name: &str) -> Self {
        LengthBox {

            name: name.to_owned(),

            slider: Entity::null(),
            textbox: Entity::null(),
            dropdown: Entity::null(),
            stretch: 1.0,
            percentage: 0.0,
            pixels: 0.0,
            units: Units::Auto,

            pixels_max: 20.0,

            check_auto: Entity::null(),
            check_stretch: Entity::null(),
            check_percentage: Entity::null(),
            check_pixels: Entity::null(),

            on_changed: None, 
        }
    }

    pub fn with_init(mut self, init: Units) -> Self {
        self.units = init;

        match self.units {
            Units::Stretch(val) => {
                self.stretch = val;
            }

            Units::Pixels(val) => {
                self.pixels = val;
            }

            Units::Percentage(val) => {
                self.percentage = val;
            }

            _=> {}
        }

        self
    }

    pub fn on_changed<F>(mut self, callback: F) -> Self 
    where F: 'static + Fn(&mut Self, &mut State, Entity)
    {
        self.on_changed = Some(Box::new(callback));

        self
    }

    pub fn with_pixels_max(mut self, max: f32) -> Self {
        self.pixels_max = max;

        self
    }

    pub fn value(&self) -> Units {
        match self.units {
            Units::Auto => {
                Units::Auto
            }

            Units::Stretch(_) => {
                Units::Stretch(self.stretch)
            }

            Units::Percentage(_) => {
                Units::Percentage(self.percentage)
            }

            Units::Pixels(_) => {
                Units::Pixels(self.pixels)
            }
        }
    }
}

impl Widget for LengthBox {
    type Ret = Entity;
    type Data = Units;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity.set_layout_type(state, LayoutType::Row).set_col_between(state, Pixels(10.0));


        println!("BUILD");

        self.slider = Slider::new()
            .with_min(0.0)
            .with_max(self.pixels_max)
            .on_changing(|data, state, slider|{
                slider.emit(state, LengthBoxEvent::SetValue(data.value, true));
            })
            .build(state, entity, |builder| 
                builder
                    .set_child_top(Auto)
                    .set_child_bottom(Auto)
            );
        
        Label::new(&self.name).build(state, self.slider, |builder|
            builder
                .set_position_type(PositionType::SelfDirected)
                .set_child_top(Stretch(1.0))
                .set_child_bottom(Stretch(1.0))
                .set_child_left(Pixels(5.0))
                .set_hoverable(false)
                .set_focusable(false)
        );

        self.textbox = Textbox::new("auto")
        .on_submit(|data, state, textbox|{
            if &data.text == "auto" {
                textbox.emit(state, LengthBoxEvent::SetType(Units::Auto));
                textbox.emit(state, LengthBoxEvent::Reset(true));
                
            } else {
                if let Some(last_char) = data.text.chars().last() {
                    let mut val_text = data.text.as_ref();
                    let mut val_type = Units::Auto;
                    match last_char {
                        's' => {
                            val_text = &data.text[0..data.text.len()-1];
                            val_type = Units::Stretch(0.0);
                        }
    
                        '%' => {
                            val_text = &data.text[0..data.text.len()-1];
                            val_type = Units::Percentage(0.0);
                        }
    
                        'x' => {
                            if let Some(previous_char) = data.text[0..data.text.len()-1].chars().last() {
                                if previous_char == 'p' {
                                    val_text = &data.text[0..data.text.len()-2];
                                    val_type = Units::Pixels(0.0);
                                }
                            }
                        }
    
                        _=> {},
                    }

                    //println!("{} {:?}", val_text, val_type);
    
                    if let Ok(val) = val_text.parse::<f32>() {
                        if val_type != Units::Auto {
                            textbox.emit(state, LengthBoxEvent::SetType(val_type));
                        }
                        textbox.emit(state, LengthBoxEvent::SetValue(val, true));
                    } else {
                        textbox.emit(state, LengthBoxEvent::Reset(true));
                    }
                } else {
                    textbox.emit(state, LengthBoxEvent::Reset(true));
                }
            }            
        })
        .build(state, entity, |builder| 
            builder
                .set_width(Pixels(80.0))
                .set_child_space(Stretch(1.0))
        );

        self.dropdown = Dropdown::new("auto")
            .build(state, entity, |builder| {
                builder
                    .set_width(Pixels(80.0))
                    .set_child_left(Pixels(0.0))
                    .set_child_right(Pixels(0.0))
                    .class("unit")
            });

        self.dropdown.set_width(state, Pixels(80.0));


        
        // Spacer
        // Element::new().build(state, self.dropdown, |builder| 
        //     builder
        //         .set_height(Pixels(5.0))
        // );

        self.check_auto = CheckButton::with_label("auto")
            //.set_checked(true)
            .on_checked(|_, state, button|{
                println!("This thing here: {}", button);
                button.emit(state, LengthBoxEvent::SetType(Units::Auto));
                button.emit(state, LengthBoxEvent::Reset(true));
                button.emit(state, PopupEvent::Close);
                button.emit(state, DropdownEvent::SetText("auto".to_string()));
            })
            .build(state, self.dropdown, |builder| 
                builder
                    //.set_color(Color::black())
            );

        entity.emit_to(state, self.check_auto, CheckboxEvent::Check);

        self.check_stretch = CheckButton::with_label("stretch")
            .on_checked(|_, state, button|{
                button.emit(state, LengthBoxEvent::SetType(Units::Stretch(0.0)));
                button.emit(state, LengthBoxEvent::Reset(true));
                button.emit(state, PopupEvent::Close);
                button.emit(state, DropdownEvent::SetText("stretch".to_string()));
            })
            .build(state, self.dropdown, |builder| 
                builder
                    //.set_color(Color::black())
            );

        self.check_percentage = CheckButton::with_label("%")
            .on_checked(|_, state, button|{
                button.emit(state, LengthBoxEvent::SetType(Units::Percentage(0.0)));
                button.emit(state, LengthBoxEvent::Reset(true));
                button.emit(state, PopupEvent::Close);
                button.emit(state, DropdownEvent::SetText("%".to_string()));
            })
            .build(state, self.dropdown, |builder| 
                builder
                    //.set_color(Color::black())
            );

        self.check_pixels = CheckButton::with_label("px")
            .on_checked(|_, state, button|{
                button.emit(state, LengthBoxEvent::SetType(Units::Pixels(0.0)));
                button.emit(state, LengthBoxEvent::Reset(true));
                button.emit(state, PopupEvent::Close);
                button.emit(state, DropdownEvent::SetText("px".to_string()));
            })
            .build(state, self.dropdown, |builder| 
                builder
                    //.set_color(Color::black())
            );


        // Spacer
        // Element::new().build(state, self.dropdown, |builder| 
        //     builder
        //         .set_height(Pixels(5.0))
        // );

        entity.set_element(state, "length_box");

        entity
    }


    fn on_update(&mut self, state: &mut State, entity: Entity, data: &Self::Data) {
        
        //println!("UPDATE");

        if *data != self.value() {
            //println!("Data: {:?} {:?}", entity, data);
            entity.emit(state, LengthBoxEvent::SetType(*data));
            match data {
                Units::Auto => {
                    entity.emit_to(state, entity, LengthBoxEvent::SetValue(0.0, false));
                }
                Units::Stretch(val) | Units::Percentage(val) | Units::Pixels(val) => {
                    entity.emit_to(state, entity, LengthBoxEvent::SetValue(*val, false));
                }


            }
        }

    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(lengthbox_event) = event.message.downcast() {
            match lengthbox_event {

                LengthBoxEvent::SetValue(val, flag) => {
                    match self.units {
                        Units::Auto => {
                            entity.emit_to(state, self.dropdown, DropdownEvent::SetText("-".to_string()));
                            entity.emit_to(state, self.textbox, TextboxEvent::SetValue("auto".to_string()));
                            entity.emit_to(state, self.slider, SliderEvent::SetValue(0.0));
                            self.check_auto.emit(state, CheckboxEvent::Check);
                            self.check_auto.emit(state, DropdownEvent::SetText("auto".to_string()));
                            self.slider.set_disabled(state, true);

                        }

                        Units::Stretch(_) => {
                            self.stretch = *val;
                            entity.emit_to(state, self.dropdown, DropdownEvent::SetText("stretch".to_string()));
                            entity.emit_to(state, self.textbox, TextboxEvent::SetValue(format!("{:.0}s", val)));
                            entity.emit_to(state, self.slider, SliderEvent::SetMax(10.0));
                            entity.emit_to(state, self.slider, SliderEvent::SetValue(*val));
                            self.check_stretch.emit(state, CheckboxEvent::Check);
                            self.check_stretch.emit(state, DropdownEvent::SetText("stretch".to_string()));
                            self.slider.set_disabled(state, false);

                        }

                        Units::Percentage(_) => {
                            self.percentage = *val;
                            entity.emit_to(state, self.dropdown, DropdownEvent::SetText("%".to_string()));
                            entity.emit_to(state, self.textbox, TextboxEvent::SetValue(format!("{:.0}%", val)));
                            entity.emit_to(state, self.slider, SliderEvent::SetMax(100.0));
                            entity.emit_to(state, self.slider, SliderEvent::SetValue(*val));
                            self.check_percentage.emit(state, CheckboxEvent::Check);
                            self.check_percentage.emit(state, DropdownEvent::SetText("%".to_string()));
                            self.slider.set_disabled(state, false);

                        }

                        Units::Pixels(_) => {
                            self.pixels = *val;
                            entity.emit_to(state, self.dropdown, DropdownEvent::SetText("px".to_string()));
                            entity.emit_to(state, self.textbox, TextboxEvent::SetValue(format!("{:.0}px", val)));
                            entity.emit_to(state, self.slider, SliderEvent::SetMax(self.pixels_max));
                            entity.emit_to(state, self.slider, SliderEvent::SetValue(*val));
                            self.check_pixels.emit(state, CheckboxEvent::Check);
                            self.check_pixels.emit(state, DropdownEvent::SetText("px".to_string()));
                            self.slider.set_disabled(state, false);
                        }
                    }
                    if *flag {
                        if let Some(callback) = self.on_changed.take() {
                            (callback)(self, state, entity);
                            self.on_changed = Some(callback);
                        }
                    }

                    event.consume();
                }

                LengthBoxEvent::SetType(units) => {
                    //println!("Do This: {} {:?} {:?}", entity, units, event.origin);
                    self.units = *units;

                    event.consume();
                }

                LengthBoxEvent::Reset(flag) => {
                    match self.units {
                        Units::Auto => {
                            entity.emit_to(state, entity, LengthBoxEvent::SetValue(0.0, *flag));
                        }

                        Units::Stretch(_) => {
                            entity.emit_to(state, entity, LengthBoxEvent::SetValue(self.stretch, *flag));
                        }

                        Units::Percentage(_) => {
                            entity.emit_to(state, entity, LengthBoxEvent::SetValue(self.percentage, *flag));
                        }

                        Units::Pixels(_) => {
                            entity.emit_to(state, entity, LengthBoxEvent::SetValue(self.pixels, *flag));
                        }
                    }

                    event.consume();
                }
            }
        }
    }
}
