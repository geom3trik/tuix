use crate::style::*;
use crate::widgets::*;

#[derive(Debug, Clone, PartialEq)]
pub enum AudioLevelEvent {
    SetLevel(f32),
}

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

impl Widget for AudioLevelBar {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        self.front = Element::new().build(state, entity, |builder| {
            builder.class("front")
            //.set_height(Length::Percentage(1.0))
        });

        entity.set_element(state, "level_bar")
    }

    fn on_event(&mut self, state: &mut State, _entity: Entity, event: &mut Event) {
        if let Some(audio_level_event) = event.message.downcast::<AudioLevelEvent>() {
            match audio_level_event {
                AudioLevelEvent::SetLevel(val) => {
                    self.level = *val;
                    let level_db = 1.0 + (20.0 * self.level.abs().log10()).max(-60.0) / 60.0;
                    self.front.set_height(state, Units::Percentage(level_db));
                }
            }
        }
    }
}
