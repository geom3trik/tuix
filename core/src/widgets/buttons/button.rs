use crate::widgets::*;

/// Events sent when interacting with a button and events to set properties of a button
#[derive(Debug, Clone, PartialEq)]
pub enum ButtonEvent {
    /// Emitted by a button when the button is pressed
    Pressed,
    /// Emitted by a button when the button is released
    Released,
    /// Received by the button and triggers the on_press event to be emitted
    Press,
    /// Received by the button and triggers the on_release event to be emitted
    Release,
    /// Sets the label of the button
    SetLabel(String),
    /// Sets the keycode for triggering the button with the keyboard
    SetKey(Code),
}

#[derive(Default)]
/// A basic button widget with an optional label.
///
/// A Widget that can be pressed and released and may emit an event on_press and on_release.
/// The button can also be triggered with a keyboard key (default space bar).
///
/// # Example
/// Create a button which closes the window when pressed:
/// ```
/// Button::new()
///     .on_press(|_, state, button| {
///         button.emit(state, WindowEvent::CloseWindow);
///     })
///    .build(state, parent, |builder| builder);
/// ```
pub struct Button {
    on_press: Option<Box<dyn Fn(&mut Self, &mut State, Entity)>>,
    on_release: Option<Box<dyn Fn(&mut Self, &mut State, Entity)>>,
    pub text: Option<String>,
    key: Code,
}

impl Button {
    /// Create a new button widget with no callbacks.
    ///
    /// # Example
    /// ```
    /// Button::new().build(state, parent, |builder| builder);
    /// ```
    pub fn new() -> Self {
        Button {
            on_press: None,
            on_release: None,
            text: None,
            key: Code::Space,
        }
    }

    /// Create a new button widget with a specified text label.
    ///
    /// # Example
    /// ```
    /// Button::with_label("A Button").build(state, parent, |builder| builder);
    /// ```
    pub fn with_label(text: &str) -> Self {
        Button {
            on_press: None,
            on_release: None,
            text: Some(text.to_string()),
            key: Code::Space,
        }
    }

    /// Set the callback triggered when the button is pressed.
    ///
    /// # Example
    /// Creates a button which closes the window when pressed:
    /// ```
    /// Button::new()
    ///     .on_press(|_, state, button| {
    ///         button.emit(state, WindowEvent::CloseWindow);
    ///     })
    ///    .build(state, parent, |builder| builder);
    /// ```
    pub fn on_press<F>(mut self, callback: F) -> Self 
    where
        F: 'static + Fn(&mut Self, &mut State, Entity)
    {
        self.on_press = Some(Box::new(callback));
        self
    }

    /// Set the callback triggered when the button is released.
    ///
    /// # Example
    /// Create a button which closes the window when released:
    /// ```
    /// Button::new()
    ///     .on_release(|_, state, button| {
    ///         button.emit(state, WindowEvent::CloseWindow);
    ///     })
    ///    .build(state, parent, |builder| builder);
    /// ```
    pub fn on_release<F>(mut self, callback: F) -> Self 
    where
        F: 'static + Fn(&mut Self, &mut State, Entity)
    {
        self.on_release = Some(Box::new(callback));
        self
    }

    /// Set the keyboard key which triggers the button.
    ///
    /// # Example
    /// Triggers the button's on_press callback when the Enter key is pressed and the button has focus.
    /// ```
    /// Button::new()
    ///     .with_key(Code::Enter)
    ///    .build(state, parent, |builder| builder);
    /// ```
    pub fn with_key(mut self, key: Code) -> Self {
        self.key = key;

        self
    }

    // Resets the stored events to None
    // pub fn reset(mut self) -> Self {
    //     self.on_press = None;
    //     self.on_release = None;

    //     self
    // }
}

impl Widget for Button {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        // If there is a specified label then set the text of the button entity to this
        if let Some(text) = &self.text {
            entity.set_text(state, text);
        }

        entity.set_element(state, "button")
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(button_event) = event.message.downcast::<ButtonEvent>() {
            match button_event {
                ButtonEvent::SetLabel(label) => {
                    entity.set_text(state, label);
                }

                ButtonEvent::SetKey(key) => {
                    self.key = *key;
                }

                ButtonEvent::Pressed => {
                    if event.target == entity {
                        if let Some(callback) = self.on_press.take() {
                            (callback)(self, state, entity);
                            self.on_press = Some(callback);
                        }

                        entity.set_active(state, true);
                    }
                }

                ButtonEvent::Released => {
                    if event.target == entity {
                        if let Some(callback) = self.on_release.take() {
                            (callback)(self, state, entity);
                            self.on_release = Some(callback);
                        }

                        entity.set_active(state, false);
                    }
                }

                ButtonEvent::Press => {
                    state.insert_event(
                        Event::new(ButtonEvent::Pressed)
                            .target(entity)
                            .propagate(Propagation::Direct),
                    );
                }

                ButtonEvent::Release => {
                    state.insert_event(
                        Event::new(ButtonEvent::Released)
                            .target(entity)
                            .propagate(Propagation::Direct),
                    );
                }
            }
        }

        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                    if entity == event.target && !entity.is_disabled(state) {
                        state.capture(entity);
                        state.insert_event(
                            Event::new(ButtonEvent::Pressed)
                                .target(entity)
                                .origin(entity),
                        );
                    }
                }

                WindowEvent::MouseUp(button) if *button == MouseButton::Left => {
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
