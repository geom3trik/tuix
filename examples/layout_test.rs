use tuix::*;


fn main() {
    let app = Application::new(|state, window| {
        
        state.style.layout_type.insert(window.entity(), LayoutType::Horizontal);
        state.style.main_axis_align.insert(window.entity(), AxisAlign {
            space_before_first: Units::Pixels(20.0),
            space_between: Units::Pixels(20.0),
            space_after_last: Units::Pixels(20.0),
        });

        state.style.cross_axis_align.insert(window.entity(), AxisAlign {
            space_before_first: Units::Pixels(0.0),
            space_between: Units::Pixels(20.0),
            space_after_last: Units::Stretch(1.0),
        });
        
        let one = Element::new().build(state, window.entity(), |builder| 
            builder
                .set_background_color(Color::red())
                .set_text_justify(Justify::Center)
                .set_font_size(30.0)
                .set_text("1")
        );

        state.style.main_axis.insert(one, Axis {
            space_before: Units::Inherit,
            size: Units::Pixels(100.0),
            space_after: Units::Inherit,
        });

        state.style.cross_axis.insert(one, Axis {
            space_before: Units::Inherit,
            size: Units::Pixels(100.0),
            space_after: Units::Inherit,
        });

        let one = Element::new().build(state, window.entity(), |builder| 
            builder
                .set_background_color(Color::green())
                .set_text_justify(Justify::Center)
                .set_font_size(30.0)
                .set_text("2")
        );

        state.style.main_axis.insert(one, Axis {
            space_before: Units::Inherit,
            size: Units::Pixels(100.0),
            space_after: Units::Inherit,
        });

        state.style.cross_axis.insert(one, Axis {
            space_before: Units::Inherit,
            size: Units::Pixels(100.0),
            space_after: Units::Inherit,
        });

        let one = Element::new().build(state, window.entity(), |builder| 
            builder
                .set_background_color(Color::blue())
                .set_text_justify(Justify::Center)
                .set_font_size(30.0)
                .set_text("3")
        );

        state.style.main_axis.insert(one, Axis {
            space_before: Units::Inherit,
            size: Units::Pixels(100.0),
            space_after: Units::Inherit,
        });

        state.style.cross_axis.insert(one, Axis {
            space_before: Units::Inherit,
            size: Units::Pixels(100.0),
            space_after: Units::Inherit,
        });


        let one = Element::new().build(state, window.entity(), |builder| 
            builder
                .set_background_color(Color::red())
                .set_text_justify(Justify::Center)
                .set_font_size(30.0)
                .set_text("4")
        );

        state.style.main_axis.insert(one, Axis {
            space_before: Units::Inherit,
            size: Units::Pixels(100.0),
            space_after: Units::Inherit,
        });

        state.style.cross_axis.insert(one, Axis {
            space_before: Units::Inherit,
            size: Units::Pixels(100.0),
            space_after: Units::Inherit,
        });

        let one = Element::new().build(state, window.entity(), |builder| 
            builder
                .set_background_color(Color::green())
                .set_text_justify(Justify::Center)
                .set_font_size(30.0)
                .set_text("5")
        );

        state.style.main_axis.insert(one, Axis {
            space_before: Units::Inherit,
            size: Units::Pixels(100.0),
            space_after: Units::Inherit,
        });

        state.style.cross_axis.insert(one, Axis {
            space_before: Units::Inherit,
            size: Units::Pixels(100.0),
            space_after: Units::Inherit,
        });

        let one = Element::new().build(state, window.entity(), |builder| 
            builder
                .set_background_color(Color::blue())
                .set_text_justify(Justify::Center)
                .set_font_size(30.0)
                .set_text("6")
        );

        state.style.main_axis.insert(one, Axis {
            space_before: Units::Inherit,
            size: Units::Pixels(100.0),
            space_after: Units::Inherit,
        });
        state.style.cross_axis.insert(one, Axis {
            space_before: Units::Inherit,
            size: Units::Pixels(100.0),
            space_after: Units::Inherit,
        });



    });

    app.run();
}