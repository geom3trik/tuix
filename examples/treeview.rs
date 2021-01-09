extern crate tuix;

use tuix::*;

use tuix::widgets::{
    Button, Checkbox, Dimension, Dropdown, Spinner, Panel, RadioBox, RadioList, ScrollContainer,
    Textbox, VectorEdit, VectorEditEvent,
};

static THEME: &'static str = include_str!("themes/treeview_theme.css");

pub struct ResizableVBox {
    resizing: bool,
    hovering: bool,
    previous_width: f32,
    resize_marker: Entity,
}

impl ResizableVBox {
    pub fn new() -> Self {
        ResizableVBox {
            resizing: false,
            hovering: false,
            previous_width: 0.0,
            resize_marker: Entity::null(),
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

        //self.resize_marker = Button::new().build(state, entity, |builder| builder.set_position(Position::Absolute)
        //    .set_width(Length::Pixels(5.0)).set_height(Length::Percentage(1.0)).set_background_color(Color::rgb(70, 70, 70)));

        //state.style.z_order.set(self.resize_marker, 1);

        entity
    }
}

impl EventHandler for ResizableVBox {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) -> bool {
        if let Some(color_edit_event) = event.is_type::<ColorEditEvent>() {
            match color_edit_event {
                ColorEditEvent::ColorChanged(r, g, b, a) => {
                    entity.set_background_color(state, Color::rgba(*r, *g, *b, *a));
                }
            }
        }

        if let Some(window_event) = event.is_type::<WindowEvent>() {
            match window_event {
                WindowEvent::MouseDown(button) => {
                    if *button == MouseButton::Left {
                        if state.mouse.left.pos_down.0
                            >= state.transform.get_posx(entity) + state.transform.get_width(entity)
                                - 2.0
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
                            //state.insert_event(Event::new(WindowEvent::SetCursor(CursorIcon::Arrow)));
                        }
                        //self.resizing = false;
                        //state.release();
                    }
                }

                // Occurs when the cursor leaves the entity
                WindowEvent::MouseOut => {
                    if !self.resizing {
                        state.insert_event(Event::new(WindowEvent::SetCursor(CursorIcon::Arrow)));
                    }
                }

                WindowEvent::MouseMove(x, y) => {
                    //println!("Received mouse move: {} {}", state.hovered, entity);

                    if self.resizing {
                        let distx = *x - state.mouse.left.pos_down.0;
                        entity.set_width(state, Length::Pixels(self.previous_width + distx));
                    } else {
                        if *x
                            > state.transform.get_posx(entity) + state.transform.get_width(entity)
                                - 2.0
                            && *x
                                < state.transform.get_posx(entity)
                                    + state.transform.get_width(entity)
                        {

                            //if self.hovering == false {
                            //    self.hovering = true;
                            //println!("Change Cursor");
                            state.insert_event(Event::new(WindowEvent::SetCursor(
                                CursorIcon::EResize,
                            )));
                        //state.capture(entity);
                        //}
                        } else {
                            //if self.hovering == true {
                            //    self.hovering = false;
                            //println!("Normal Cursor");
                            state.insert_event(Event::new(WindowEvent::SetCursor(
                                CursorIcon::Arrow,
                            )));
                            //println!("Resizable box release");
                            state.release(entity);
                            //if state.hovered != entity {
                            //    state.insert_event(Event::new(WindowEvent::MouseMove(*x, *y)).target(state.hovered));
                            //}
                            //}
                        }
                    }
                }

                _ => {}
            }
        }

        // if let Some(widget_event) = event.message.downcast::<WidgetEvent>() {
        //     match widget_event {
        //         WidgetEvent::MouseLeave(id) => {
        //             println!("Mouse Left: {}", id);
        //             if *id == entity && !self.resizing {
        //                 self.resize_marker.set_visibility(state, Visibility::Invisible);
        //                 state.insert_event(Event::new(StyleEvent::Restyle));
        //             }
        //         }

        //         _=> {}
        //     }
        // }

        false
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ColorEditEvent {
    ColorChanged(u8, u8, u8, u8),
}

pub struct ColorEdit {
    vector_edit: Entity,

    rval: u8,
    gval: u8,
    bval: u8,
    aval: u8,
}

impl ColorEdit {
    pub fn new() -> Self {
        ColorEdit {
            vector_edit: Entity::null(),

            rval: 0,
            gval: 0,
            bval: 0,
            aval: 0,
        }
    }
}

impl BuildHandler for ColorEdit {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity.set_flex_direction(state, FlexDirection::Row);

        let test = Dropdown::new("RGB")
            .build(state, entity, |builder| {
                builder
                    .set_flex_basis(40.0)
                    .set_text_justify(Justify::End)
                    .class("dim")
            })
            .2;

        let one = Dimension::new("RGB").build(state, test, |builder| builder.class("item"));
        let two = Dimension::new("HSV").build(state, test, |builder| builder.class("item"));

        self.vector_edit = VectorEdit::new()
            .with_x(100u8)
            .with_y(50u8)
            .with_z(50u8)
            .with_w(255u8)
            .build(state, entity, |builder| {
                builder
                    .set_flex_grow(1.0)
                    .set_margin_left(Length::Pixels(5.0))
                    .class("item")
            });

        entity
    }
}

impl EventHandler for ColorEdit {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) -> bool {
        if let Some(vectoredit_event) = event.is_type::<VectorEditEvent<u8>>() {
            match vectoredit_event {
                VectorEditEvent::Dim1(val) => {
                    state.insert_event(
                        Event::new(ColorEditEvent::ColorChanged(*val, *val, *val, *val))
                            .target(entity),
                    );
                }

                VectorEditEvent::Dim2(r, g) => {
                    state.insert_event(
                        Event::new(ColorEditEvent::ColorChanged(*r, *g, 255, 255)).target(entity),
                    );
                }

                VectorEditEvent::Dim3(r, g, b) => {
                    state.insert_event(
                        Event::new(ColorEditEvent::ColorChanged(*r, *g, *b, 255)).target(entity),
                    );
                }

                VectorEditEvent::Dim4(r, g, b, a) => {
                    state.insert_event(
                        Event::new(ColorEditEvent::ColorChanged(*r, *g, *b, *a)).target(entity),
                    );
                }

                _ => {}
            }
        }

        return false;
    }
}

fn main() {
    //let event_loop = EventLoop::new();
    //Create the glutin window
    //let window = Window::new(&event_loop, WindowDescription::new().with_title("Panels").with_inner_size(800, 600));

    // Create the app
    let mut app = Application::new(|win_desc, state, window| {
        state.insert_theme(THEME);

        let rvbox = ResizableVBox::new().build(state, window, |builder| {
            builder
                .set_width(Length::Pixels(300.0))
                .set_height(Length::Percentage(1.0))
                .set_background_color(Color::rgb(100, 50, 50))
        });

        let panel1 = Panel::new("ROOT").build(state, rvbox, |builder| builder);

        let panel2 = Panel::new("Level 1").build(state, panel1, |builder| builder);
        let panel3 = Panel::new("Level 1").build(state, panel1, |builder| builder);
        let panel = Panel::new("Level 1").build(state, panel1, |builder| builder);

        let row = HBox::new().build(state, panel, |builder| builder);

        Label::new("Colour").build(state, row, |builder| builder.class("label"));
        let color_edit = ColorEdit::new().build(state, row, |builder| builder.set_flex_grow(1.0));

        let row = HBox::new().build(state, panel, |builder| builder);

        Label::new("Translate").build(state, row, |builder| builder.class("label"));
        LengthBox::new().build(state, row, |builder| {
            builder.set_flex_grow(1.0).class("item")
        });

        let row = HBox::new().build(state, panel, |builder| builder);

        Label::new("Translate").build(state, row, |builder| builder.class("label"));
        ValueSlider::new("test").build(state, row, |builder| {
            builder.set_flex_grow(1.0).class("item")
        });

        let row = HBox::new().build(state, panel, |builder| builder);

        Label::new("Translate").build(state, row, |builder| builder.class("label"));
        Spinner::new(100.0, 1.0).build(state, row, |builder| {
            builder.set_flex_grow(1.0).class("item")
        });

        win_desc.with_title("Panels").with_inner_size(800, 600)
    });

    app.run();
}
