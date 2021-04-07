use crate::state::{Entity, State};

use crate::widgets::*;
use crate::widgets::{Dropdown, DropdownEvent, Item, Textbox, TextboxEvent};

use crate::state::style::*;

#[derive(Clone)]
pub struct LengthBox {
    pub value: Entity,
    pub unit: Entity,
    pub pixels: f32,
    pub percentage: f32,
    pub length_type: Units,
}

impl LengthBox {
    pub fn new() -> Self {
        LengthBox {
            value: Entity::null(),
            unit: Entity::null(),
            pixels: 0.0,
            percentage: 0.0,
            length_type: Units::Auto,
        }
    }
}

impl Widget for LengthBox {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity.set_flex_direction(state, FlexDirection::Row);

        self.value = Textbox::new("0.0").build(state, entity, |builder| {
            builder.set_flex_grow(1.0).class("value")
        });
        // self.unit = Dropdown::new("-")
        //     .add_item("Auto", "-")
        //     .add_item("px", "px")
        //     .add_item("%", "%")
        //     .add_item("Initial", "-")
        //     .build(state, entity, |builder| builder.set_flex_basis(30.0).set_text_justify(Justify::End).class("unit")).1;

        self.unit = Dropdown::new("-")
            .build(state, entity, |builder| {
                builder
                    .set_flex_basis(Units::Pixels(30.0))
                    .set_text_justify(Justify::End)
                    .class("unit")
            })
            .2;

        let _auto = Item::new("auto", "-").build(state, self.unit, |builder| builder.class("item"));
        let _pixel = Item::new("px", "px").build(state, self.unit, |builder| builder.class("item"));
        let _percentage =
            Item::new("%", "%").build(state, self.unit, |builder| builder.class("item"));
        let _initial =
            Item::new("initial", "-").build(state, self.unit, |builder| builder.class("item"));

        state.style.insert_element(entity, "length_box");

        self.value
    }

    fn on_event(&mut self, state: &mut State, _entity: Entity, event: &mut Event) {
        if let Some(dropdown_event) = event.message.downcast::<DropdownEvent>() {
            match dropdown_event {
                DropdownEvent::SetText(text) => {
                    if text == "auto" {
                        self.value.set_text(state, text);
                        self.length_type = Units::Auto;
                    }

                    if text == "stretch" {
                        self.value.set_text(state, text);
                        self.length_type = Units::Stretch(0.0);
                    }

                    if text == "px" {
                        self.value.set_text(state, &self.pixels.to_string());
                        self.length_type = Units::Pixels(0.0);
                    }

                    if text == "%" {
                        self.value.set_text(state, &self.percentage.to_string());
                        self.length_type = Units::Percentage(0.0);
                    }
                }
            }
        }

        if let Some(textbox_event) = event.message.downcast::<TextboxEvent>() {
            match textbox_event {
                TextboxEvent::ValueChanged(value) => match self.length_type {
                    Units::Pixels(_) => {
                        self.pixels = value.parse::<f32>().unwrap();
                    }

                    Units::Percentage(_) => {
                        self.percentage = value.parse::<f32>().unwrap();
                    }

                    _ => {}
                },

                _ => {}
            }
        }
    }
}
