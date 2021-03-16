#![allow(dead_code)]

use crate::widgets::*;
use crate::style::*;

use crate::widgets::checkable::*;

const ICON_CHECK: &str = "\u{2713}";

//TODO
const CHECKBOX_STYLE: &str = r#"
    checkbox {
        font: icons,
        width: 20px;
        height: 20px;
        background-color: white;
        border-width: 1px;
        border-color: black;
        border-radius: 3px;
        transition: background-color 0.1 0.0;
    }

    checkbox:checked {
        background-color: #ff5e1a;
        border-color: #ff5e1a;
        transition: background-color 0.1 0.0;
    }
"#;

// A checkable with an added icon
pub struct Checkbox {

    checkbutton: CheckButton,

    icon_unchecked: Option<String>,
    icon_checked: Option<String>,
}

impl Checkbox {
    pub fn new(checked: bool) -> Self {
        Self {

            checkbutton: CheckButton::new(checked),

            icon_unchecked: Some(String::new()),
            icon_checked: Some(ICON_CHECK.to_string()),
        }
    }

    pub fn with_icon_checked(mut self, icon_checked: &str) -> Self {
        self.icon_checked = Some(icon_checked.to_string());

        self
    }

    pub fn with_icon_unchecked(mut self, icon_unchecked: &str) -> Self {
        self.icon_unchecked = Some(icon_unchecked.to_string());

        self
    }

    pub fn on_checked(mut self, event: Event) -> Self {
        self.checkbutton = self.checkbutton.on_checked(event);
        self
    }

    pub fn on_unchecked(mut self, event: Event) -> Self {
        self.checkbutton = self.checkbutton.on_unchecked(event);
        self
    }
}

impl Widget for Checkbox {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
            .set_font(state, "icons")
            .set_text_justify(state, Justify::Center)
            .set_text_align(state, Align::Center);

        if self.checkbutton.is_checked() {
            entity.set_checked(state, true);

            if let Some(icon_checked) = &self.icon_checked {
                entity.set_text(state, &icon_checked);
            }
        } else {
            entity.set_checked(state, false);

            if let Some(icon_unchecked) = &self.icon_unchecked {
                entity.set_text(state, &icon_unchecked);
            }
        }

        state.style.insert_element(entity, "checkbox");

        entity
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {

        // Inherit chackable behaviour
        self.checkbutton.on_event(state, entity, event);


        if self.checkbutton.is_checked() {
            if let Some(icon_checked) = &self.icon_checked {
                entity.set_text(state, &icon_checked);
            }
        } else {
            if let Some(icon_unchecked) = &self.icon_unchecked {
                entity.set_text(state, &icon_unchecked);
            }
        }
    }
}

pub struct CheckItem {
    name: String,
    checked: bool,

    button: Button,

    checkbox: Entity,
    label: Entity,
}

impl CheckItem {
    pub fn new(label: &str, checked: bool) -> Self {
        Self {
            name: label.to_string(),
            checked,

            button: Button::default(),

            checkbox: Entity::null(),
            label: Entity::null(),
        }
    }
}

impl Widget for CheckItem {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        
        self.checkbox = Checkbox::new(self.checked).build(state, entity, |builder| 
            builder
                .set_hoverability(false)
                .set_focusability(false)
        );
        self.label = Label::new(&self.name).build(state, entity, |builder| 
            builder
            .set_flex_grow(1.0)
            .set_hoverability(false)
            .set_focusability(false)
            .set_align_self(AlignSelf::Stretch)
            .set_margin_left(Length::Pixels(5.0))
        
        );

        self.button = Button::new().on_release(Event::new(CheckboxEvent::Switch).target(self.checkbox));

        //let checkbox = self.checkbox;
        // self.button.on_test(move |button, state, entity| {
        //     println!("Send message to checkbox");
        //     state.insert_event(Event::new(CheckboxEvent::Switch).target(checkbox))
        // });
        
        entity.set_flex_direction(state, FlexDirection::Row).set_align_items(state, AlignItems::Center);

        entity.set_element(state, "check_item")
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        self.button.on_event(state, entity, event);
    }
}