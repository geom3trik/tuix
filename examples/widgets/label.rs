use tuix::*;
use tuix::widgets::*;
use rand::prelude::*;

const STYLE: &str = r#"

    label {
        child-space: 1s;
        color: black;
    }

"#;

#[derive(Debug, Clone, PartialEq)]
enum CustomEvent {
    ChangeColor(Color),
}

#[derive(Default)]
struct Container {

}

impl Widget for Container {
    type Ret = Entity;
    type Data = ();
    fn on_build(&mut self, state: &mut State, container: Entity) -> Self::Ret {

        Label::new("Hello World")
            .build(state, container, |builder| {
                builder
                    .set_width(Pixels(210.0))
                    .set_height(Pixels(30.0))
                    .set_space(Stretch(1.0))
            });

        container.set_background_color(state, Color::white())
    }
}

fn main() {
    let app = Application::new(
    WindowDescription::new()
            .with_title("Button")
            .with_inner_size(300, 300),
    |state, window| {

            state.add_theme(STYLE);
            
            Container::default().build(state, window, |builder| builder);
        },
    );

    app.run();
}