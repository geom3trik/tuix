use tuix::*;

fn main() {
    let window_description = WindowDescription::new().with_title("Hello GUI");
    let app = Application::new(window_description, |state, window| {

        // Create a shared style wich applies to all widgets with class name "my_class"
        let style_rule: StyleRule = StyleRule::new()
            .selector(Selector::new().class("my_class"))
            .set_height(Pixels(30.0))
            .set_background_color(Color::rgb(80, 200, 20));

        // Add the shared style rule to state
        state.add_style_rule(style_rule);

        state
            .style
            .layout_type
            .insert(window.entity(), LayoutType::Column);

        let container = Element::new().build(state, window.entity(), |builder| {
            builder
                .set_width(Pixels(100.0))
                .set_left(Stretch(1.0))
                .set_right(Stretch(1.0))
                .set_top(Stretch(1.0))
                .set_bottom(Stretch(1.0))
                .set_layout_type(LayoutType::Column)
                .class("my_class")
        });

        // Add a Button widget as a child of the Element widget
        Button::new().build(state, container, |builder| {
            builder
                .set_width(Pixels(30.0))
                .set_background_color(Color::rgb(20, 80, 200))
                .class("my_class")
                .on_press(|button, state, entity| {
                    println!("Found button: {}", entity);
                })
        });
    });

    app.run();
}

// use tuix::*;

// fn main() {
//     let app = Application::new(|state, window| {

//         window.set_title("Custom Title").set_inner_size(300,300);

//         state.style.layout_type.insert(window.entity(), LayoutType::Column);

//         Button::new().build(state, window.entity(), |builder|
//             builder
//                 .set_width(Units::Stretch(1.0))
//                 .set_height(Units::Pixels(30.0))
//                 .set_background_color(Color::rgb(255,94,20))
//         );

//     });

//     app.run();
// }
