use crate::{State, Entity, Event, Widget, WindowEvent, PropSet, PropGet};
use tts::*;


#[derive(Debug, Clone, PartialEq)]
pub enum TtsEvent {
    Speak(String, bool),
}

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
    type Data<'a> = TextToSpeachConfig;
    
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity.set_focusable(state, false).set_hoverable(state, false)
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::FocusIn => {
                    
                    let element = event.target.element(state);
                    let name = event.target.name(state);
                    let text = event.target.get_text(state);
                    //let widget_name = state.event_handlers.get(&event.target).map(|widget| widget.widget_name()).unwrap_or(String::new());
                    // if !widget_name.is_empty() {
                    //     println!("Widget: {}", widget_name);
                    //     self.tts.speak(format!("{}", widget_name), true);
                    // }
                    println!("{}", name);
                    self.tts.speak(format!("{}, {}, {}", element, name, text), true);
                }

                _=> {}
            }
        }

        if let Some(tts_event) = event.message.downcast() {
            match tts_event {
                TtsEvent::Speak(text, interrupt) => {
                    self.tts.speak(text.clone(), *interrupt);
                    event.consume();
                }
            }
        }
    }
}