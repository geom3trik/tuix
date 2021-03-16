use crate::{Message};


#[derive(Clone, Debug)]
pub struct Action {
    pub message: Box<dyn Message>,
}
pub trait State: Clone
{
    fn on_action(&self, action: &Action) -> Self {
        self.clone()
    }
}
pub struct Store<S: State> 
{
    state: S,
    action_list: Vec<Action>,
}

impl<S> Store<S> 
where S: State
{
    pub fn new(state: S) -> Self {
        Self {
            state,
            action_list: Vec::new(),
        }
    }

    pub fn process(&mut self) {
        for action in self.action_list.iter() {
            self.state = self.state.on_action(&self.state, action)
        }
        
    }
}

// Example Usage
#[derive(Clone)]
struct AppState {
    counter: i32,
}

pub enum CounterAction {
    Increment,
    Decrement,
}

impl State for AppState {
    fn on_action(&self, action: &Action) -> Self {
        // This acts like a reducer
        if let Some(counter_action) = action.message.downcast::<CounterAction>() {
            match counter_action {
                CounterAction::Increment => {
                    AppState {
                        counter: self.counter + 1,
                    }                    
                }

                CounterAction::Decrement => {
                    AppState {
                        counter: self.counter - 1,
                    }
                }
            }

        }
    }
}