#![allow(dead_code)]


use crate::widgets::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ButtonEvent {
    // Emitted by a button when the button is pressed
    Pressed,
    // Emitted by a button when the button is released
    Released,
    // Received by the button and triggers the on_press event to be emitted
    Press,
    // Received by the button and triggers the on_release event to be emitted
    Release,
}


#[derive(Default)]
// A component that can be pressed and released and may emit an event on_press and on_release
pub struct Button {
    on_press: Option<Event>,
    on_release: Option<Event>,
    pub text: Option<String>,
}

impl Button {
    /// Create a new button component
    pub fn new() -> Self {
        Button {
            on_press: None,
            on_release: None,
            text: None,
        }
    }

    /// Create a new button component with a specified text label
    pub fn with_label(text: &str) -> Self {
        Button {
            on_press: None,
            on_release: None,
            text: Some(text.to_string()),
        }
    }

    /// Specifies the event that should be emitted when the button is pressed
    pub fn on_press(mut self, event: Event) -> Self {
        self.on_press = Some(event);
        self
    }

    /// Specifies the event that should be emitted when the button is released
    pub fn on_release(mut self, event: Event) -> Self {
        self.on_release = Some(event);
        self
    }

    /// Resets the stored events
    pub fn reset(mut self) -> Self {
        self.on_press = None;
        self.on_release = None;

        self
    }
}

impl BuildHandler for Button {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        // If there is a specified label then set the text of the button entity to this
        if let Some(text) = &self.text {
            entity.set_text(state, text);
        }

        // Set the element name to 'button'
        entity.set_element(state, "button")
    }
}

impl EventHandler for Button {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        
        if let Some(button_event) = event.message.downcast::<ButtonEvent>() {
            match button_event {
                ButtonEvent::Pressed => {
                    if event.target == entity {
                        if let Some(mut on_press) = self.on_press.clone() {
                            if on_press.target == Entity::null() {
                                on_press.target = entity;
                            }

                            on_press.origin = entity;
                            on_press.propagation = Propagation::Down;

                            state.insert_event(on_press);
                        }

                        entity.set_active(state, true);                        
                    }

                }

                ButtonEvent::Released => {
                    if event.target == entity {
                        if let Some(mut on_release) = self.on_release.clone() {
                            if on_release.target == Entity::default() {
                                on_release.target = entity;
                            }

                            on_release.origin = entity;
                            on_release.propagation = Propagation::Down;
                            state.insert_event(on_release);
                        }

                        entity.set_active(state, false);                        
                    }

                }

                ButtonEvent::Press => {
                    state.insert_event(Event::new(ButtonEvent::Pressed).target(entity).propagate(Propagation::Direct));
                }

                ButtonEvent::Release => {
                    state.insert_event(Event::new(ButtonEvent::Released).target(entity).propagate(Propagation::Direct));
                }

                _ => {}
            }
        }

        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::MouseDown(button) => match button {
                    MouseButton::Left => {
                        if entity == event.target && !entity.is_disabled(state) {
                            
                            state.capture(entity);
                            state.insert_event(
                                Event::new(ButtonEvent::Pressed)
                                    .target(entity)
                                    .origin(entity),
                            );
                        }
                    }

                    _ => {}
                },

                WindowEvent::MouseUp(button) => match button {
                    MouseButton::Left => {
                        if entity == event.target && state.mouse.left.pressed == entity {
                            state.release(entity);
                            entity.set_active(state, false);
                            if !entity.is_disabled(state) {
                                if state.hovered == entity {
                                    state.insert_event(
                                        Event::new(ButtonEvent::Released)
                                            .target(entity)
                                            .origin(entity),
                                    );
                                }
                            }
                        }
                    }

                    _ => {}
                },

                WindowEvent::KeyDown(code, _) => match code {
                    Code::Space => {
                        if state.focused == entity && !entity.is_disabled(state) {
                            state.insert_event(
                                Event::new(ButtonEvent::Pressed)
                                    .target(entity)
                                    .origin(entity),
                            );
                        }
                    }

                    _ => {}
                },

                WindowEvent::KeyUp(code, _) => match code {
                    Code::Space => {
                        state.insert_event(
                            Event::new(ButtonEvent::Released)
                                .target(entity)
                                .origin(entity),
                        );
                    }

                    _ => {}
                },

                _ => {}
            }
        }
    }
}
