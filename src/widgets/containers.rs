#![allow(dead_code)]

use crate::widgets::*;

use crate::events::{BuildHandler, Event, EventHandler};
use crate::state::style::{FlexDirection, Length};
use crate::{CursorIcon, MouseButton, WindowEvent};

pub struct HBox {}

impl HBox {
    pub fn new() -> Self {
        HBox {}
    }
}

impl BuildHandler for HBox {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity.set_flex_direction(state, FlexDirection::Row);

        entity.set_element(state, "hbox");

        entity
    }
}

impl EventHandler for HBox {}

pub struct VBox {}

impl VBox {
    pub fn new() -> Self {
        VBox {}
    }
}

impl BuildHandler for VBox {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity.set_flex_direction(state, FlexDirection::Column);

        entity.set_element(state, "vbox");

        entity
    }
}

impl EventHandler for VBox {}

pub struct ResizableVBox {
    resizing: bool,
    previous_width: f32,
}

impl ResizableVBox {
    pub fn new() -> Self {
        ResizableVBox {
            resizing: false,
            previous_width: 0.0,
        }
    }
}

impl BuildHandler for ResizableVBox {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
            .set_width(state, Length::Pixels(300.0))
            .set_max_width(state, Length::Pixels(500.0))
            .set_min_width(state, Length::Pixels(300.0));
        //state.style.z_order.set(self.resize_marker, 1);

        entity
    }
}

impl EventHandler for ResizableVBox {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) -> bool {
        if let Some(window_event) = event.is_type::<WindowEvent>() {
            match window_event {
                WindowEvent::MouseDown(button) => {
                    if *button == MouseButton::Left {
                        if state.mouse.left.pos_down.0
                            >= state.transform.get_posx(entity) + state.transform.get_width(entity)
                                - 4.0
                            && state.mouse.left.pos_down.0
                                <= state.transform.get_posx(entity)
                                    + state.transform.get_width(entity)
                        {
                            self.resizing = true;
                            self.previous_width = state.transform.get_width(entity);
                            state.capture(entity);
                        }
                    }
                }

                WindowEvent::MouseUp(button) => {
                    if *button == MouseButton::Left {
                        if self.resizing == true {
                            //state.release(entity);
                            self.resizing = false;
                            state.insert_event(
                                Event::new(WindowEvent::MouseMove(
                                    state.mouse.cursorx,
                                    state.mouse.cursory,
                                ))
                                .target(entity),
                            );
                        }
                    }
                }

                // Occurs when the cursor leaves the entity
                WindowEvent::MouseOut => {
                    if !self.resizing {
                        state.insert_event(Event::new(WindowEvent::SetCursor(CursorIcon::Arrow)));
                    }
                }

                WindowEvent::MouseMove(x, y) => {
                    if self.resizing {
                        let distx = *x - state.mouse.left.pos_down.0;
                        entity.set_width(state, Length::Pixels(self.previous_width + distx));
                    } else {
                        if *x
                            > state.transform.get_posx(entity) + state.transform.get_width(entity)
                                - 4.0
                            && *x
                                < state.transform.get_posx(entity)
                                    + state.transform.get_width(entity)
                        {
                            state.insert_event(Event::new(WindowEvent::SetCursor(
                                CursorIcon::EResize,
                            )));
                        } else {
                            state.insert_event(Event::new(WindowEvent::SetCursor(
                                CursorIcon::Arrow,
                            )));
                            state.release(entity);
                        }
                    }
                }

                _ => {}
            }
        }

        false
    }
}
