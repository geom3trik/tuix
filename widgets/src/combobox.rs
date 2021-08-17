


use crate::common::*;

const ICON_DOWN_DIR: &str = "\u{25be}";


pub enum ComboBoxEvent {
    SetCurrentText(String),
    
}

pub struct ComboBox {
    header: Entity,
    label: Entity,
    popup: Entity,

    current_text: String,
    current_index: u32,

    on_index_changed: Option<Box<dyn Fn(&mut Self, &mut State, Entity)>>,
}

impl ComboBox {
    pub fn new(current_text: &str) -> Self {
        Self {
            header: Entity::null(),
            label: Entity::null(),
            popup: Entity::null(),

            current_text: current_text.to_owned(),
            current_index: 0,

            on_index_changed: None,
        }
    }
}

impl Widget for ComboBox {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        self.header = Element::new().build(state, entity, |builder| {
            builder
                //.set_background_color(Color::rgb(100,100,50))
                .set_hoverable(false)
                .set_focusable(false)
                .set_layout_type(LayoutType::Row)
                .set_width(Stretch(1.0))
                .set_height(Stretch(1.0))
                .class("header")
        });

        self.label = Label::new(&self.current_text).build(state, self.header, |builder| {
            builder
                //.set_background_color(Color::rgb(100,50,50))
                .set_hoverable(false)
                .set_focusable(false)
                .set_child_left(Pixels(10.0))
                .set_child_top(Stretch(10.0))
                .set_child_bottom(Stretch(10.0))
                .class("label")
        });

        // Icon
        Element::new().build(state, self.header, |builder| {
            builder
                .set_font("icons")
                .set_hoverable(false)
                .set_focusable(false)
                //.set_background_color(Color::rgb(100,100,100))
                .set_text(ICON_DOWN_DIR)
                .set_width(Pixels(30.0))
                .set_top(Stretch(1.0))
                .set_bottom(Stretch(1.0))
                .set_child_space(Stretch(1.0))
                .class("icon")
        });

        self.popup = Popup::new().build(state, entity, |builder| 
            builder
                .set_width(Pixels(100.0))
                .set_height(Pixels(200.0))
                .set_top(Percentage(1.0))
                .set_background_color(Color::red())
        );

        entity.set_element(state, "combobox")
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::MouseDown(button) => {
                    
                    entity.emit(state, self.popup, Event::new(PopupEvent::Switch));
                }

                _=> {}
            }
        }
    }
}

pub struct ComboBoxItem {
    text: String,
}

impl ComboBoxItem {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_owned(),
        }
    }
}

impl Widget for ComboBoxItem {
    type Ret = Entity;

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
    }
}