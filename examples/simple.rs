use tuix::*;

fn main() {
    let app = Application::new(
        WindowDescription::new()
            .with_title("Custom Title")
            .with_inner_size(300, 300),
    |state, window| {
            

            let animation_state = AnimationState::new()
                .with_duration(std::time::Duration::from_secs(1))
                .with_keyframe((0.0, 0.0))
                .with_keyframe((1.0, 90.0));

            let animation = state.style.rotate.insert_animation(animation_state);

            let container = Button::new()
            .on_press(move |_, state, button|{
                state.style.rotate.play_animation(button, animation);
            })
            .build(state, window.entity(), |builder| {
                builder
                    .set_width(Units::Pixels(100.0))
                    .set_height(Units::Pixels(30.0))
                    .set_background_color(Color::rgb(200, 80, 20))
                    .set_left(Pixels(50.0))
                    .set_top(Pixels(50.0))
                    .set_rotate(45.0)
                    //.set_scale(2.0)
            });

            Button::new()
            .on_press(move |_, state, button|{
                //state.style.rotate.play_animation(button, animation);
            })
            .build(state, container, |builder| {
                builder
                    .set_width(Units::Pixels(20.0))
                    .set_height(Units::Pixels(20.0))
                    .set_background_color(Color::rgb(200, 80, 200))
                    .set_rotate(45.0)
                    //.set_scale(2.0)
            });

        },
    );

    app.run();
}
