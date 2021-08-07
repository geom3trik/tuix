use crate::style::*;
use crate::widgets::*;

#[derive(Debug, Clone, PartialEq)]
pub enum PopupEvent {
    OpenAtCursor,
    Open,
    Close,
    Switch,
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
    type Data = ();
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
            .set_focusability(state, false)
            .set_element(state, "popup")
            .set_overflow(state, Overflow::Visible)
            .set_position_type(state, PositionType::SelfDirected)
            .set_opacity(state, 0.0)
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(popup_event) = event.message.downcast::<PopupEvent>() {
            match popup_event {

                PopupEvent::OpenAtCursor => {

                    let cursor_x = state.mouse.cursorx;
                    let cursor_y = state.mouse.cursory;

                    let width = state.data.get_width(entity);
                    let height = state.data.get_height(entity);

                    let right_edge = cursor_x + width;
                    let bottom_edge = cursor_y + height;

                    let window_width = state.data.get_width(Entity::root());
                    let window_height = state.data.get_height(Entity::root());

                    let mut new_posx = if right_edge > window_width {
                        cursor_x - width
                    } else {
                        cursor_x
                    };

                    let mut new_posy = if bottom_edge > window_height {
                        window_height - height
                    } else {
                        cursor_y
                    };

                    if new_posx < 0.0 {
                        new_posx = 0.0;
                    }

                    if new_posy < 0.0 {
                        new_posy = 0.0;
                    }

                    entity.set_left(state, Pixels(new_posx)).set_top(state, Pixels(new_posy));

                    self.open = true;
                    state.capture(entity);
                    entity.set_opacity(state, 1.0);

                }

                PopupEvent::Open => {
                    println!("Open");
                    self.open = true;
                    state.capture(entity);
                    entity.set_opacity(state, 1.0);
                }

                PopupEvent::Close => {
                    self.open = false;
                    state.release(entity);
                    entity.set_opacity(state, 0.0);
                }

                PopupEvent::Switch => {
                    if self.open {
                        self.open = false;
                        state.release(entity);
                        entity.set_opacity(state, 0.0);
                    } else {
                        self.open = true;
                        state.capture(entity);
                        entity.set_opacity(state, 1.0);
                    }
                }
            }
        }

        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::MouseCaptureOutEvent => {
                    //println!("Hide");
                    // state
                    //     .style
                    //     .opacity
                    //     .play_animation(self.container, self.fade_out_animation);

                    entity.set_opacity(state, 0.0);
                }

                WindowEvent::MouseCaptureEvent => {
                    //println!("Show");
                    // state
                    //     .style
                    //     .opacity
                    //     .play_animation(self.container, self.fade_in_animation);

                    entity.set_opacity(state, 1.0);
                    // Shouldn't need to do this but it's required for some reason. TODO: Investigate
                    //self.container.set_z_order(state, 1);
                }

                WindowEvent::MouseDown(button) => {
                    if event.target == entity && event.origin != entity {
                        if !entity.is_over(state) {
                            entity.emit(state, PopupEvent::Close);
                        
                        } else {
                            state.insert_event(
                                Event::new(WindowEvent::MouseDown(*button))
                                    .target(state.hovered)
                                    .origin(entity)
                                    .propagate(Propagation::Direct),
                            );
                        }
                    }
                    
                }

                WindowEvent::MouseUp(button) => match button {
                    MouseButton::Left => {
                        if event.target == entity && event.origin != entity {
                            if state.mouse.left.pressed == state.hovered {
                                if !self.open {
                                    state.capture(entity);
                                } else {
                                    println!("Release");
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

                WindowEvent::KeyDown(code, _) => match code {
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
