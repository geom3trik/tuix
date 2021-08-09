extern crate tuix;
use tuix::*;

use tuix::button::Button;

static THEME: &'static str = include_str!("themes/counter_theme.css");

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CounterMessage {
    Increment,
    Decrement,
}

#[derive(Default)]
struct Counter {
    value: i32,
    label: Entity,
}

impl Counter {
    pub fn set_initial_value(mut self, val: i32) -> Self {
        self.value = val;
        self
    }
}

impl Widget for Counter {
    type Ret = Entity;

    // Build
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        Button::with_label("increment")
            .on_press(|_, state, button|{
                button.emit(state,CounterMessage::Increment);
            })
            .build(state, entity, |builder| builder.class("increment"));

        Button::with_label("decrement")
            .on_press(|_, state, button|{
                button.emit(state,CounterMessage::Decrement);
            })
            .build(state, entity, |builder| builder.class("decrement"));

        self.label = Label::new(&self.value.to_string()).build(state, entity, |builder| builder);

        entity.set_element(state, "counter").set_layout_type(state, LayoutType::Row)
    }

    // Events
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(counter_event) = event.message.downcast::<CounterMessage>() {
            match counter_event {
                CounterMessage::Increment => {
                    self.value += 1;
                    self.label.set_text(state, &self.value.to_string());
                    event.consume();
                }

                CounterMessage::Decrement => {
                    self.value -= 1;
                    self.label.set_text(state, &self.value.to_string());
                    event.consume();
                }
            }
        }
    }
}

fn main() {
    // Create the app
    let window_description = WindowDescription::new().with_title("Counter").with_inner_size(400, 100);
    let app = Application::new(window_description, |state, window| {
        state.add_theme(THEME);

        window.set_background_color(state, Color::rgb(250, 250, 250));

        Counter::default()
            // Set local state
            .set_initial_value(50)
            // Build the widget
            .build(state, window.entity(), |builder| builder);
    });

    app.run();
}
