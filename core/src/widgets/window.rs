use crate::{Entity, Event, EventHandler, State, WindowEvent};

use crate::systems::{
    apply_clipping, apply_layout, apply_styles, apply_visibility, apply_z_ordering,
};

#[derive(Clone)]
pub struct WindowWidget {}

impl WindowWidget {
    pub fn new() -> Self {
        WindowWidget {}
    }

    pub fn build_window(self, state: &mut State) {
        state.build(Entity::new(0, 0), self);
        //state.shared_state.borrow_mut().event_handlers.insert(Entity::new(0,0), self);
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
                    let hierarchy = state.hierarchy.clone();
                    //println!("Restyle");
                    //apply_styles2(state, &state.hierarchy.clone(), event.origin);
                    apply_styles(state, &hierarchy);
                    //apply_visibility(state, &state.hierarchy.clone());
                }

                WindowEvent::Relayout => {
                    let hierarchy = state.hierarchy.clone();
                    apply_z_ordering(state, &hierarchy);
                    apply_visibility(state, &hierarchy);
                    apply_clipping(state, &hierarchy);
                    apply_layout(state, &hierarchy);
                }

                WindowEvent::Redraw => {}

                _ => {}
            }
        }

        false
    }
}
