use crate::widgets::button::Button;
use crate::widgets::*;

// An element that switches between checked and unchecked when pressed / released

#[derive(Default)]
pub struct CheckButton {
    button: Button,
    checkable: Checkable,
}

impl CheckButton {
    pub fn new(checked: bool) -> Self {
        Self {
            button: Button::new().on_release(|_, state, entity|
                state.insert_event(
                    Event::new(CheckboxEvent::Switch).target(entity)
                )
            ),
            checkable: Checkable::new(checked),
        }
    }

    pub fn with_label(name: &str, checked: bool) -> Self {
        Self {
            button: Button::with_label(name).on_release(|_, state, entity|
                state.insert_event(
                    Event::new(CheckboxEvent::Switch).target(entity)
                )
            ),
            checkable: Checkable::new(checked),
        }
    }

    pub fn is_checked(&self) -> bool {
        self.checkable.is_checked()
    }

pub fn on_checked<F>(mut self, callback: F) -> Self 
where
    F: 'static + Fn(&Self, &mut State, Entity)
{
    self.checkable = self.checkable.on_checked(|_, state, entity|{
        (callback)(&self, state, entity);
    });
    self
}

    pub fn on_unchecked<F>(mut self, mut callback: F) -> Self 
    where
        F: 'static + FnMut(&mut Self, &mut State, Entity)
    {
        self.checkable = self.checkable.on_unchecked(|checkable, state, entity|{
            (callback)(&mut self, state, entity);
        });
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
