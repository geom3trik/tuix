use tuix::*;

fn main() {
    let app = Application::new(|window, state, root| {
        Button::with_label("Button")
            .build(state, root, |builder| {
                builder
                    .set_width(Length::Pixels(100.0))
                    .set_height(Length::Pixels(30.0))
                    .set_background_color(Color::from("#ff5e1a"))
                    .set_text_justify(Justify::Center)
            });

        window.with_title("Hello GUI")
    });

    app.run();
}
