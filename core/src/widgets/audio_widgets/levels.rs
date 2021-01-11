use crate::entity::Entity;
use crate::mouse::*;
use crate::State;
use crate::{BuildHandler, Event, EventHandler, WindowEvent};

use crate::style::{Display, Length};

use crate::widgets::slider::SliderEvent;
use crate::widgets::Element;

pub struct AudioLevelBar {
    front: Entity,

    level: f32,
}

impl AudioLevelBar {
    pub fn new() -> Self {
        AudioLevelBar {
            front: Entity::null(),

            level: 0.0,
        }
    }
}

impl BuildHandler for AudioLevelBar {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        self.front = Element::new().build(state, entity, |builder| {
            builder.set_height(Length::Percentage(0.5)).class("front")
        });

        state.style.insert_element(entity, "level_bar");

        entity
    }
}

impl EventHandler for AudioLevelBar {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) -> bool {
        return false;
    }
}

// pub struct AudioLevels {

// }
