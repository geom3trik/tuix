use crate::state::Handle;

use crate::{BuildHandler, EventHandler, State};

pub struct Element {}

impl Element {
    pub fn new() -> Self {
        Element {}
    }
}

impl BuildHandler for Element {
    type Ret = Handle;
    fn on_build(&mut self, state: &mut State, handle: Handle) -> Self::Ret {
        handle
    }
}

impl EventHandler for Element {}
