use tuix::*;


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

// The state of the application
#[derive(Default, Lens)]
pub struct CounterState {
    value: i32,
}

// Messages for mutating the application state
#[derive(PartialEq)]
pub enum CounterEvent {
    Increment,
    Decrement,
}

// The Model allows the state to be mutated in response to messages
impl Model for CounterState {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(counter_event) = event.message.downcast() {
            match counter_event {
                CounterEvent::Increment => {
                    self.value += 1;
                    entity.emit(state, BindEvent::Update);
                }

                CounterEvent::Decrement => {
                    self.value -= 1;
                    entity.emit(state, BindEvent::Update);
                }
            }            
        }
    }
}

// A widget for the counter, with 2 buttons and a label
#[derive(Default)]
struct CounterWidget {

}

impl Widget for CounterWidget {
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

        // Using a lens, the label is bound to the value field of the app data
        Label::new("0")
            .bind(CounterState::value, |value| value.to_string())
            .build(state, row, |builder| 
                builder
                    .set_width(Pixels(100.0))
                    .set_height(Pixels(30.0))
                    .class("count")
            );

        entity
    }
}

fn main() {

    let window_description = WindowDescription::new().with_title("Counter").with_inner_size(400, 100);
    let app = Application::new(window_description, |state, window| {

        state.add_theme(STYLE);

        // Build the app data at the root of the visual tree
        let data_widget = CounterState::default().build(state, window);

        CounterWidget::default()
            .build(state, data_widget, |builder| builder);
        
        // Another label is bound to the counter value, but with a conversion closure 
        // which converts the value to english text form
        Label::new("Zero")
            .bind(CounterState::value, |value| english_numbers::convert_all_fmt(*value as i64))
            .build(state, data_widget, |builder| 
                builder
                    .set_height(Pixels(30.0))
                    .set_space(Pixels(5.0))
            );
    });

    app.run();
}