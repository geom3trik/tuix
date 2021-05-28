use tuix::*;
use rand::prelude::*;

const STYLE: &str = r#"

    checkbox {
        width: 20px;
        height: 20px;
        background-color: white;
        border-width: 1px;
        border-color: #757575;
        color: black;
    }
"#;

#[derive(Debug, Clone, PartialEq)]
enum CustomEvent {
    ChangeColor(Color),
}

#[derive(Default)]
struct Container {
    checkbox: Entity,
}

impl Widget for Container {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        self.checkbox = Checkbox::new(true)
            .on_checked(|checkbox, state, entity| {
                state.insert_event(Event::new(CustomEvent::ChangeColor(Color::rgba(255, 100, 100, 255))).target(entity));
            })
            .on_unchecked(|checkbox, state, entity|{
                state.insert_event(Event::new(CustomEvent::ChangeColor(Color::rgba(255, 255, 255, 255))).target(entity));
            })
            .build(state, entity, |builder| {
                builder
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
            .with_title("Checkbox")
            .with_inner_size(300, 300),
    |state, window| {

            window.set_background_color(state, Color::white());

            state.add_theme(STYLE);
            
            Container::default().build(state, window, |builder| builder);

        },
    );

    app.run();
}