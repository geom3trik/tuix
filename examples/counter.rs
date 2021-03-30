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
    fn on_build(&mut self, mut builder: Builder) -> Self::Ret {
        Button::with_label("increment")
            .on_press(Event::new(CounterMessage::Increment))
            .build(&mut builder).class("increment");

        Button::with_label("decrement")
            .on_press(Event::new(CounterMessage::Decrement))
            .build(&mut builder).class("decrement");

        self.label = Label::new(&self.value.to_string()).build(&mut builder).entity();

        builder.set_element("counter").entity()
    }

    // Events
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(counter_event) = event.message.downcast::<CounterMessage>() {
            match counter_event {
                CounterMessage::Increment => {
                    self.value += 1;
                    self.label.set_text(state, &self.value.to_string());
                }

                CounterMessage::Decrement => {
                    self.value -= 1;
                    self.label.set_text(state, &self.value.to_string());
                }
            }
        }
    }
}

fn main() {
    // Create the app
    let app = Application::new(|mut ctx, window| {
        ctx.state().add_theme(THEME);

        // Set the window title and size
        window.set_title("Counter").set_inner_size(400, 100);

        Counter::default()
            // Set local state
            .set_initial_value(50)
            // Build the widget
            .build(&mut ctx);
    });

    app.run();
}
