use crate::{State, EventHandler, Entity, Event, WindowEvent};

use crate::systems::{apply_clipping, apply_z_ordering, apply_styles, apply_visibility, apply_layout};

#[derive(Clone)]
pub struct WindowWidget {}

impl WindowWidget {
    pub fn new() -> Self {
        WindowWidget {}
    }

    pub fn build_window(self, state: &mut State) {
        state.build(state.root, self);
    }
}

impl EventHandler for WindowWidget {
    fn on_event(&mut self, state: &mut State, _entity: Entity, event: &mut Event) -> bool {
        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::WindowClose => {
                    println!("Window Close Event");
                }

                WindowEvent::Restyle => {
                    //println!("Restyle");
                    //apply_styles2(state, &state.hierarchy.clone(), event.origin);
                    apply_styles(state, &state.hierarchy.clone());
                    //apply_visibility(state, &state.hierarchy.clone());
                }

                WindowEvent::Relayout => {
                    apply_z_ordering(state, &state.hierarchy.clone());
                    apply_visibility(state, &state.hierarchy.clone());
                    apply_clipping(state, &state.hierarchy.clone());
                    apply_layout(state, &state.hierarchy.clone());
                }

                _ => {}
            }
        }

        false
    }
}