
use tuix::*;
use tuix::widgets::*;

pub struct BorderWidget {

}

impl BorderWidget {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl Widget for BorderWidget {
    type Ret = Entity;
    type Data = ();

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {


        Textbox::new("TL").build(state, entity, |builder|
            builder
                .set_col_index(0)
                .set_row_index(0)
        );

        Textbox::new("TR").build(state, entity, |builder|
            builder
                .set_col_index(2)
                .set_row_index(0)
        );

        Textbox::new("BL").build(state, entity, |builder|
            builder
                .set_col_index(0)
                .set_row_index(2)
        );

        Textbox::new("BR").build(state, entity, |builder|
            builder
                .set_col_index(2)
                .set_row_index(2)
        );

        Element::new().build(state, entity, |builder|
            builder
                .set_col_index(1)
                .set_row_index(0)
                .set_row_span(3)
                .set_border_color(Color::white())
                .set_border_width(Pixels(1.0))
        );


        entity
            .set_layout_type(state, LayoutType::Grid)
            .set_grid_cols(state, vec![Pixels(100.0), Stretch(1.0), Pixels(100.0)])
            .set_grid_rows(state, vec![Pixels(30.0), Stretch(1.0), Pixels(30.0)])
            .set_col_between(state, Pixels(10.0))
    }


}