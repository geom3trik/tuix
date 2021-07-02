use tuix::*;

const STYLE: &str = r#"

    arc {
        width: 50px;
        height: 50px;
        background-color: red;
        radius: 5px;
    }

"#;

#[derive(Debug, Clone, PartialEq)]
enum CustomEvent {
    ChangeColor(Color),
}

#[derive(Default)]
struct Container {
    knob: Entity,
}

impl Widget for Container {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        self.knob = Knob::default()
            .build(state, entity, |builder| {
                builder
                    .set_space(Stretch(1.0))
            });

        entity.set_background_color(state, Color::rgb(79,79,79))
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
            .with_title("Knob")
            .with_inner_size(300, 300),
    |state, window| {

            window.set_background_color(state, Color::rgb(79,79,79));

            //state.add_theme(STYLE);
            state.add_stylesheet("examples/themes/knob_theme.css").expect("Failed to load theme");
            
            Container::default().build(state, window, |builder| builder);

        },
    );

    app.run();
}