use tuix::*;
use tuix::widgets::*;

const STYLE: &str = r#"

"#;

#[derive(Debug, Clone, PartialEq)]
enum CustomEvent {
    ChangeColor(Color),
}

#[derive(Default)]
struct Container {
    color_picker: Entity,
}

impl Widget for Container {
    type Ret = Entity;
    type Data = ();
    fn on_build(&mut self, state: &mut State, container: Entity) -> Self::Ret {

        self.color_picker = ColorPicker::new()
            .build(state, container, |builder| {
                builder
                    .set_left(Pixels(10.0))
                    .set_top(Pixels(10.0))
            });

        container.set_background_color(state, Color::white())
    }

    // fn on_event(&mut self, state: &mut State, container: Entity, event: &mut Event) {
    //     if let Some(custom_event) = event.message.downcast() {
    //         match custom_event {
    //             CustomEvent::ChangeColor(color) => {
    //                 container.set_background_color(state, *color);
    //             }
    //         }
    //     }
    // }
}

fn main() {
    let app = Application::new(
    WindowDescription::new()
            .with_title("Color Picker")
            .with_inner_size(480, 270),
    |state, window| {

            state.add_theme(STYLE);
            
            Container::default().build(state, window, |builder| builder);
        },
    );

    app.run();
}