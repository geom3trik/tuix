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

// A component that can be in a checked or unchecked state
pub struct Checkable {

    button: Button,
    checked: bool,

    on_checked: Option<Event>,
    on_unchecked: Option<Event>,
}

impl Checkable {
    pub fn new(checked: bool) -> Self {
        Self {

            button: Button::new().on_release(Event::new(CheckboxEvent::Switch)),
            checked,

            on_checked: None,
            on_unchecked: None,
        }
    }

    pub fn check_on_press(mut self) -> Self {
        self.button = self.button.reset();
        self.button = self.button.on_press(Event::new(CheckboxEvent::Switch));

        self
    } 

    pub fn is_checked(&self) -> bool {
        self.checked
    }

    pub fn on_checked(mut self, event: Event) -> Self {
        self.on_checked = Some(event);
        self
    }

    pub fn on_unchecked(mut self, event: Event) -> Self {
        self.on_unchecked = Some(event);
        self
    }
}

impl BuildHandler for Checkable {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
    
        if self.checked {
            entity.set_checked(state, true);
        } else {
            entity.set_checked(state, false);
        }

        entity.set_element(state, "checkable")
    }
}

impl EventHandler for Checkable {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {

        self.button.on_event(state, entity, event);
        
        if let Some(checkbox_event) = event.message.downcast::<CheckboxEvent>() {
            match checkbox_event {
                CheckboxEvent::Switch => {
                    if event.target == entity {
                        //self.switch(state, entity);
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

                    if let Some(mut on_checked) = self.on_checked.clone() {
                        if on_checked.target == Entity::null() {
                            on_checked.target = entity;
                        }

                        on_checked.origin = entity;
                        state.insert_event(on_checked);
                    }
                }

                CheckboxEvent::Unchecked => {
                    self.checked = false;

                    entity.set_checked(state, false);

                    if let Some(mut on_unchecked) = self.on_unchecked.clone() {
                        if on_unchecked.target == Entity::null() {
                            on_unchecked.target = entity;
                        }

                        on_unchecked.origin = entity;

                        state.insert_event(on_unchecked);
                    }
                }
            }
        }
    }
}
