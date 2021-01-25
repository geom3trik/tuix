use crate::{Entity, Length, Style, Text};
use flume::Sender;

use std::cell::RefCell;
use std::{cell::Ref, rc::Rc};

#[derive(Clone)]
pub struct Handle {
    pub entity: Entity,
    pub(crate) style_data: Rc<RefCell<Style>>,
}

impl Handle {
    pub fn new(entity: Entity, style_data: Rc<RefCell<Style>>) -> Self {
        Self { entity, style_data }
    }
}

impl Handle {
    // CSS Selector Properties
    pub fn set_element(&self, value: &str) -> &Handle {
        self.style_data
            .borrow_mut()
            .insert_element(self.entity, value);
        self
    }

    pub fn class(&self, value: &str) -> &Handle {
        self.style_data
            .borrow_mut()
            .insert_class(self.entity, value);

        self
    }

    // CSS Pseudoclasses
    pub fn set_checked(&self) -> &Handle {
        self
    }

    pub fn set_text(&self, value: &str) -> &Handle {
        if let Some(text) = self.style_data.borrow_mut().text.get_mut(self.entity) {
            text.text = value.to_string();

            return self;
        }

        self.style_data.borrow_mut().text.insert(
            self.entity,
            Text {
                text: value.to_string(),
                ..Default::default()
            },
        );

        self
    }

    pub fn set_width(&self, value: Length) -> &Handle {
        self.style_data
            .borrow_mut()
            .width
            .insert(self.entity, value);
        self
    }

    pub fn set_height(&self, value: Length) -> &Handle {
        self.style_data
            .borrow_mut()
            .height
            .insert(self.entity, value);
        self
    }
}
