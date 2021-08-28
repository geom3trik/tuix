use tuix_core::Position;

use crate::{Button, Label, common::*};


pub struct PopupWindow {
    title: String,

}

impl PopupWindow {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_owned(),
        }
    }
}

impl Widget for PopupWindow {
    /// Returns an id to the conatiner and to the header as a tuple
    type Ret = (Entity, Entity);
    type Data = ();
    
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        let header = Element::new().build(state, entity, |builder|
            builder
                .set_layout_type(LayoutType::Row)
                .set_height(Pixels(30.0))
                .class("header")
        );

        let label = Label::new(&self.title).build(state, header, |builder |
            builder
                .set_child_space(Stretch(1.0))
                .class("label")
        );

        Button::with_label("X").build(state, header, |builder|
            builder
                .set_width(Pixels(30.0))
        );

        let container = Element::new().build(state, entity, |builder|
            builder
                .class("container")
        );

        entity.set_element(state, "window").set_position_type(state, PositionType::SelfDirected);


        (container, header)
    }
}