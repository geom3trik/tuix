use tuix::*;

fn main() {
    let app = Application::new(
        WindowDescription::new()
            .with_title("Custom Title")
            .with_inner_size(300, 300),
    |state, window| {
            
            Element::new().build(state, window.entity(), |builder| {
                builder
                    .set_width(Units::Pixels(100.0))
                    .set_height(Units::Pixels(30.0))
                    .set_background_color(Color::rgb(200, 80, 20))
            });
        },
    );

    app.run();
}
