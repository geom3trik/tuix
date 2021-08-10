use crate::widgets::*;
use tts::*;

pub struct TextToSpeachConfig {

}

pub struct TextToSpeach {
    tts: Tts,
}

impl TextToSpeach {
    pub fn new() -> Self {
        Self {
            tts: Tts::default().expect("Failed to load TTS"),
        }
    } 
}

impl Widget for TextToSpeach {
    type Ret = Entity;
    type Data = TextToSpeachConfig;
    
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::FocusIn => {
                    println!("Entity: {}", event.target);
                    self.tts.speak(format!("Entity: {}", event.target), true);
                }

                _=> {}
            }
        }
    }
}