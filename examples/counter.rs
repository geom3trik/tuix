extern crate tuix;
use tuix::*;
use tuix::widgets::*;
const STYLE: &str = r#"
    button {
        border-radius: 3px;
        child-space: 1s;
    }

    button.increment {
        background-color: #2e7d32;
        border-radius: 3px;
    }

    button.increment:hover {
        background-color: #60ad5e;
    }

    button.increment:active {
        background-color: #005005;
    }
    
    button.decrement {
        background-color: #c62828;
        border-radius: 3px;
    }

    button.decrement:hover {
        background-color: #ff5f52;
    }

    button.decrement:active {
        background-color: #8e0000;
    }

    label {
        background-color: #404040;
        border-color: #606060;
        border-width: 1px;
        child-space: 1s;
    }
"#;

pub enum CounterEvent {
    Increment,
    Decrement,
}

#[derive(Default)]
struct Counter {
    value: i32,
    label: Entity,
}

impl Widget for Counter {
    type Ret = Entity;
    type Data = ();

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        let row = Row::new().build(state, entity, |builder|
            builder
                .set_child_space(Stretch(1.0))
                .set_col_between(Pixels(10.0))
        );

        Button::with_label("Decrement")
        .on_press(|_, state, button|{
            button.emit(state,CounterEvent::Decrement);
        })
        .build(state, row, |builder| 
            builder
                .set_width(Pixels(100.0))
                .set_height(Pixels(30.0))
                .class("decrement")
        );

        Button::with_label("Increment")
            .on_press(|_, state, button|{
                button.emit(state,CounterEvent::Increment);
            })
            .build(state, row, |builder| 
                builder
                    .set_width(Pixels(100.0))
                    .set_height(Pixels(30.0))
                    .class("increment")
            );

        self.label = Label::new("0")
            .build(state, row, |builder| 
                builder
                    .set_width(Pixels(100.0))
                    .set_height(Pixels(30.0))
            );

        entity
    }

    // Events
    fn on_event(&mut self, state: &mut State, _entity: Entity, event: &mut Event) {
        if let Some(counter_event) = event.message.downcast() {
            match counter_event {
                CounterEvent::Increment => {
                    self.value += 1;
                    self.label.set_text(state, &self.value.to_string());
                }

                CounterEvent::Decrement => {
                    self.value -= 1;
                    self.label.set_text(state, &self.value.to_string());
                }
            }
        }
    }
}

fn main() {

    let window_description = WindowDescription::new().with_title("Counter").with_inner_size(400, 100);
    let app = Application::new(window_description, |state, window| {
        state.add_theme(STYLE);

        Counter::default()
            .build(state, window, |builder| builder);
    });

    app.run();
}
