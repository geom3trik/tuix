use crate::common::*;
use crate::{CheckboxEvent, ButtonEvent};
// An element that switches between checked and unchecked when pressed / released

#[derive(Default)]
pub struct CheckButton {
    checked: bool,

    on_checked: Option<Box<dyn Fn(&mut Self, &mut State, Entity)>>,
    on_unchecked: Option<Box<dyn Fn(&mut Self, &mut State, Entity)>>,

    pub text: Option<String>,
    
    key: Code,
}

impl CheckButton {
    pub fn new() -> Self {
        Self {
            checked: false,

            on_checked: None,
            on_unchecked: None,

            text: None,

            key: Code::Space,
        }
    }

    pub fn with_label(name: &str) -> Self {
        Self {
            checked: false,

            on_checked: None,
            on_unchecked: None,

            text: Some(name.to_string()),

            key: Code::Space,
        }
    }

    pub fn set_checked(mut self, checked: bool) -> Self {
        self.checked = checked;

        self
    }

    pub fn on_checked<F>(mut self, callback: F) -> Self 
    where
        F: 'static + Fn(&mut Self, &mut State, Entity)
    {
        self.on_checked = Some(Box::new(callback));

        self
    }

    pub fn on_unchecked<F>(mut self, callback: F) -> Self 
    where
        F: 'static + Fn(&mut Self, &mut State, Entity)
    {
        self.on_unchecked = Some(Box::new(callback));

        self
    }
}

impl Widget for CheckButton {
    type Ret = Entity;
    type Data<'a> = &'a bool;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {


        if let Some(text) = &self.text {
            entity.set_text(state, text);
        }

        if self.checked {
            entity.emit(state, CheckboxEvent::Checked);
        } else {
            entity.emit(state, CheckboxEvent::Unchecked);
        }
        

        entity.set_element(state, "check_button")
    }

    fn on_update<'a>(&mut self, state: &mut State, entity: Entity, data: &Self::Data<'a>) {
        if **data != self.checked {
            if **data {
                entity.emit(state, CheckboxEvent::Checked);
            } else {
                entity.emit(state, CheckboxEvent::Unchecked);
            }
        }
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
     
        if let Some(checkbox_event) = event.message.downcast::<CheckboxEvent>() {
            match checkbox_event {
                CheckboxEvent::Switch => {
                    if event.target == entity {
                        if self.checked {

                            //entity.set_checked(state, false);

                            state.insert_event(
                                Event::new(CheckboxEvent::Unchecked)
                                    .propagate(Propagation::Up)
                                    .target(entity)
                                    .origin(entity),
                            );
                        } else {

                            //entity.set_checked(state, true);

                            state.insert_event(
                                Event::new(CheckboxEvent::Checked)
                                    .propagate(Propagation::Up)
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
                    println!("Checked {} {}",entity,  event.origin);
                    self.checked = true;

                    entity.set_checked(state, true);

                    if let Some(callback) = self.on_checked.take() {
                        (callback)(self, state, entity);
                        self.on_checked = Some(callback);
                    }
                }

                CheckboxEvent::Unchecked => {
                    self.checked = false;

                    entity.set_checked(state, false);

                    if let Some(callback) = self.on_unchecked.take() {
                        (callback)(self, state, entity);
                        self.on_unchecked = Some(callback);
                    }
                }
            }
        }
        
        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                    if entity == event.target && !entity.is_disabled(state) {
                        //state.capture(entity);
                    }
                }

                WindowEvent::MouseUp(button) if *button == MouseButton::Left => {
                    if entity == event.target && state.mouse.left.pressed == entity {
                        //state.release(entity);
                        entity.set_active(state, false);
                        if !entity.is_disabled(state) {
                            if state.hovered == entity {
                                state.insert_event(
                                    Event::new(CheckboxEvent::Switch)
                                        .target(entity)
                                        .origin(entity),
                                );
                            }
                        }
                    }
                }

                WindowEvent::KeyDown(code, _) if *code == self.key => {
                    if state.focused == entity && !entity.is_disabled(state) {
                        state.insert_event(
                            Event::new(ButtonEvent::Pressed)
                                .target(entity)
                                .origin(entity),
                        );

                        state.insert_event(
                            Event::new(CheckboxEvent::Switch)
                                .target(entity)
                                .origin(entity),
                        );
                    }
                }

                WindowEvent::KeyUp(code, _) if *code == self.key => {
                    state.insert_event(
                        Event::new(ButtonEvent::Released)
                            .target(entity)
                            .origin(entity),
                    );
                }

                _ => {}
            }
        }

    }
}
