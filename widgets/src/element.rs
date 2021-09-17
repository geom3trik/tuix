use crate::common::*;

/// The unit widget
///
/// Represents a widget with no behaviour
///
pub struct Element {}

impl Element {
    pub fn new() -> Self {
        Element {}
    }
}

impl Widget for Element {
    type Ret = Entity;
    type Data = ();
    fn on_build(&mut self, _state: &mut State, entity: Entity) -> Self::Ret {
        entity
    }
}
