use tuix::*;
fn main() {
    let app = Application::new(WindowDescription::new(), |state, window| {
        //window.set_title("Hello GUI");

        Textbox::new("ä").build(state, window.entity(), |builder| {
            builder
                .set_width(Pixels(100.0))
                .set_height(Pixels(30.0))
                .set_background_color(Color::from("#202020"))
                .set_child_space(Stretch(1.0))
        });

        

        
        // let text_area = TextArea::new("Hello There إلا بسم الله Beep Boop!!").build(
        //     state,
        //     window.entity(),
        //     |builder| {
        //         builder
        //             .set_left(Pixels(200.0))
        //             .set_top(Pixels(200.0))
        //             .set_width(Pixels(150.0))
        //             .set_height(Pixels(200.0))
        //             .set_background_color(Color::rgb(150, 200, 50))
        //             .set_child_space(Stretch(1.0))
        //             .set_font_size(20.0)
        //     },
        // );
        let text_area = TextArea::new("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.")
            .build(state, window.entity(), |builder| {
                builder
                    .set_left(Pixels(100.0))
                    .set_top(Pixels(100.0))
                    .set_width(Pixels(400.0))
                    .set_height(Pixels(400.0))
                    .set_background_color(Color::rgb(150, 200, 50))
                    .set_child_space(Stretch(1.0))
                    .set_font_size(20.0)
            },
        );
        state.focused = text_area;
    });
    app.run();
}
