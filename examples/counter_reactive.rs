use tuix::*;
use fnv::FnvHashMap;
use std::{any::Any, collections::HashSet};

static THEME: &'static str = include_str!("themes/counter_theme.css");


#[derive(Default)]
pub struct CounterState {
    value: i32,
}

#[derive(Default)]
pub struct GeneratedLens_CounterState_value;

impl GeneratedLens_CounterState_value {
    pub const fn new() -> Self {
        Self {}
    }
}

impl CounterState {
    pub const value: GeneratedLens_CounterState_value = GeneratedLens_CounterState_value::new();
}

impl Lens for GeneratedLens_CounterState_value {
    type Source = CounterState;
    type Target = i32;

    fn view(&self, data: &Self::Source) -> Self::Target {
        data.value
    }
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
                }

                CounterMessage::Decrement => {
                    self.value -= 1;
                }
            }            
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BindEvent {
    Bind(Entity),
    Update,
}

pub struct Store<D> {
    data_widget: D,
    observers: HashSet<Entity>,
}

impl<D: Widget> Store<D> {
    pub fn new() -> Self {

    }
}

impl<D: Widget> Widget for Store<D> {
    type Ret = Entity;
    type Data = ();
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        self.data_widget.on_build(state, entity);
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(bind_event) = event.message.downcast() {
            match bind_event {
                BindEvent::Bind(target) => {
                    self.observers.insert(*target);
                }

                BindEvent::Update => {
                    for observer in self.observers.iter() {
                        
                    }
                }
            }
        }

        self.data_widget.on_event_(state, entity, event);
    }
}



#[derive(Default)]
struct CounterWidget {
    value: i32,
    label: Entity,
    data: Entity,
}

impl CounterWidget {

    pub fn new(data_id: Entity) -> Self {
        Self {
            value: 0,
            label: Entity::null(),
            data: data_id,
        }
    }
}

impl Widget for CounterWidget {
    type Ret = Entity;
    type Data = CounterState;

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        Button::with_label("increment")
            .on_press(|_, state, button|{
                button.update(state, Event::new(CounterMessage::Increment));
            })
            .build(state, entity, |builder| builder.class("increment"))
            .bind(state, self.data);

        Button::with_label("decrement")
            .on_press(|_, state, button|{
                button.update(state,  Event::new(CounterMessage::Decrement));
            })
            .build(state, entity, |builder| builder.class("decrement"))
            .bind(state, self.data);

        // self.label = Label::<CounterState>::new(&self.value.to_string())
        //     .with_converter(|data| data.value.to_string())
        //     .build(state, entity, |builder| builder)
        //     .bind(state, self.data);

        Label::new(&self.value.to_string())
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

        let app_data = CounterState::default().build(state, window);

        CounterWidget::new(app_data)
            .build(state, window, |builder| builder);

        CounterWidget::new(app_data)
            .build(state, window, |builder| builder);
        
        // Label::<CounterState>::new("Zero")
        //     .with_converter(|data| english_numbers::convert_all_fmt(data.value as i64))
        //     .build(state, window, |builder| 
        //         builder
        //             .set_width(Stretch(1.0))
        //             .set_space(Pixels(5.0))
        //     )
        //     .bind(state, app_data); 
    });

    app.run();
}
