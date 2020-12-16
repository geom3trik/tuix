use crate::entity::Entity;

use crate::{Builder, Event, EventHandler, EventManager, Message};
use crate::{PropSet, State};

use crate::state::style::color::Color;

use nanovg::Frame;

#[derive(Debug, Copy, Clone)]
pub enum CanvasEvent {
    SetWidth(f32),
    SetHeight(f32),
}

impl Message for CanvasEvent {}


pub struct Canvas {
    border: Entity,
}

impl Canvas {
    pub fn new() -> Self {
        Canvas {
            border: Entity::null(),
        }
    }
}

impl EventHandler for Canvas {
    fn build<'a>(
        mut self,
        state: &'a mut State,
        parent: Entity,
    ) -> Builder<'a> {
        let id = state.add(parent);

        self.border = state.add(id);

        self.border
            //.set_border_color(state, Color::rgb(50, 50, 50))
            //.set_border_width(state, 2.0)
            .set_width(state, 300.0)
            .set_height(state, 300.0);

        state
            .build(id, self)
            .element("canvas")
    }

    fn handle_event(
        &mut self,
        id: Entity,
        state: &mut State,
        event: &Event,
        _event_manager: &mut EventManager,
    ) -> bool {
        if let Some(canvas_event) = event.message.downcast::<CanvasEvent>() {
            if event.target == id {
                match canvas_event {
                    CanvasEvent::SetWidth(width) => {
                        self.border.set_width(state, *width);
                    }

                    CanvasEvent::SetHeight(width) => {
                        self.border.set_height(state, *width);
                    }
                }
            }
        }

        return false;
    }

    fn draw2(&self, _state: &mut State, frame: &Frame) {
        frame.path(
            |path| {
                path.rect((2.0, 2.0), (300.0, 300.0));
                path.fill(nanovg::Color::from_rgb(100, 100, 0), Default::default())
            },
            Default::default(),
        );
    }
}
