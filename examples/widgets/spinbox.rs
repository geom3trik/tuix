use tuix::*;
use tuix::widgets::*;

const STYLE: &str = r#"

    spinbox {
        background-color: white;
    }

    spinbox .increment {
        color: #ff5e1a;
    }

    spinbox .decrement {
        color: #ff5e1a;
    }

    spinbox>textbox {
        color: black;
        border-width: 1px;
        border-color: #757575;
        right: -1px;
        child-space: 1s;
    }

    spinbox>.arrow_container {
        border-width: 1px;
        border-color: #757575;
    }

"#;

#[derive(Debug, Clone, PartialEq)]
enum CustomEvent {
    ChangeColor(Color),
}

#[derive(Default)]
struct Container {
    spinbox: Entity,
}

impl Widget for Container {
    type Ret = Entity;
    type Data = ();
    fn on_build(&mut self, state: &mut State, container: Entity) -> Self::Ret {

        self.spinbox = Spinbox::new(128u8)
            .on_change(|spinbox_data, state, spinbox| {
                spinbox.emit(state, CustomEvent::ChangeColor(Color::rgb(spinbox_data.value, spinbox_data.value, spinbox_data.value)));
            })
            .build(state, container, |builder| {
                builder
                    .set_width(Pixels(210.0))
                    .set_height(Pixels(30.0))
                    .set_space(Stretch(1.0))
            });

        container.set_background_color(state, Color::white())
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(custom_event) = event.message.downcast() {
            match custom_event {
                CustomEvent::ChangeColor(color) => {
                    entity.set_background_color(state, *color);
                }
            }
        }
    }
}

fn main() {
    let app = Application::new(
    WindowDescription::new()
            .with_title("Spinbox")
            .with_inner_size(300, 300),
    |state, window| {

            window.set_background_color(state, Color::white());

            state.add_theme(STYLE);
            
            Container::default().build(state, window, |builder| builder);

        },
    );

    app.run();
}