use crate::widgets::*;
pub struct Element {}

impl Element {
    pub fn new() -> Self {
        Element {}
    }
}

impl Widget for Element {
    type Ret = Entity;
    fn on_build(&mut self, builder: Builder) -> Self::Ret {
        builder.entity()
    }
}
