use tuix::*;

fn main() {
    // Create the app
    let window_description = WindowDescription::new().with_title("Position");
    let app = Application::new(
        window_description,
        |state, window| {
        
        window.set_background_color(state, Color::white());

        Element::new().build(state, window.entity(), |builder| {
            builder
                .set_left(Pixels(100.0))
                .set_top(Pixels(100.0))
                .set_width(Pixels(100.0))
                .set_height(Pixels(30.0))
                .set_background_color(Color::rgb(100, 100, 100))
        });
    });

    app.run();
}
