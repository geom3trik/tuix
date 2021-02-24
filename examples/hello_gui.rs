use tuix::*;

fn main() {
    let app = Application::new(|win_desc, state, window| {
        Button::with_label("Button").build(state, window, |builder| {
            builder
                .set_width(Length::Pixels(100.0))
                .set_height(Length::Pixels(30.0))
                .set_background_color(Color::from("#ff5e1a"))
                .set_text_justify(Justify::Center)
        });

        win_desc.with_title("Hello GUI")
    });

    app.run();
}
