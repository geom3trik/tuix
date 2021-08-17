const ICON_CHECK: &str = "\u{2713}";

use crate::style::*;
use crate::widgets::*;

pub struct Radio {
    marker: Entity,
    check: CheckButton,
}

impl Radio {
    pub fn new() -> Self {
        Self {
            marker: Entity::null(),
            check: CheckButton::new(),
        }
    }

    pub fn set_checked(mut self, checked: bool) -> Self {
        self.check = self.check.set_checked(checked);

        self
    }

    pub fn on_checked<F>(mut self, callback: F) -> Self 
    where
        F: 'static + Fn(&mut CheckButton, &mut State, Entity)
    {
        self.check = self.check.on_checked(callback);
        self
    }

    pub fn on_unchecked<F>(mut self, callback: F) -> Self 
    where
        F: 'static + Fn(&mut CheckButton, &mut State, Entity)
    {
        self.check = self.check.on_unchecked(callback);
        self
    }
}

impl Widget for Radio {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        
        self.check.on_build(state, entity);
        
        self.marker = Element::new().build(state, entity, |builder| {
            builder
                .set_hoverable(false)
                .class("marker")
                .set_hoverable(false)
        });

        entity.set_element(state, "radio")
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        self.check.on_event(state, entity, event);
    }
}
