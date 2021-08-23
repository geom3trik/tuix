#![allow(dead_code)]

use crate::common::*;
use tuix_core::CursorIcon;

pub struct Row {}

impl Row {
    pub fn new() -> Self {
        Self {}
    }
}

impl Widget for Row {
    type Ret = Entity;
    type Data<'a> = &'a ();
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
            .set_layout_type(state, LayoutType::Row)
            .set_focusable(state, false)
            .set_element(state, "row")
        //.set_width(state, Auto)
        //.set_height(state, Auto)
    }
}

pub struct Column {}

impl Column {
    pub fn new() -> Self {
        Self {}
    }
}

impl Widget for Column {
    type Ret = Entity;
    type Data<'a> = &'a ();
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
            .set_layout_type(state, LayoutType::Column)
            .set_focusable(state, false)
            .set_element(state, "column")
        //.set_width(state, Auto)
        //.set_height(state, Auto)
    }
}

pub struct ResizableColumn {
    resizing: bool,
    previous_width: f32,
}

impl ResizableColumn {
    pub fn new() -> Self {
        Self {
            resizing: false,
            previous_width: 0.0,
        }
    }
}

impl Widget for ResizableColumn {
    type Ret = Entity;
    type Data<'a> = &'a ();
    fn on_build(&mut self, _state: &mut State, entity: Entity) -> Self::Ret {
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
