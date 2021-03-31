use tuix::*;

use tuix::style::themes::DEFAULT_THEME;
// An example for demonstrating the addition and removal of entities

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AddRemoveEvent {
    Add,
    Remove,
}

#[derive(Default)]
struct Counter {}

impl Counter {
    pub fn new() -> Self {
        Counter {}
    }
}

impl Widget for Counter {
    type Ret = Entity;

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        println!("Entity Index: {}", entity);

        state.focused = entity;
        entity.set_flex_grow(state, 1.0)
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(counter_event) = event.message.downcast::<AddRemoveEvent>() {
            match counter_event {
                AddRemoveEvent::Add => {
                    Button::new().build(state, entity, |context| {
                        context.set_height(Length::Pixels(30.0))
                    });
                }

                AddRemoveEvent::Remove => {
                    if let Some(first_child) = state.hierarchy.get_first_child(entity) {
                        state.remove(first_child);
                    }
                }
            }
        }

        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::KeyDown(code, key) => {
                    if *code == Code::KeyA {
                        state.insert_event(Event::new(AddRemoveEvent::Add).target(entity));
                    }

                    if *code == Code::KeyS {
                        state.insert_event(Event::new(AddRemoveEvent::Remove).target(entity));
                    }
                }

                _ => {}
            }
        }
    }
}

fn main() {
    // Create the app
    let app = Application::new(|state, window| {
        state.add_theme(DEFAULT_THEME);


        window.set_title("Counter").set_inner_size(400, 100);

        Counter::new().build(state, window.entity(), |context| context);
        
    });

    app.run();
}
