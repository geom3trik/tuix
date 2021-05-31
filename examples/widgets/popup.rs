use tuix::*;

const STYLE: &str = r#"

    popup {
        background-color: #d2d2d2;
    }
"#;

#[derive(Debug, Clone, PartialEq)]
enum CustomEvent {
    ChangeColor(Color),
}

#[derive(Default)]
struct Container {
    popup: Entity,
}

impl Widget for Container {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        self.popup = Popup::new()
            .build(state, entity, |builder| {
                builder
                    .set_width(Pixels(100.0))
                    .set_height(Pixels(200.0))
            });

        entity.set_background_color(state, Color::white())
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::MouseDown(_) => {

                }

                _=> {}
            }
        }
        
        // if let Some(custom_event) = event.message.downcast() {
        //     match custom_event {
        //         CustomEvent::ChangeColor(color) => {
        //             entity.set_background_color(state, *color);
        //         }
        //     }
        // }
    }
}

fn main() {
    let app = Application::new(
    WindowDescription::new()
            .with_title("Button")
            .with_inner_size(300, 300),
    |state, window| {

            window.set_background_color(state, Color::white());

            state.add_theme(STYLE);
            
            Container::default().build(state, window, |builder| builder);

        },
    );

    app.run();
}