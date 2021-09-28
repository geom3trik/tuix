use tuix::*;
use tuix::widgets::*;

const STYLE: &str = r#"

    arc {
        width: 50px;
        height: 50px;
        background-color: red;
        radius: 5px;
    }

"#;

#[derive(Default, Lens)]
pub struct AppState {
    backgorund_color: Color,
}

impl Model for AppState {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(custom_event) = event.message.downcast() {
            match custom_event {
                CustomEvent::ChangeColor(color) => {
                    self.backgorund_color = *color;
                    entity.emit(state, BindEvent::Update);
                    event.consume();
                }
            }
        }
    }
}

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
    type Data = Color;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        /*
        // Generic float range

        let map = GenericMap::new(-1.0, 1.0, Gradient::Linear, DisplayDecimals::Two, None);
        let normalized_default = map.value_to_normalized(-0.5);

        self.knob = Knob::new(map, normalized_default)
            .build(state, entity, |builder| {
                builder
                    .set_space(Stretch(1.0))
            });
        */

        // Decibel range

        // let map = DecibelMap::new(-90.0, 3.0, ValueScaling::Power(0.15), DisplayDecimals::One, true);
        // let normalized_default = map.db_to_normalized(0.0);

        let map = DecibelMap::new(-12.0, 12.0, ValueScaling::Linear, DisplayDecimals::One, true);
        let normalized_default = map.db_to_normalized(0.0);
        
        self.knob = Knob::new(map, normalized_default)
        .on_changing(|data, state, knob|{
            let col = (data.normalized_value * 255.0) as u8;
            knob.emit(state, CustomEvent::ChangeColor(Color::rgb(col, col, col)));
        })
        .bind(AppState::backgorund_color, |color| color.r() as f32 / 255.0)
        .build(state, entity, |builder| {
            builder
                .set_space(Stretch(1.0))
        });

        let map = DecibelMap::new(-90.0, 3.0, ValueScaling::Power(0.15), DisplayDecimals::One, true);
        let normalized_default = map.db_to_normalized(0.0);

        self.knob = Knob::new(map, normalized_default)
        .on_changing(|data, state, knob|{
            let col = (data.normalized_value * 255.0) as u8;
            knob.emit(state, CustomEvent::ChangeColor(Color::rgb(col, col, col)));
        })
        .bind(AppState::backgorund_color, |color| color.r() as f32 / 255.0)
        .build(state, entity, |builder| {
            builder
                .set_space(Stretch(1.0))
        });

        /*
        // Frequency range

        let map = FrequencyMap::new(20.0, 20_000.0, Gradient::Frequency, FrequencyDisplayMode::default(), true);
        let normalized_default = map.hz_to_normalized(1_000.0);

        self.knob = Knob::new(map, normalized_default)
        .build(state, entity, |builder| {
            builder
                .set_space(Stretch(1.0))
        });
        */

        /*
        // Integer range

        let map = IntMap::new(
            0,
            6,
            Some(&|int: i32| -> String {
                String::from(match int {
                    0 => "A",
                    1 => "B",
                    2 => "C",
                    3 => "D",
                    4 => "E",
                    5 => "F",
                    _ => "G",
                })
            }));
        let normalized_default = map.int_to_normalized(2);

        self.knob = Knob::new(map, normalized_default)
        .build(state, entity, |builder| {
            builder
                .set_space(Stretch(1.0))
        });
        */

        entity.set_background_color(state, Color::rgb(79,79,79))
    }

    fn on_update(&mut self, state: &mut State, entity: Entity, data: &Self::Data) {
        entity.set_background_color(state, *data);
    }

    // fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
    //     if let Some(custom_event) = event.message.downcast() {
    //         match custom_event {
    //             CustomEvent::ChangeColor(color) => {
    //                 entity.set_background_color(state, *color);
    //             }
    //         }
    //     }
    // }
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
            
            let app_data = AppState::default().build(state, window);

            Container::default()
                .bind(AppState::backgorund_color, |value| *value)
                .build(state, app_data, |builder| builder);

        },
    );

    app.run();
}