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
pub struct Checkable {
    checked: bool,

    on_checked: Option<Event>,
    on_unchecked: Option<Event>,
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

    pub fn on_checked(mut self, event: Event) -> Self {
        self.on_checked = Some(event);
        self
    }

    pub fn on_unchecked(mut self, event: Event) -> Self {
        self.on_unchecked = Some(event);
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
                        //self.switch(state, entity);
                        if self.checked {
                            println!("Send unchecked");
                            state.insert_event(
                                Event::new(CheckboxEvent::Unchecked)
                                    .target(entity)
                                    .origin(entity),
                            );
                        } else {
                            let check_event = Event::new(CheckboxEvent::Checked)
                                .target(entity)
                                .origin(entity);
                            println!("Send checked: {:?}", check_event);
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
