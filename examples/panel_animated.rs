extern crate tuix;

use tuix::*;

use tuix::widgets::{
    Button, CheckButton, Checkbox, Dimension, Dropdown, Panel, List, ResizableVBox,
    ScrollContainer, Spinbox, Textbox, VectorEdit, VectorEditEvent,
};

static THEME: &'static str = include_str!("themes/panel_animated_theme.css");

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

impl Widget for ColorEdit {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity.set_flex_direction(state, FlexDirection::Row);

        let test = Dropdown::new("RGB")
            .build(state, entity, |context| {
                context
                    .set_flex_basis(Length::Pixels(40.0))
                    .set_text_justify(Justify::End)
                    .class("dim")
            })
            .2;

        let one = Dimension::new("RGB").build(state, test, |context| context.class("item"));
        let two = Dimension::new("HSV").build(state, test, |context| context.class("item"));

        self.vector_edit = VectorEdit::new()
            .with_x(100u8)
            .with_y(50u8)
            .with_z(50u8)
            .with_w(255u8)
            .build(state, entity, |context| {
                context
                    .set_flex_grow(1.0)
                    .set_margin_left(Length::Pixels(5.0))
                    .class("item")
            });

        entity
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(vectoredit_event) = event.message.downcast::<VectorEditEvent<u8>>() {
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
    }
}

fn main() {
    // Create the app
    let mut app = Application::new(|state, window| {
        state.add_theme(THEME);

        window.set_title("Panels").set_inner_size(800, 600);

        let rvbox = ResizableVBox::new().build(state, window.entity(), |context| {
            context
                .set_width(Length::Pixels(300.0))
                .set_height(Length::Percentage(1.0))
                .set_background_color(Color::rgb(100, 50, 50))
        });

        //let scroll = ScrollContainer::new().build(state, rvbox, |context| context);

        let panel = Panel::new("Background Colour").build(state, rvbox, |context| context);

        let row = HBox::new().build(state, panel, |context| context);

        Label::new("Colour").build(state, row, |context| context.class("label"));
        let color_edit = ColorEdit::new().build(state, row, |context| context.set_flex_grow(1.0));

        let row = HBox::new().build(state, panel, |context| context);

        Label::new("Length").build(state, row, |context| context.class("label"));
        LengthBox::new().build(state, row, |context| {
            context.set_flex_grow(1.0).class("item")
        });

        let row = HBox::new().build(state, panel, |context| context);

        Label::new("Slider").build(state, row, |context| context.class("label"));
        ValueSlider::new("test").build(state, row, |context| {
            context.set_flex_grow(1.0).class("item")
        });

        let row = HBox::new().build(state, panel, |context| context);

        Label::new("Number").build(state, row, |context| context.class("label"));
        Spinbox::new(100.0).build(state, row, |context| {
            context.set_flex_grow(1.0).class("item")
        });

        let row = HBox::new().build(state, panel, |context| context);

        Button::with_label("Button").build(state, row, |context| context.class("label"));
        //Button::with_label("Press Me").build(state, row, |context| context.set_flex_grow(1.0).class("item"));
        Button::new().build(state, row, |context| {
            context.set_flex_grow(1.0).set_text("PRESS").class("item")
        });
        // //Dropdown::new("Position").add_item("Absolute", "Absolute").add_item("Relative", "Relative").build(state, row, |context| context.set_flex_grow(1.0));
        // //Textbox::new("Textbox").build(state, row, |context| context.set_flex_grow(1.0).set_background_color(Color::rgb(50, 100, 100)));

        let row = HBox::new().build(state, panel, |context| context);

        Label::new("Radio").build(state, row, |context| context.class("label"));
        let radio_list = List::new().build(state, row, |context| context.set_flex_grow(1.0));

        let hbox = HBox::new().build(state, radio_list, |context| {
            context.set_height(Length::Pixels(30.0))
        });
        CheckButton::new(true).build(state, hbox, |context| {
            context.set_align_self(AlignSelf::Center)
        });
        Label::new("Option 1").build(state, hbox, |context| context.set_flex_grow(1.0));

        let hbox = HBox::new().build(state, radio_list, |context| {
            context.set_height(Length::Pixels(30.0))
        });
        CheckButton::new(false).build(state, hbox, |context| {
            context.set_align_self(AlignSelf::Center)
        });
        Label::new("Option 2").build(state, hbox, |context| context.set_flex_grow(1.0));

        let hbox = HBox::new().build(state, radio_list, |context| {
            context.set_height(Length::Pixels(30.0))
        });
        CheckButton::new(false).build(state, hbox, |context| {
            context.set_align_self(AlignSelf::Center)
        });
        Label::new("Option 3").build(state, hbox, |context| context.set_flex_grow(1.0));

        let panel = Panel::new("Control Knobs").build(state, rvbox, |context| context);

        let row = HBox::new().build(state, panel, |context| {
            context.set_justify_content(JustifyContent::SpaceEvenly)
        });

        let knob = ValueKnob::new("Red", 0.0, 0.0, 1.0).build(state, row, |context| {
            context.set_width(Length::Pixels(50.0))
        });

        let knob = ValueKnob::new("Green", 0.0, 0.0, 1.0).build(state, row, |context| {
            context.set_width(Length::Pixels(50.0))
        });

        let knob = ValueKnob::new("Blue", 0.0, 0.0, 1.0).build(state, row, |context| {
            context.set_width(Length::Pixels(50.0))
        });

        let panel = Panel::new("Control Knobs").build(state, rvbox, |context| context);

        let row = HBox::new().build(state, panel, |context| {
            context.set_justify_content(JustifyContent::SpaceEvenly)
        });

        let knob = ValueKnob::new("Red", 0.0, 0.0, 1.0).build(state, row, |context| {
            context.set_width(Length::Pixels(50.0))
        });

        let knob = ValueKnob::new("Green", 0.0, 0.0, 1.0).build(state, row, |context| {
            context.set_width(Length::Pixels(50.0))
        });

        let knob = ValueKnob::new("Blue", 0.0, 0.0, 1.0).build(state, row, |context| {
            context.set_width(Length::Pixels(50.0))
        });

        
    });

    // Get the state from the window
    //let state = &mut app.state;

    // Get the window from the app
    //let window = state.root;

    // let row = HBox::new().build(state, panel, |context| {
    //     context
    // });

    // Label::new("Radio").build(state, row, |context| context.class("label"));
    // let radio_list = RadioList::new("First").build(state, row, |context| context.set_flex_grow(1.0));

    // let hbox = HBox::new().build(state, radio_list, |context| context.set_height(Length::Pixels(30.0)));
    // RadioBox::new("Second").build(state, hbox, |context| context.set_align_self(AlignSelf::Center));
    // Label::new("TEST1").build(state, hbox, |context| context.set_flex_grow(1.0));

    // let hbox = HBox::new().build(state, radio_list, |context| context.set_height(Length::Pixels(30.0)));
    // RadioBox::new("Second").build(state, hbox, |context| context.set_align_self(AlignSelf::Center));
    // Label::new("TEST2").build(state, hbox, |context| context.set_flex_grow(1.0));

    // let hbox = HBox::new().build(state, radio_list, |context| context.set_height(Length::Pixels(30.0)));
    // RadioBox::new("Second").build(state, hbox, |context| context.set_align_self(AlignSelf::Center));
    // Label::new("TEST3").build(state, hbox, |context| context.set_flex_grow(1.0));

    // let row3 = HBox::new().build(state, panel, |context| {
    //     context.class("item")
    // });
    // Label::new("Checkbox").build(state, row3, |context| context.class("label"));
    // Checkbox::new().build(state, row3, |context| context.set_align_self(AlignSelf::Center).class("check"));

    // let row4 = Button::new().build(state, panel, |context| {
    //     context.set_flex_direction(FlexDirection::Row).class("item")
    // });
    // Button::with_label("Right").build(state, row4, |context| context.class("label"));
    // LengthBox::new().build(state, row4, |context| context.set_flex_grow(1.0));

    // let row5 = Button::new().build(state, panel, |context| {
    //     context.set_flex_direction(FlexDirection::Row).class("item")
    // });
    // Button::with_label("Bottom").build(state, row5, |context| context.class("label"));
    // LengthBox::new().build(state, row5, |context| context.set_flex_grow(1.0));

    // let flex_panel = Panel::new("Flex").build(state, scroll, |context| {
    //     context
    // });

    // let flex_panel_row1 = Button::new().build(state, flex_panel, |context| {
    //     context.set_flex_direction(FlexDirection::Row).class("item")
    // });

    // Button::with_label("Basis").build(state, flex_panel_row1, |context| context.class("label"));
    // LengthBox::new().build(state, flex_panel_row1, |context| context.set_flex_grow(1.0));

    // let flex_panel_row2 = Button::new().build(state, flex_panel, |context| {
    //     context.set_flex_direction(FlexDirection::Row).class("item")
    // });

    // Button::with_label("Grow").build(state, flex_panel_row2, |context| context.class("label"));
    // LengthBox::new().build(state, flex_panel_row2, |context| context.set_flex_grow(1.0));

    // let flex_panel_row3 = Button::new().build(state, flex_panel, |context| {
    //     context.set_flex_direction(FlexDirection::Row).class("item")
    // });

    // Button::with_label("Shrink").build(state, flex_panel_row3, |context| context.class("label"));
    // LengthBox::new().build(state, flex_panel_row3, |context| context.set_flex_grow(1.0));

    // let panel = Panel::new("Image").build(state, scroll, |context| {
    //     context
    // });
    // Button::new().build(state, panel, |context| context.class("img"));

    app.run();
}
