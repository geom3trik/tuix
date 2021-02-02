use crate::entity::Entity;
use crate::mouse::*;
use crate::State;
use crate::{BuildHandler, Event, EventHandler, WindowEvent};

use crate::style::{Display, Visibility};

use crate::widgets::slider::SliderEvent;
use crate::widgets::Element;

use femtovg::{
    renderer::OpenGl, Baseline, Canvas, Color, FillRule, FontId, ImageFlags, ImageId, LineCap,
    LineJoin, Paint, Path, Renderer, Solidity,
};

pub struct Grid {}

impl Grid {
    pub fn new() -> Self {
        Grid {}
    }
}

impl BuildHandler for Grid {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
    }
}

impl EventHandler for Grid {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        
    }

    fn on_draw(&mut self, state: &mut State, entity: Entity, canvas: &mut Canvas<OpenGl>) {

        // Draw background

        // Draw vertical lines

        // Draw horizontal lines
    }
}
