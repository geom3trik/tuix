use crate::{Entity, Event, EventHandler, State, WindowEvent, apply_hover};

use crate::systems::{
    apply_clipping, apply_layout, apply_styles, apply_visibility, apply_z_ordering,
};

#[derive(Clone)]
pub struct WindowWidgetOld {
    
}

impl WindowWidgetOld {
    pub fn new() -> Self {
        WindowWidgetOld {}
    }

    pub fn build_window(self, state: &mut State) {
        state.build(Entity::root(), self);
    }
}

impl EventHandler for WindowWidgetOld {
    fn on_event(&mut self, state: &mut State, _entity: Entity, event: &mut Event) {
        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::WindowClose => {
                    println!("Window Close Event");
                }

                WindowEvent::Restyle => {
                    //println!("Restyle");
                    //apply_styles2(state, &state.hierarchy.clone(), event.origin);
                    apply_styles(state, &state.hierarchy.clone());
                    apply_visibility(state, &state.hierarchy.clone());
                }

                WindowEvent::Relayout => {
                    //println!("Relayout");
                    apply_z_ordering(state, &state.hierarchy.clone());
                    apply_visibility(state, &state.hierarchy.clone());
                    apply_clipping(state, &state.hierarchy.clone());
                    apply_layout(state, &state.hierarchy.clone());
                    apply_hover(state);
                }

                WindowEvent::Redraw => {}

                _ => {}
            }
        }
    }
}
