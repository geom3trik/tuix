use tuix::*;
use tuix::widgets::*;

fn main() {
    let window_description = WindowDescription::new();
    let app = Application::new(window_description, |state, window|{
        let animation = state.create_animation(std::time::Duration::from_secs(5))
        .add_keyframe(0.0, |keyframe| keyframe.set_left(Pixels(0.0)))
        .add_keyframe(1.0, |keyframe| keyframe.set_left(Pixels(300.0)))
        .build();

        let element = Element::new().build(state, window, |builder|
            builder
                .set_width(Pixels(100.0))
                .set_height(Pixels(100.0))
                .set_background_color(Color::red())
                //.set_left(Pixels(0.0))
        );

        state.style.left.play_animation(element, animation);
    });

    app.run();
}