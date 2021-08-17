#![allow(dead_code)]

use crate::common::*;
use crate::ButtonEvent;

const ICON_CHECK: &str = "\u{2713}";

//TODO
const CHECKBOX_STYLE: &str = r#"
    checkbox {
        font: icons,
        width: 20px;
        height: 20px;
        background-color: white;
        border-width: 1px;
        border-color: black;
        border-radius: 3px;
        transition: background-color 0.1 0.0;
    }

    checkbox:checked {
        background-color: #ff5e1a;
        border-color: #ff5e1a;
        transition: background-color 0.1 0.0;
    }
"#;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CheckboxEvent {
    Check,
    Uncheck,
    Switch,
    Checked,
    Unchecked,
}

pub struct Atom<T> {
    key: &'static str,
    default: T,
}

impl<T> Atom<T> {
    pub const fn new(key: &'static str, default: T) -> Self {
        Self {
            key,
            default,
        }
    }
}

#[derive(Default)]
pub struct Checkbox {
    icon_unchecked: Option<String>,
    icon_checked: Option<String>,

    checked: bool,

    on_checked: Option<Box<dyn Fn(&mut Self, &mut State, Entity)>>,
    on_unchecked: Option<Box<dyn Fn(&mut Self, &mut State, Entity)>>,

    key: Code,
}

impl Checkbox {
    pub fn new(checked: bool) -> Self {
        Self {
            icon_unchecked: Some(String::new()),
            icon_checked: Some(ICON_CHECK.to_string()),
        
            on_checked: None,
            on_unchecked: None,

            checked,

            key: Code::Space,
        }
    }

    pub fn bind<T>(self, state: &mut State, atom: Atom<T>) -> Self {
        self
    }

    pub fn with_icon_checked(mut self, icon_checked: &str) -> Self {
        self.icon_checked = Some(icon_checked.to_string());

        self
    }

    pub fn with_icon_unchecked(mut self, icon_unchecked: &str) -> Self {
        self.icon_unchecked = Some(icon_unchecked.to_string());

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

impl Widget for Checkbox {
    type Ret = Entity;
    type Data = bool;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
            .set_font(state, "icons")
            .set_child_space(state, Stretch(1.0));

        if self.checked {
            entity.set_checked(state, true);

            if let Some(icon_checked) = &self.icon_checked {
                entity.set_text(state, &icon_checked);
            }

            state.insert_event(
                Event::new(CheckboxEvent::Checked)
                    .target(entity)
                    .origin(entity),
            );

        } else {
            entity.set_checked(state, false);

            if let Some(icon_unchecked) = &self.icon_unchecked {
                entity.set_text(state, &icon_unchecked);
            }

            state.insert_event(
                Event::new(CheckboxEvent::Unchecked)
                    .target(entity)
                    .origin(entity),
            );
        }

        entity.set_element(state, "checkbox")
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        
        if let Some(checkbox_event) = event.message.downcast::<CheckboxEvent>() {
            match checkbox_event {
                CheckboxEvent::Switch => {
                    if event.target == entity {
                        if self.checked {

                            entity.set_checked(state, false);

                            if let Some(icon_unchecked) = &self.icon_unchecked {
                                entity.set_text(state, &icon_unchecked);
                            }

                            state.insert_event(
                                Event::new(CheckboxEvent::Unchecked)
                                    .target(entity)
                                    .origin(entity),
                            );
                        } else {

                            entity.set_checked(state, true);

                            if let Some(icon_checked) = &self.icon_checked {
                                entity.set_text(state, &icon_checked);
                            }

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
        
        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                    if entity == event.target && !entity.is_disabled(state) {
                        state.capture(entity);
                    }
                }

                WindowEvent::MouseUp(button) if *button == MouseButton::Left => {
                    if entity == event.target && state.mouse.left.pressed == entity {
                        state.release(entity);
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

/*
pub struct CheckItem {
    name: String,
    checked: bool,

    button: Button,

    checkbox: Entity,
    label: Entity,
}

impl CheckItem {
    pub fn new(label: &str, checked: bool) -> Self {
        Self {
            name: label.to_string(),
            checked,

            button: Button::default(),

            checkbox: Entity::null(),
            label: Entity::null(),
        }
    }
}

impl Widget for CheckItem {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        self.checkbox = Checkbox::new(self.checked).build(state, entity, |builder| {
            builder.set_hoverable(false).set_focusable(false)
        });
        self.label = Label::new(&self.name).build(state, entity, |builder| {
            builder
                .set_hoverable(false)
                .set_focusable(false)
                .set_left(Pixels(5.0))
        });

        let checkbox = self.checkbox;
        self.button =
            Button::new().on_release(move |_, state, entity|
                state.insert_event(
                    Event::new(CheckboxEvent::Switch).target(checkbox).target(entity)
                )
            );

        //let checkbox = self.checkbox;
        // self.button.on_test(move |button, state, entity| {
        //     println!("Send message to checkbox");
        //     state.insert_event(Event::new(CheckboxEvent::Switch).target(checkbox))
        // });

        entity.set_layout_type(state, LayoutType::Row);

        entity.set_element(state, "check_item")
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        self.button.on_event(state, entity, event);
    }
}
*/
