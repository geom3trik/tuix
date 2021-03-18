
use crate::widgets::*;
use crate::widgets::button::Button;

// An element that switches between checked and unchecked when pressed / released

pub struct CheckButton {
    button: Button,
    checkable: Checkable,
}

impl CheckButton {
    pub fn new(checked: bool) -> Self {
        Self {
            button: Button::new().on_release(Event::new(CheckboxEvent::Switch)),
            checkable: Checkable::new(checked),
        }
    }

    pub fn with_label(name: &str, checked: bool) -> Self {
        Self {
            button: Button::with_label(name).on_release(Event::new(CheckboxEvent::Switch)),
            checkable: Checkable::new(checked),
        }
    }

    pub fn is_checked(&self) -> bool {
        self.checkable.is_checked()
    }

    
    pub fn on_checked(mut self, event: Event) -> Self {
        self.checkable = self.checkable.on_checked(event);
        self
    }

    pub fn on_unchecked(mut self, event: Event) -> Self {
        self.checkable = self.checkable.on_unchecked(event);
        self
    }
}

impl Widget for CheckButton {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        
        self.button.on_build(state, entity);
        self.checkable.on_build(state, entity);

        entity.set_element(state, "check_button")
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        self.button.on_event(state, entity, event);
        self.checkable.on_event(state, entity, event);
    }
}