use tuix::*;

use std::cell::RefCell;
use std::{marker::PhantomData, rc::Rc};

use std::any::Any;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Todo {
    id: u32,
    text: String,
    completed: bool,
}

#[derive(Default)]
pub struct AppState {
    todos: Rc<RefCell<Vec<Todo>>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TodoEvent {
    Add,
    Remove,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataEvent {
    TodoChanged(Rc<RefCell<Vec<Todo>>>),
}

#[derive(Default)]
pub struct StateHandler {
    //app_state: AppState,
    todos: Vec<Todo>,
}

impl Widget for StateHandler {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
    }

    fn on_event(
        &mut self,
        state: &mut State,
        entity: Entity,
        parent: &Box<dyn EventHandler>,
        event: &mut Event,
    ) {
        if let Some(todo_event) = event.message.downcast::<TodoEvent>() {
            match todo_event {
                TodoEvent::Add => {
                    println!("Received add event");
                    //self.app_state.todos.borrow_mut().push(Todo::default());
                    self.todos.push(Todo::default());

                    //state.insert_event(Event::new(DataEvent::TodoChanged(self.app_state.todos.clone())).target(entity).propagate(Propagation::Fall));

                    event.consume();
                }

                TodoEvent::Remove => {}
            }
        }
    }
}

#[derive(Default)]
pub struct TodoList {}

impl Widget for TodoList {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        state.focused = entity;
        entity
    }

    fn on_test(&self) {
        println!("TodoList");
    }

    fn on_event(
        &mut self,
        state: &mut State,
        entity: Entity,
        parent: &Box<dyn EventHandler>,
        event: &mut Event,
    ) {
        // if let Some(data_event) = event.message.downcast::<DataEvent>() {
        //     match data_event {
        //         DataEvent::TodoChanged(data) => {
        //             println!("A todo item was added! {:?}", data);
        //         }
        //     }
        // }

        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::KeyDown(code, key) => match code {
                    Code::KeyA => {
                        println!("Pessed A key");
                        state.insert_event(Event::new(TodoEvent::Add).target(entity));
                        if let Some(state_handler) = parent.downcast_ref::<StateHandler>() {
                            println!("State Handler: {:?}", state_handler.todos);
                        } else {
                            println!("Failed");
                        }
                    }

                    _ => {}
                },

                _ => {}
            }
        }
    }
}

fn main() {
    let app = Application::new(|wind_desc, state, window| {
        let state_handler = StateHandler::default().build(state, window, |builder| builder);
        TodoList::default().build(state, state_handler, |builder| builder);

        wind_desc
    });

    app.run();
}
