use tuix::*;
use fnv::FnvHashMap;
use std::any::Any;

static THEME: &'static str = include_str!("themes/counter_theme.css");


#[derive(Default)]
pub struct CounterState {
    value: i32,
}

impl Node for CounterState {
    fn get_data(&self) -> Option<&dyn Any> {
        Some(self)
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(counter_event) = event.message.downcast() {
            match counter_event {
                CounterMessage::Increment => {
                    self.value += 1;
                }

                CounterMessage::Decrement => {
                    self.value -= 1;
                }
            }
        }
    }
}

// impl Model for CounterState {
    
//     fn on_build() -> Entity {
//         // Build the internal state
//     }

//     fn on_event() {
//         // Process update events to mutate the internal state
//     }

//     fn data(row: usize, col: usize) -> Any {

//     }

    
// }

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CounterMessage {
    Increment,
    Decrement,
}

#[derive(Default)]
struct CounterWidget {
    value: i32,
    label: Entity,
}

impl CounterWidget {

    pub fn new() -> Self {
        Self {
            value: 0,
            label: Entity::null(),
        }
    }
}

impl Widget for CounterWidget {
    type Ret = Entity;

    // Build
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        Button::with_label("increment")
            .on_press(Event::new(CounterMessage::Increment))
            .build(state, entity, |builder| builder.class("increment"));

        Button::with_label("decrement")
            .on_press(Event::new(CounterMessage::Decrement))
            .build(state, entity, |builder| builder.class("decrement"));

        self.label = Label::new(&self.value.to_string()).build(state, entity, |builder| builder);

        entity.set_element(state, "counter").set_layout_type(state, LayoutType::Row)
    }

    fn on_update(&mut self, state: &mut State, entity: Entity, data: &dyn Any, nodes: &FnvHashMap<Entity, Box<dyn Node>>) {
        if let Some(counter_state) = data.downcast_ref::<CounterState>() {
            // Optional: set local state
            self.value = counter_state.value;
            // Update label
            self.label.set_text(state, &self.value.to_string());
        }
    }

    // Events
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(counter_event) = event.message.downcast::<CounterMessage>() {
            match counter_event {
                CounterMessage::Increment => {
                    state.insert_update(Event::new(CounterMessage::Increment).origin(entity));
                    event.consume();
                }

                CounterMessage::Decrement => {
                    state.insert_update(Event::new(CounterMessage::Decrement).origin(entity));
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

        let app_data = CounterState::default().build(state, window);

        let column = Column::new().build(state, window, |builder| builder);
        CounterWidget::new()
            .build(state, column, |builder| builder)
            .bind(state, app_data);

        CounterWidget::new()
            .build(state, column, |builder| builder)
            .bind(state, app_data);
    });

    app.run();
}
