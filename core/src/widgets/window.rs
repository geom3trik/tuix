use crate::{apply_hover, Entity, Event, State, Widget, WindowEvent, Context};

use crate::systems::{
    apply_layout, apply_styles, apply_visibility, apply_z_ordering,
};

#[derive(Clone)]
pub struct WindowWidget {}

impl WindowWidget {
    pub fn new() -> Self {
        WindowWidget {}
    }

    pub fn build_window(self, state: &mut State) {
        state.build(Entity::root(), self);
    }
}

impl Widget for WindowWidget {
    type Ret = ();

    fn on_build(&mut self, context: Context<Self::Ret>) -> Self::Ret {

    }

    fn on_event(&mut self, state: &mut State, _entity: Entity, event: &mut Event) {
        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::WindowClose => {
                    println!("Window Close Event");
                }

                WindowEvent::Restyle => {
                    //state.needs_restyle = true;
                    //println!("Restyle");
                    //apply_styles2(state, &state.hierarchy.clone(), event.origin);
                    // apply_styles(state, &state.hierarchy.clone());
                    // apply_visibility(state, &state.hierarchy.clone());
                    let hierarchy = state.hierarchy.clone();
                    apply_styles(state, &hierarchy);
                }

                WindowEvent::Relayout => {
                    //state.needs_relayout = true;
                    let hierarchy = state.hierarchy.clone();
                    state.needs_redraw = true;
                    //println!("Relayout");
                    // apply_z_ordering(state, &state.hierarchy.clone());
                    // apply_visibility(state, &state.hierarchy.clone());
                    // apply_clipping(state, &state.hierarchy.clone());
                    // apply_layout(state, &state.hierarchy.clone());
                    // apply_hover(state);
                    apply_z_ordering(state, &hierarchy);
                    apply_visibility(state, &hierarchy);
                    apply_layout(state, &hierarchy);
                    apply_hover(state);
                }

                WindowEvent::Redraw => {
                    state.needs_redraw = true;
                }

                _ => {}
            }
        }
    }
}
