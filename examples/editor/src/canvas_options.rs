
use tuix::*;

#[derive(Default)]
pub struct CanvasOptionsDropdown {
    popup: Entity,
}

impl CanvasOptionsDropdown {
    pub fn new() -> Self {
        Self {
            popup: Entity::null(),
        }
    }
}

impl Widget for CanvasOptionsDropdown {
    type Ret = Entity;
    type Data = ();

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        
        self.popup = Popup::new().build(state, entity, |builder| builder);

        Element::new().build(state, self.popup, |builder| 
            builder
                .set_border_radius_top_left(Percentage(50.0))
                .set_border_radius_top_right(Percentage(50.0))
                .set_border_top_left_shape(BorderCornerShape::Bevel)
                .set_border_top_right_shape(BorderCornerShape::Bevel)
                .set_background_color(Color::blue())
        );
        
        
        entity
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::MouseDown(button) => {

                }

                _=> {}
            }
        }
    }
}

