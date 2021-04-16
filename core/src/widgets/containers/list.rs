#![allow(dead_code)]

use crate::style::*;
use crate::widgets::*;
use crate::CursorIcon;

pub struct HBox {}

impl HBox {
    pub fn new() -> Self {
        HBox {}
    }
}

impl Widget for HBox {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
            .set_flex_direction(state, FlexDirection::Row)
            .set_layout_type(state, LayoutType::Horizontal)
            .set_focusability(state, false)
            .set_element(state, "hbox")
            .set_width(state, Auto)
            .set_height(state, Auto)
            //.set_child_top(state, Stretch(1.0))
            //.set_child_bottom(state, Stretch(1.0))
    }
}

pub struct VBox {}

impl VBox {
    pub fn new() -> Self {
        VBox {}
    }
}

impl Widget for VBox {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
            .set_flex_direction(state, FlexDirection::Column)
            .set_layout_type(state, LayoutType::Vertical)
            .set_focusability(state, false)
            .set_element(state, "vbox")
            .set_width(state, Auto)
            .set_height(state, Auto)
    }
}

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

impl Widget for ResizableVBox {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
            //.set_width(state, Units::Pixels(300.0))
            //.set_max_width(state, Units::Pixels(500.0))
            //.set_min_width(state, Units::Pixels(300.0));
        //state.style.z_order.set(self.resize_marker, 1);

        //entity
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::MouseDown(button) => {
                    if *button == MouseButton::Left {
                        if state.mouse.left.pos_down.0
                            >= state.data.get_posx(entity) + state.data.get_width(entity) - 4.0
                            && state.mouse.left.pos_down.0
                                <= state.data.get_posx(entity) + state.data.get_width(entity)
                        {
                            self.resizing = true;
                            self.previous_width = state.data.get_width(entity);
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

                WindowEvent::MouseMove(x, _) => {
                    if self.resizing {
                        let distx = *x - state.mouse.left.pos_down.0;
                        entity.set_width(state, Units::Pixels(self.previous_width + distx));
                    } else {
                        if *x > state.data.get_posx(entity) + state.data.get_width(entity) - 4.0
                            && *x < state.data.get_posx(entity) + state.data.get_width(entity)
                        {
                            // state.insert_event(Event::new(WindowEvent::SetCursor(
                            //     CursorIcon::EResize,
                            // )));
                        } else {
                            // state.insert_event(Event::new(WindowEvent::SetCursor(
                            //     CursorIcon::Arrow,
                            // )));
                            state.release(entity);
                        }
                    }
                }

                _ => {}
            }
        }
    }
}
