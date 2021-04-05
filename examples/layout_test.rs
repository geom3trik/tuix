use tuix::*;


fn main() {
    let app = Application::new(|state, window| {
        
        state.style.layout_type.insert(window.entity(), LayoutType::Horizontal);
        state.style.main_axis_align.insert(window.entity(), AxisAlign {
            space_before_first: Units::Pixels(0.0),
            space_between: Units::Pixels(0.0),
            space_after_last: Units::Pixels(0.0),
        });

        state.style.cross_axis_align.insert(window.entity(), AxisAlign {
            space_before_first: Units::Pixels(0.0),
            space_between: Units::Pixels(20.0),
            space_after_last: Units::Pixels(0.0),
        });
        
        let one = Element::new().build(state, window.entity(), |builder| 
            builder
                .set_background_color(Color::rgb(200,70,20))
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
            space_before: Units::Stretch(1.0),
            size: Units::Stretch(1.0),
            space_after: Units::Pixels(0.0),
        });

        let one = Element::new().build(state, window.entity(), |builder| 
            builder
                .set_background_color(Color::rgb(20, 70, 200))
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
            space_before: Units::Pixels(0.0),
            size: Units::Stretch(1.0),
            space_after: Units::Pixels(0.0),
        });

        let one = Element::new().build(state, window.entity(), |builder| 
            builder
                .set_background_color(Color::rgb(20, 100, 70))
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
            space_before: Units::Stretch(1.0),
            size: Units::Stretch(1.0),
            space_after: Units::Stretch(1.0),
        });


        let one = Element::new().build(state, window.entity(), |builder| 
            builder
                .set_background_color(Color::rgb(120, 100, 20))
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
            space_before: Units::Stretch(1.0),
            size: Units::Pixels(300.0),
            space_after: Units::Stretch(1.0),
        });

        let one = Element::new().build(state, window.entity(), |builder| 
            builder
                .set_background_color(Color::rgb(150, 20, 200))
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
            space_before: Units::Stretch(1.0),
            size: Units::Stretch(1.0),
            space_after: Units::Stretch(1.0),
        });

        let one = Element::new().build(state, window.entity(), |builder| 
            builder
                .set_background_color(Color::rgb(20, 150, 200))
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
            space_before: Units::Pixels(0.0),
            size: Units::Pixels(100.0),
            space_after: Units::Stretch(1.0),
        });



    });

    app.run();
}