use tuix::*;

fn main() {
    // Create the app
    let app = Application::new(|state, window| {
        
        window
            .set_title("position")
            .set_background_color(state, Color::white());

        Element::new().build(state, window.entity(), |context| {
            context
                .set_left(Length::Pixels(100.0))
                .set_top(Length::Pixels(100.0))
                .set_width(Length::Pixels(100.0))
                .set_height(Length::Pixels(30.0))
                .set_background_color(Color::rgb(100, 100, 100))
        });

    });

    app.run();
}
