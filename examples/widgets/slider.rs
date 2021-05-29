use tuix::*;

const STYLE: &str = r#"

    slider {
        height: 20px;
    }

    slider>.track {
        background-color: #dfdfdf;
        border-radius: 2px;
    }

    slider>.track>.active {
        background-color: #f74c00;
        border-radius: 2px;
    }

    slider>.thumb {
        background-color: white;
        width: 20px;
        height: 20px;
        border-radius: 9.5px;
        border-color: #757575;
        border-width: 1px;
    }

"#;

#[derive(Debug, Clone, PartialEq)]
enum CustomEvent {
    ChangeColor(Color),
}

#[derive(Default)]
struct Container {
    slider: Entity,
}

impl Widget for Container {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        self.slider = Slider::new()
            .on_changing(|slider, state, entity| {
                let val = (slider.value * 255.0) as u8;
                state.insert_event(Event::new(CustomEvent::ChangeColor(Color::rgba(val, val, val, 255))).target(entity));
            })
            .build(state, entity, |builder| {
                builder
                    .set_width(Pixels(210.0))
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
            .with_title("Slider")
            .with_inner_size(300, 300),
    |state, window| {

            window.set_background_color(state, Color::white());

            state.add_theme(STYLE);
            
            Container::default().build(state, window, |builder| builder);

        },
    );

    app.run();
}