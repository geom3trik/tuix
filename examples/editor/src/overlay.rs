

use tuix::*;
use femtovg::{Path, Paint};

pub struct Overlay {
    selected: Entity,
}

impl Overlay {
    pub fn new() -> Self {
        Self {
            selected: Entity::null(),
        }
    }
}

impl Widget for Overlay {
    type Ret = Entity;
    type Data = Entity;

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity.set_hoverable(state, false)
    }

    fn on_update(&mut self, state: &mut State, entity: Entity, data: &Self::Data) {
        if *data != self.selected {
            self.selected = *data;
        }
    }

    fn on_draw(&mut self, state: &mut State, entity: Entity, canvas: &mut femtovg::Canvas<femtovg::renderer::OpenGl>) {

        if self.selected == Entity::null() {
            return;
        }

        let parent = self.selected.parent(&state.tree).unwrap();

        let parent_posx = state.data.get_posx(parent); 
        let parent_posy = state.data.get_posy(parent); 
        let parent_width = state.data.get_width(parent); 
        let parent_height = state.data.get_height(parent);
        
        let selected_posx = state.data.get_posx(self.selected);
        let selected_posy = state.data.get_posy(self.selected);
        let selected_width = state.data.get_width(self.selected);
        let selected_height = state.data.get_height(self.selected);

        // Draw height line
        let mut path = Path::new();
        path.move_to(selected_posx - 8.0, selected_posy);
        path.line_to(selected_posx - 2.0, selected_posy);
        canvas.stroke_path(&mut path, Paint::color(femtovg::Color::rgb(255, 0, 255)));

        let mut path = Path::new();
        path.move_to(selected_posx - 5.0, selected_posy);
        path.line_to(selected_posx - 5.0, selected_posy + selected_height);
        canvas.stroke_path(&mut path, Paint::color(femtovg::Color::rgb(255, 0, 255)));

        let mut path = Path::new();
        path.move_to(selected_posx - 8.0, selected_posy + selected_height);
        path.line_to(selected_posx - 2.0, selected_posy + selected_height);
        canvas.stroke_path(&mut path, Paint::color(femtovg::Color::rgb(255, 0, 255)));

        let mut path = Path::new();
        path.rect(selected_posx - 30.0, selected_posy + selected_height / 2.0 - 10.0, 20.0, 20.0);
        canvas.stroke_path(&mut path, Paint::color(femtovg::Color::rgb(255, 0, 255)));
    }
}