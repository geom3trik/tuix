use tuix::*;

fn main() {
    // Create the app
    let app = Application::new(|state, window| {
        
        window
            .set_title("position")
            .set_background_color(state, Color::white());

        Element::new().build(state, window.entity(), |builder| {
            builder
                .set_left(Units::Pixels(100.0))
                .set_top(Units::Pixels(100.0))
                .set_width(Units::Pixels(100.0))
                .set_height(Units::Pixels(30.0))
                .set_background_color(Color::rgb(100, 100, 100))
        });

    });

    app.run();
}
