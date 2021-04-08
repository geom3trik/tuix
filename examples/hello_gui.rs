use tuix::*;

fn main() {
    let app = Application::new(|state, window| {
        
        window.set_title("Custom Title").set_inner_size(300,300);

        let container = Element::new().build(state, window.entity(), |builder| 
            builder
                .set_width(Units::Pixels(100.0))
                .set_height(Units::Pixels(30.0))
                .set_background_color(Color::rgb(200,80,20))
        );

        // Add a Button widget as a child of the Element widget
        Button::new().build(state, container, |builder| 
            builder
                .set_width(Units::Pixels(30.0))
                .set_height(Units::Pixels(30.0))
                .set_background_color(Color::rgb(20,80,200))
        );

    });

    app.run();
}



// use tuix::*;

// fn main() {
//     let app = Application::new(|state, window| {
        
//         window.set_title("Custom Title").set_inner_size(300,300);

//         state.style.layout_type.insert(window.entity(), LayoutType::Vertical);

//         Button::new().build(state, window.entity(), |builder| 
//             builder
//                 .set_width(Units::Stretch(1.0))
//                 .set_height(Units::Pixels(30.0))
//                 .set_background_color(Color::rgb(255,94,20))
//         );

//     });

//     app.run();
// }