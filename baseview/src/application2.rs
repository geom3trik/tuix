use tuix_core::{Entity, EventManager, State, WindowDescription};

pub struct Application2 {
    state: State,
    event_manager: EventManager,
    context: raw_gl_context::GlContext,
}

impl Application2 {
    pub fn new<F>() -> Self
    where
        F: FnMut(WindowDescription),
    {
    }

    pub fn parented() -> Self {}
}
