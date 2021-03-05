use tuix::*;

fn main() {
    // Create the app
    let app = Application::new(|win_desc, state, window| {
        window.set_background_color(state, Color::white());

        Element::new().build(state, window, |builder| {
            builder
                .set_left(Length::Pixels(100.0))
                .set_top(Length::Pixels(100.0))
                .set_width(Length::Pixels(100.0))
                .set_height(Length::Pixels(30.0))
                .set_background_color(Color::rgb(50, 50, 50))
        });

        win_desc.with_title("position")
    });

    app.run();
}
