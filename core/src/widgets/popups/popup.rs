use crate::widgets::*;

#[derive(Debug, Clone, PartialEq)]
pub enum PopupEvent {
    Open,
    Close,
}

pub struct Popup {
    open: bool,
}

impl Popup {
    pub fn new() -> Self {
        Self { open: false }
    }
}

impl Widget for Popup {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
            .set_focusability(state, false)
            .set_element(state, "popup")
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(popup_event) = event.message.downcast::<PopupEvent>() {
            match popup_event {
                PopupEvent::Open => {
                    println!("Received Open Event");
                    self.open = true;
                    state.capture(entity);
                    entity.set_opacity(state, 1.0);
                }

                PopupEvent::Close => {
                    println!("Received Close Event");
                    self.open = false;
                    state.release(entity);
                    entity.set_opacity(state, 0.0);
                }
            }
        }

        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::MouseCaptureOutEvent => {
                    // state
                    //     .style
                    //     .opacity
                    //     .play_animation(self.container, self.fade_out_animation);

                    entity.set_opacity(state, 0.0);
                }

                WindowEvent::MouseCaptureEvent => {
                    // state
                    //     .style
                    //     .opacity
                    //     .play_animation(self.container, self.fade_in_animation);

                    entity.set_opacity(state, 1.0);
                    // Shouldn't need to do this but it's required for some reason. TODO: Investigate
                    //self.container.set_z_order(state, 1);
                }

                WindowEvent::MouseUp(button) => match button {
                    MouseButton::Left => {
                        if event.target == entity && event.origin != entity {
                            if state.mouse.left.pressed == state.hovered {
                                if !self.open {
                                    state.capture(entity);
                                } else {
                                    state.release(entity);
                                }

                                state.insert_event(
                                    Event::new(WindowEvent::MouseUp(*button))
                                        .target(state.hovered)
                                        .origin(entity)
                                        .propagate(Propagation::Direct),
                                );
                            }
                        }
                    }

                    _ => {}
                },

                WindowEvent::KeyDown(code, key) => match code {
                    Code::Escape => {
                        state.insert_event(Event::new(PopupEvent::Close).target(entity));
                    }

                    _ => {}
                },

                _ => {}
            }
        }
    }
}
