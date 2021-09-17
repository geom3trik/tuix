

use tuix::*;

fn main() {
    let window_description = WindowDescription::new();

    let app = Application::new(window_description, |state, window|{
        let parent = Element::new().build(state, window, |builder|
            builder
                .set_left(Pixels(200.0))
                .set_top(Pixels(200.0))
                .set_width(Pixels(200.0))
                .set_height(Pixels(200.0))
                .set_background_color(Color::red())
                .set_border_width(Pixels(10.0))
                .set_border_color(Color::black())
                .set_min_width(Pixels(0.0))
                .set_min_height(Pixels(0.0))
        );

        Element::new().build(state, parent, |builder|
            builder
                .set_left(Pixels(50.0))
                .set_top(Pixels(50.0))
                .set_width(Pixels(200.0))
                .set_height(Pixels(200.0))
                .set_background_color(Color::green())
                //.set_position_type(PositionType::SelfDirected)
                .set_clip_widget(parent)
        );

        
    });

    app.run();
}