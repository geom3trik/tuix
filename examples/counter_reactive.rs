use tuix::*;
use fnv::FnvHashMap;
use std::{any::Any, collections::HashSet, hash::Hash};

static THEME: &'static str = include_str!("themes/counter_theme.css");


#[derive(Default, Data, Lens)]
pub struct CounterState {
    value: i32,
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CounterMessage {
    Increment,
    Decrement,
}

impl Widget for CounterState {
    type Ret = Entity;
    type Data = ();

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(counter_event) = event.message.downcast() {
            match counter_event {
                CounterMessage::Increment => {
                    self.value += 1;
                    entity.emit(state, BindEvent::Update);
                    event.consume();
                }

                CounterMessage::Decrement => {
                    self.value -= 1;
                    entity.emit(state, BindEvent::Update);
                    event.consume();
                }
            }            
        }
    }
}



#[derive(Default)]
struct CounterWidget {

}

impl CounterWidget {

    pub fn new() -> Self {
        Self {

        }
    }
}

impl Widget for CounterWidget {
    type Ret = Entity;
    type Data = CounterState;

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        Button::with_label("increment")
            .on_press(|_, state, button|{
                button.emit(state, CounterMessage::Increment);
            })
            .build(state, entity, |builder| builder.class("increment"));

        Button::with_label("decrement")
            .on_press(|_, state, button|{
                button.emit(state,  CounterMessage::Decrement);
            })
            .build(state, entity, |builder| builder.class("decrement"));

        Label::new("0")
            .bind(CounterState::value, |value| value.to_string())
            .build(state, entity, |builder| builder);

        entity.set_element(state, "counter").set_layout_type(state, LayoutType::Row)
    }
}

fn main() {
    // Create the app
    let window_description = WindowDescription::new().with_title("Counter").with_inner_size(400, 100);
    let app = Application::new(window_description, |state, window| {
        state.add_theme(THEME);

        let data_widget = Store::new(CounterState::default()).build(state, window, |builder| builder);

        CounterWidget::new()
            .build(state, data_widget, |builder| builder);

        CounterWidget::new()
            .build(state, data_widget, |builder| builder);
        
        Label::new("Zero")
            .bind(CounterState::value, |value| english_numbers::convert_all_fmt(*value as i64))
            .build(state, data_widget, |builder| 
                builder
                    .set_width(Stretch(1.0))
                    .set_space(Pixels(5.0))
            );
    });

    app.run();
}
