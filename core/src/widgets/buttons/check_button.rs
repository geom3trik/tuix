
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
            button: Button::new(),
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
        
        self.button = Button::new().on_release(Event::new(CheckboxEvent::Switch).target(entity));
        // self.button = Button::new().on_release(|button, state, entity| {
        //     state.insert_event(Event::new(CheckboxEvent::Switch).target(entity));
        //     let label_text = entity.get_text(state);
        //     state.insert_event(Event::new(DropdownEvent::SetText(label_text)).target(entity));
        // });

        self.button.on_build(state, entity);
        self.checkable.on_build(state, entity);

        entity.set_element(state, "check_button")
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        self.button.on_event(state, entity, event);
        self.checkable.on_event(state, entity, event);
    }
}