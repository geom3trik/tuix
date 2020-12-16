#![allow(dead_code)]

use crate::entity::Entity;
use crate::mouse::*;

use crate::{Builder, Event, EventHandler, EventManager, WindowEvent};
use crate::{PropSet, State};

use nanovg::Frame;

#[derive(Clone)]
pub struct Node {
    pub id: Entity,
}

impl Node {
    pub fn new() -> Self {
        Node {
            id: Entity::default(),
        }
    }
}

impl EventHandler for Node {
    fn build<'a>(mut self, state: &'a mut State, parent: Entity) -> Builder<'a> {
        self.id = state.add(parent);

        self.id.set_width(state, 100.0).set_height(state, 100.0);
        //.set_background_color(state, nanovg::Color::from_rgb(100, 50, 50));

        state.build(self.id, self).element("node")
    }

    fn handle_event(
        &mut self,
        _id: Entity,
        _state: &mut State,
        event: &Event,
        _event_manager: &mut EventManager,
    ) -> bool {
        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::MouseDown(button, state) => if *button == MouseButton::Left {},
                _ => {}
            }
        }

        false
    }

    fn draw2(&self, state: &mut State, _frame: &Frame) {
        let size = state.style.size.get(self.id).cloned().unwrap_or_default();
        let position = state
            .style
            .positioning
            .get(self.id)
            .cloned()
            .unwrap_or_default();

        println!("DRAW NODE: {}", size.width);

        unsafe {
            gl::Enable(gl::SCISSOR_TEST);
            gl::Viewport(
                position.left as i32,
                position.top as i32 + 600 - size.height as i32,
                size.width as i32,
                size.height as i32,
            );
            gl::Scissor(
                position.left as i32,
                position.top as i32 + 600 - size.height as i32,
                size.width as i32,
                size.height as i32,
            );
            gl::ClearColor(0.3, 0.3, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}
