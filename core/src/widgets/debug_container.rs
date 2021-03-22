use crate::style::*;
use crate::widgets::*;

use femtovg::{
    renderer::OpenGl, Align, Baseline, Canvas, FillRule, FontId, ImageFlags, ImageId, LineCap,
    LineJoin, Paint, Path, Renderer, Solidity,
};

pub enum DebugEvent {}
pub struct DebugContainer {
    selected_widget: Entity,
}

impl DebugContainer {
    pub fn new() -> Self {
        Self {
            selected_widget: Entity::default(),
        }
    }
}

impl Widget for DebugContainer {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
            .set_flex_grow(state, 1.0)
            .set_flex_direction(state, FlexDirection::Row);

        let left = Element::new().build(state, entity, |builder| builder.set_flex_grow(1.0));
        let right = Element::new().build(state, entity, |builder| {
            builder.set_flex_basis(Length::Pixels(300.0))
        });

        let panel = Panel::new("Position").build(state, right, |builder| builder);
        // Left
        let row = HBox::new().build(state, panel, |builder| builder);
        Label::new("Left").build(state, row, |builder| builder);
        //ValueSlider::new("value").build(state, row, |builder| builder.set_flex_grow(1.0));
        LengthBox::new().build(state, row, |builder| builder.set_flex_grow(1.0));

        left
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(window_event) = event.message.downcast::<WindowEvent>() {}
    }

    fn on_draw(&mut self, state: &mut State, entity: Entity, canvas: &mut Canvas<OpenGl>) {
        let posx = state.data.get_posx(self.selected_widget);
        let posy = state.data.get_posy(self.selected_widget);
        let width = state.data.get_width(self.selected_widget);
        let height = state.data.get_height(self.selected_widget);

        let mut path = Path::new();
        path.rect(posx, posy, width, height);

        canvas.fill_path(&mut path, Paint::color(femtovg::Color::rgb(100, 50, 50)));
    }
}
