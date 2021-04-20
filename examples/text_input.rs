use tuix::*;
fn main() {
    let app = Application::new(|state, window| {
        window.set_title("Hello GUI");

        Textbox::new("ä").build(state, window.entity(), |builder| {
            builder
                .set_width(Units::Pixels(100.0))
                .set_height(Units::Pixels(30.0))
                .set_background_color(Color::from("#202020"))
                .set_text_justify(Justify::Center)
        });

        let text_area = TextArea::new("Hello There إلا بسم الله Beep Boop!!").build(
            state,
            window.entity(),
            |builder| {
                builder
                    .set_left(Pixels(200.0))
                    .set_top(Pixels(200.0))
                    .set_width(Pixels(150.0))
                    .set_height(Pixels(200.0))
                    .set_background_color(Color::rgb(150, 200, 50))
                    .set_text_align(Align::Center)
                    .set_font_size(20.0)
            },
        );

        state.focused = text_area;
    });
    app.run();
}
