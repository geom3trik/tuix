
use tuix::state::{State};

pub struct Application {
    state: State,
    event_manager: EventManager,
    context: raw_gl_context::GlContext,
}

impl Application {
    pub fn new() -> Self {

    }

    pub fn parented() -> Self {
        
    }
}   