use tuix::*;

fn main() {
    let app = Application::new(|state, window| {
        
        window.set_title("Hello GUI");

        Button::with_label("Button")
            .build(state, window.entity(), |context| {
                context
                    .set_width(Length::Pixels(100.0))
                    .set_height(Length::Pixels(30.0))
                    .set_background_color(Color::from("#ff5e1a"))
                    .set_text_justify(Justify::Center)
            });

    });

    app.run();
}
