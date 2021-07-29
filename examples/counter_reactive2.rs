use tuix::*;
use fnv::FnvHashMap;
use std::any::Any;

static THEME: &'static str = include_str!("themes/counter_theme.css");

#[derive(Default, Node)]
pub struct CounterState {
    value: i32,
}

impl ToString for CounterState {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

#[derive(Default)]
struct CounterWidget {
    data: Entity,
}

impl CounterWidget {

    pub fn new(data_id: Entity) -> Self {
        Self {
            data: data_id,
        }
    }
}

impl Widget for CounterWidget {
    type Ret = Entity;
    type Data = ();

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        Button::with_label("increment")
            .on_press(|_, state, button|{
                button.mutate(state, |data: &mut CounterState|{
                    data.value += 1;
                });
            })
            .build(state, entity, |builder| builder.class("increment"))
            .bind(state, self.data);

        Button::with_label("decrement")
            .on_press(|_, state, button|{
                button.mutate(state, |data: &mut CounterState|{
                    data.value -= 1;
                });
            })
            .build(state, entity, |builder| builder.class("decrement"))
            .bind(state, self.data);

        Label::<CounterState>::new(&self.value.to_string())
            .build(state, entity, |builder| builder)
            .bind(state, self.data);

        entity.set_element(state, "counter").set_layout_type(state, LayoutType::Row)
    }
}

fn main() {
    // Create the app
    let window_description = WindowDescription::new().with_title("Counter").with_inner_size(400, 100);
    let app = Application::new(window_description, |state, window| {
        state.add_theme(THEME);

        let app_data = CounterState::default().build(state, window);

        CounterWidget::new(app_data)
            .build(state, window, |builder| builder);

        CounterWidget::new(app_data)
            .build(state, window, |builder| builder);
    });

    app.run();
}