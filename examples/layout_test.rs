use tuix::*;


fn main() {
    let app = Application::new(|state, window| {
        
        state.style.layout_type.insert(window.entity(), LayoutType::Column);
        window.set_child_top(state, Stretch(1.0)).set_child_bottom(state, Stretch(1.0));

        let one = Element::new().build(state, window.entity(), |builder| 
            builder
                .set_background_color(Color::rgb(200,70,20))
                .set_text_justify(Justify::Center)
                .set_font_size(30.0)
                .set_width(Stretch(1.0))
                .set_height(Stretch(1.0))
                .set_text("1")
                //.set_position_type(PositionType::SelfDirected)
        );

        // state.style.main_axis.insert(one, Axis {
        //     space_before: Units::Stretch(1.0),
        //     size: Units::Stretch(1.0),
        //     space_after: Units::Stretch(1.0),
        // });

        // state.style.cross_axis.insert(one, Axis {
        //     space_before: Units::Stretch(1.0),
        //     size: Units::Stretch(1.0),
        //     space_after: Units::Stretch(1.0),
        // });

        let one = Element::new().build(state, window.entity(), |builder| 
            builder
                .set_background_color(Color::rgb(20, 70, 200))
                .set_text_justify(Justify::Center)
                .set_font_size(30.0)
                .set_text("2")
                .set_width(Pixels(300.0))
                .set_height(Pixels(300.0))
                .set_position_type(PositionType::SelfDirected)
        );

        // state.style.main_axis.insert(one, Axis {
        //     space_before: Units::Auto,
        //     size: Units::Pixels(100.0),
        //     space_after: Units::Auto,
        // });

        // state.style.cross_axis.insert(one, Axis {
        //     space_before: Units::Pixels(0.0),
        //     size: Units::Stretch(1.0),
        //     space_after: Units::Pixels(0.0),
        // });

        // let one = Element::new().build(state, window.entity(), |builder| 
        //     builder
        //         .set_background_color(Color::rgb(20, 100, 70))
        //         .set_text_justify(Justify::Center)
        //         .set_font_size(30.0)
        //         .set_text("3")
        // );

        // state.style.main_axis.insert(one, Axis {
        //     space_before: Units::Auto,
        //     size: Units::Pixels(100.0),
        //     space_after: Units::Auto,
        // });

        // state.style.cross_axis.insert(one, Axis {
        //     space_before: Units::Stretch(1.0),
        //     size: Units::Stretch(1.0),
        //     space_after: Units::Stretch(1.0),
        // });


        // let one = Element::new().build(state, window.entity(), |builder| 
        //     builder
        //         .set_background_color(Color::rgb(120, 100, 20))
        //         .set_text_justify(Justify::Center)
        //         .set_font_size(30.0)
        //         .set_text("4")
        // );

        // state.style.main_axis.insert(one, Axis {
        //     space_before: Units::Auto,
        //     size: Units::Pixels(100.0),
        //     space_after: Units::Auto,
        // });

        // state.style.cross_axis.insert(one, Axis {
        //     space_before: Units::Stretch(1.0),
        //     size: Units::Pixels(300.0),
        //     space_after: Units::Stretch(1.0),
        // });

        // let one = Element::new().build(state, window.entity(), |builder| 
        //     builder
        //         .set_background_color(Color::rgb(150, 20, 200))
        //         .set_text_justify(Justify::Center)
        //         .set_font_size(30.0)
        //         .set_text("5")
        // );

        // state.style.main_axis.insert(one, Axis {
        //     space_before: Units::Auto,
        //     size: Units::Pixels(100.0),
        //     space_after: Units::Auto,
        // });

        // state.style.cross_axis.insert(one, Axis {
        //     space_before: Units::Stretch(1.0),
        //     size: Units::Stretch(1.0),
        //     space_after: Units::Stretch(1.0),
        // });

        // let one = Element::new().build(state, window.entity(), |builder| 
        //     builder
        //         .set_background_color(Color::rgb(20, 150, 200))
        //         .set_text_justify(Justify::Center)
        //         .set_font_size(30.0)
        //         .set_text("6")
        // );

        // state.style.main_axis.insert(one, Axis {
        //     space_before: Units::Auto,
        //     size: Units::Pixels(100.0),
        //     space_after: Units::Auto,
        // });
        // state.style.cross_axis.insert(one, Axis {
        //     space_before: Units::Pixels(0.0),
        //     size: Units::Pixels(100.0),
        //     space_after: Units::Stretch(1.0),
        // });



    });

    app.run();
}