
use tuix::*;
use tuix::widgets::*;

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
        

        let dropdown = Dropdown::<()>::new("Canvas").build(state, entity, |builder| builder);

        dropdown.set_width(state, Auto);

        let row = Row::new().build(state, dropdown, |builder| 
            builder
                .set_height(Pixels(30.0))
                .set_child_space(Pixels(5.0))
                .set_col_between(Pixels(5.0))
        );

        Label::new("Width: ").build(state, row, |builder| builder.set_width(Pixels(50.0)));
        Textbox::new("400").build(state, row, |builder| builder.set_width(Pixels(50.0)));


        // self.popup = Popup::new().build(state, entity, |builder| builder);

        // Element::new().build(state, self.popup, |builder| 
        //     builder
        //         .set_border_radius_top_left(Percentage(50.0))
        //         .set_border_radius_top_right(Percentage(50.0))
        //         .set_border_top_left_shape(BorderCornerShape::Bevel)
        //         .set_border_top_right_shape(BorderCornerShape::Bevel)
        //         .set_background_color(Color::blue())
        // );


        
        
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

