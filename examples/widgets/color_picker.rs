use tuix::*;
use rand::prelude::*;

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
            .with_title("Button")
            .with_inner_size(300, 300),
    |state, window| {

            state.add_theme(STYLE);
            
            Container::default().build(state, window, |builder| builder);
        },
    );

    app.run();
}