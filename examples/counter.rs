extern crate tuix;
use tuix::*;

use tuix::button::Button;

static THEME: &'static str = include_str!("themes/counter_theme.css");

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CounterMessage {
    Increment,
    Decrement,
}

struct Counter {
    // Local state
    value: i32,
    label: Entity,
}

impl Counter {
    pub fn new() -> Self {
        Counter {
            value: 0,
            label: Entity::null(),
        }
    }

    pub fn set_value(mut self, val: i32) -> Self {
        self.value = val;
        self
    }
}

impl BuildHandler for Counter {
    type Ret = Entity;

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret
    where
        Self: std::marker::Sized + 'static,
    {
        state.style.insert_class(entity, "counter");

        Button::with_label("increment")
            .on_press(Event::new(CounterMessage::Increment))
            .build(state, entity, |builder| builder.class("increment"));

        Button::with_label("decrement")
            .on_press(Event::new(CounterMessage::Decrement))
            .build(state, entity, |builder| builder.class("decrement"));

        self.label = Button::with_label("50")
            .on_press(Event::new(CounterMessage::Increment))
            .build(state, entity, |builder| builder);

        entity
    }
}

impl EventHandler for Counter {
    fn on_event(&mut self, state: &mut State, _entity: Entity, event: &mut Event) -> bool {
        if let Some(counter_event) = event.message.downcast::<CounterMessage>() {
            match counter_event {
                CounterMessage::Increment => {
                    self.value += 1;
                    self.label.set_text(state, &self.value.to_string());
                    state.insert_event(Event::new(WindowEvent::Redraw));
                    println!("Increment Value: {}", self.value);
                }

                CounterMessage::Decrement => {
                    self.value -= 1;
                    self.label.set_text(state, &self.value.to_string());
                    state.insert_event(Event::new(WindowEvent::Redraw));
                    println!("Decrement Value: {}", self.value);
                }
            }
        }

        false
    }
}

fn main() {
    // Create the app
    let mut app = Application::new(|win_desc, state, window| {
        state.insert_theme(THEME);

        Counter::new()
            // Set local state
            .set_value(50)
            // Build the component
            .build(state, window, |builder| {
                builder
                    .set_width(Length::Pixels(300.0))
                    .set_height(Length::Pixels(50.0))
            });

        win_desc.with_title("Counter")
    });

    app.run();
}
