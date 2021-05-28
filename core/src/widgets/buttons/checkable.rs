use crate::widgets::*;

// Todo - rename to CheckEvent
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CheckboxEvent {
    Check,
    Uncheck,
    Switch,
    Checked,
    Unchecked,
}

// A Widget that can be in a checked or unchecked state
#[derive(Default)]
pub struct Checkable {
    checked: bool,

    on_checked: Option<Box<dyn Fn(&Self, &mut State, Entity)>>,
    on_unchecked: Option<Box<dyn FnMut(&Self, &mut State, Entity)>>,
}

impl Checkable {
    pub fn new(checked: bool) -> Self {
        Self {
            checked,

            on_checked: None,
            on_unchecked: None,
        }
    }

    pub fn is_checked(&self) -> bool {
        self.checked
    }

    pub fn on_checked<'a, F>(mut self, callback: F) -> Self 
    where
        F: 'static + Fn(&Self, &mut State, Entity)
    {
        self.on_checked = Some(Box::new(callback));
        self
    }

    pub fn on_unchecked<F>(mut self, callback: F) -> Self 
    where
        F: 'static + FnMut(&Self, &mut State, Entity)
    {
        self.on_unchecked = Some(Box::new(callback));
        self
    }
}

impl Widget for Checkable {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        if self.checked {
            entity.set_checked(state, true);
        } else {
            entity.set_checked(state, false);
        }

        entity.set_element(state, "checkable")
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(checkbox_event) = event.message.downcast::<CheckboxEvent>() {
            match checkbox_event {
                CheckboxEvent::Switch => {
                    if event.target == entity {
                        if self.checked {
                            state.insert_event(
                                Event::new(CheckboxEvent::Unchecked)
                                    .target(entity)
                                    .origin(entity),
                            );
                        } else {
                            state.insert_event(
                                Event::new(CheckboxEvent::Checked)
                                    .target(entity)
                                    .origin(entity),
                            );
                        }
                    }
                }

                CheckboxEvent::Check => {
                    self.checked = true;
                    entity.set_checked(state, true);
                }

                CheckboxEvent::Uncheck => {
                    self.checked = false;
                    entity.set_checked(state, false);
                }

                CheckboxEvent::Checked => {
                    self.checked = true;

                    entity.set_checked(state, true);

                    if let Some(mut callback) = self.on_checked.take() {
                        (callback)(self, state, entity);
                        self.on_checked = Some(callback);
                    }
                }

                CheckboxEvent::Unchecked => {
                    self.checked = false;

                    entity.set_checked(state, false);

                    if let Some(mut callback) = self.on_unchecked.take() {
                        (callback)(self, state, entity);
                        self.on_unchecked = Some(callback);
                    }
                }
            }
        }
    }
}
