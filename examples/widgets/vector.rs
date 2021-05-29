use tuix::*;

const STYLE: &str = r#"

    vector>textbox {
        width: 1s;
        background-color: white;
        child-space: 1s;
        color: black;
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
    vector: Entity,
}

impl Widget for Container {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        self.vector = Vector::new()
            .on_change(|vector, state, entity| {
                
                state.insert_event(Event::new(CustomEvent::ChangeColor(Color::rgba(255, 255, 255, 255))).target(entity));
            })
            .build(state, entity, |builder| {
                builder
                    .set_width(Stretch(1.0))
                    .set_height(Pixels(30.0))
                    .set_space(Stretch(1.0))
            });

        entity.set_background_color(state, Color::white())
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
            .with_title("Vector Edit")
            .with_inner_size(300, 300),
    |state, window| {

            window.set_background_color(state, Color::white());

            state.add_theme(STYLE);
            
            Container::default().build(state, window, |builder| builder);

        },
    );

    app.run();
}