use tuix::*;

use tuix::style::themes::DEFAULT_THEME;
// An example for demonstrating the addition and removal of entities

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AddRemoveEvent {
    Add,
    Remove,
}

#[derive(Default)]
struct Controller {}

impl Widget for Controller {
    type Ret = Entity;
    type Data = ();
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        state.focused = entity;

        entity
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(counter_event) = event.message.downcast::<AddRemoveEvent>() {
            match counter_event {
                AddRemoveEvent::Add => {
                    Button::new().build(state, entity, |builder| {
                        builder.set_height(Units::Pixels(30.0)).set_space(Pixels(5.0))
                    });
                }

                AddRemoveEvent::Remove => {
                    if let Some(first_child) = state.tree.get_first_child(entity) {
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
    let app = Application::new(WindowDescription::new().with_title("Add / Remove"),|state, window| {
        state.add_theme(DEFAULT_THEME);

        Controller::default().build(state, window.entity(), |builder| builder);
    });

    app.run();
}
