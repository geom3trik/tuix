use tuix::*;

fn main() {

    let app = Application::new(WindowDescription::new().with_title("Hello GUI"), |state, window| {
        
        Button::with_label("Button")
            .build(state, window.entity(), |builder| {
                builder
                    .set_width(Pixels(100.0))
                    .set_height(Pixels(30.0))
                    .set_background_color(Color::from("#ff5e1a"))
                    .set_child_space(Stretch(1.0))
            });
    });

    app.run();
}