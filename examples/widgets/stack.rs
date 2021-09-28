use tuix::*;
use tuix::widgets::*;

const STYLE: &str = r#"

    stack {
        background-color: white;
    }

    button {
        width: 50px;
        height: 30px;
        color: black;
        background-color: gray;
        child-space: 1s;
        border-radius: 3px;
    }

    label {
        child-space: 1s;
    }

"#;

#[derive(Default)]
struct Container {
    stack: Entity,
}

impl Widget for Container {
    type Ret = Entity;
    type Data = ();
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        self.stack = Stack::new()
            // .on_change(|spinbox, state, entity| {
            //     state.insert_event(Event::new(CustomEvent::ChangeColor(Color::rgba(spinbox.value, spinbox.value, spinbox.value, 255))).target(entity));
            // })
            .build(state, entity, |builder| {
                builder
            });

        // First page
        let first = Element::new().build(state, self.stack, |builder| 
            builder
                .set_background_color(Color::red())
        );
        Label::new("Red Page").build(state, first, |builder| builder.set_color(Color::black()));
        Button::with_label("Next")
            .on_press(|_, state, button|{
                button.emit(state, StackEvent::SetIndex(1));
            })
            .build(state, first, |builder|
                builder
                    .set_position_type(PositionType::SelfDirected)
                    .set_left(Stretch(1.0))
                    .set_top(Stretch(1.0))
                    .set_right(Pixels(10.0))
                    .set_bottom(Pixels(10.0))
            );

        // Second page
        let second = Element::new().build(state, self.stack, |builder| 
            builder
                .set_background_color(Color::green())
        );
        Label::new("Green Page").build(state, second, |builder| builder.set_color(Color::black()));
        Button::with_label("Next")
            .on_press(|_, state, button|{
                button.emit(state, StackEvent::SetIndex(2));
            })
            .build(state, second, |builder|
                builder
                    .set_position_type(PositionType::SelfDirected)
                        .set_left(Stretch(1.0))
                        .set_top(Stretch(1.0))
                    .set_right(Pixels(10.0))
                    .set_bottom(Pixels(10.0))
            );

        Button::with_label("Prev")
        .on_press(|_, state, button|{
            button.emit(state, StackEvent::SetIndex(0));
        })
        .build(state, second, |builder|
            builder
                .set_position_type(PositionType::SelfDirected)
                .set_left(Pixels(10.0))
                .set_top(Stretch(1.0))
                .set_right(Stretch(1.0))
                .set_bottom(Pixels(10.0))
        );

        // Third page
        let third = Element::new().build(state, self.stack, |builder| 
            builder
                .set_background_color(Color::blue())
        );
        Label::new("Blue Page").build(state, third, |builder| builder.set_color(Color::black()));
        Button::with_label("Prev")
            .on_press(|_, state, button|{
                button.emit(state, StackEvent::SetIndex(1));
            })
            .build(state, third, |builder|
                builder
                    .set_position_type(PositionType::SelfDirected)
                    .set_left(Pixels(10.0))
                    .set_top(Stretch(1.0))
                    .set_right(Stretch(1.0))
                    .set_bottom(Pixels(10.0))
            );

        entity.set_background_color(state, Color::white())
    }
}

fn main() {
    let app = Application::new(
    WindowDescription::new()
            .with_title("Stack")
            .with_inner_size(300, 300),
    |state, window| {

            window.set_background_color(state, Color::white());

            state.add_theme(STYLE);
            
            Container::default().build(state, window, |builder| builder);

        },
    );

    app.run();
}