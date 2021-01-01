extern crate tuix;

use tuix::*;

static THEME: &'static str = include_str!("themes/widget_theme.css");

const ICON_FLOPPY: &str = "\u{1f4be}";
const ICON_PLUS: &str = "\u{2b}";

fn main() {
    // Create the app
    let mut app = Application::new(|win_desc, state, window| {
        state.style.parse_theme(THEME);
        // let checkbox = Checkbox::new(false).build(state, root, |builder| builder.class("widget"));
        //let switch = Switch::new(false).build(state, root, |builder| builder);

        //let dropdown = Dropdown::new()

        // let knob = ControlKnob::new().build(state, root, |builder|
        //     builder
        //         .set_width(Length::Pixels(50.0))
        //         .set_height(Length::Pixels(50.0))
        // );

        // let knob = ValueKnob::new("Dial", 0.0, 0.0, 1.0).build(state, root, |builder|
        //     builder
        //         .set_width(Length::Pixels(50.0))
        //         .set_height(Length::Pixels(75.0))
        // );

        // let gain_input = Textbox::new("0.0").build(state, window, |builder| { 
        //     builder
        //         .set_width(Length::Pixels(40.0))
        //         .set_height(Length::Pixels(20.0))
        // });

        // let audio_level = AudioLevelBar::new().build(state, window, |builder| {
        //     builder
        //         .set_height(Length::Pixels(200.0))
        //         .set_width(Length::Pixels(10.0))
        // });

        // let row = HBox::new().build(state, window, |builder| builder);
        // let eq_channel1 = EQChannel::new(1).build(state, row, |builder| builder);
        // let eq_channel2 = EQChannel::new(2).build(state, row, |builder| builder);
        // let eq_channel3 = EQChannel::new(3).build(state, row, |builder| builder);
        // let eq_channel4 = EQChannel::new(4).build(state, row, |builder| builder);
        // let eq_channel5 = EQChannel::new(5).build(state, row, |builder| builder);
        // let eq_channel6 = EQChannel::new(6).build(state, row, |builder| builder);
        // let eq_channel7 = EQChannel::new(7).build(state, row, |builder| builder);
        // let eq_channel8 = EQChannel::new(8).build(state, row, |builder| builder);

        let eq8 = EQ8::new().build(state, window, |builder| builder.set_flex_grow(1.0));


        win_desc.with_title("basic").with_inner_size(600, 600)
    });

    app.run();
}


pub struct EQ8 {
    
}

impl EQ8 {
    pub fn new() -> Self {
        EQ8 {

        }
    }
}

impl BuildHandler for EQ8 {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        //Header
        let header = HBox::new().build(state, entity, |builder| builder.class("header"));
            //Enabled Checkbox
            let enabled = Checkbox::new(true).build(state, header, |builder| builder.class("enable"));
            let label = Label::new("EQ EIGHT").build(state, header, |builder| builder);
            let preset_dropdown = Dropdown::new("Preset1").build(state, header, |builder| builder);
            let enabled = Checkbox::new(true).with_icon_checked(ICON_FLOPPY).build(state, header, |builder| builder.class("save_preset"));
            let enabled = Checkbox::new(true).with_icon_checked(ICON_PLUS).build(state, header, |builder| builder.class("save_preset"));
        
            // Body
        let body = VBox::new().build(state, entity, |builder| builder.class("body"));
        let (tab_bar, tab_container) = TabContainer::new().build(state, body, |builder| builder);

        Button::with_label("Graph")
            .on_press(Event::new(TabEvent::SwitchTab(0)))
            .build(state, tab_bar, |builder| builder.set_checked(true));
        let graph_view = Element::new().build(state, tab_container, |builder| builder.class("item1"));
        //Button::with_label("First Button").build(state, first, |builder| builder.class("test"));

        Button::with_label("Control")
         .on_press(Event::new(TabEvent::SwitchTab(1)))
         .build(state, tab_bar, |builder| builder);
        let control_view = Button::new().build(state, tab_container, |builder| {
             builder.class("item2").set_display(Display::None)
        });

        let row = HBox::new().build(state, control_view, |builder| builder.set_flex_grow(1.0));
        let eq_channel1 = EQChannel::new(1).build(state, row, |builder| builder);
        let eq_channel2 = EQChannel::new(2).build(state, row, |builder| builder);
        let eq_channel3 = EQChannel::new(3).build(state, row, |builder| builder);
        let eq_channel4 = EQChannel::new(4).build(state, row, |builder| builder);
        let eq_channel5 = EQChannel::new(5).build(state, row, |builder| builder);
        let eq_channel6 = EQChannel::new(6).build(state, row, |builder| builder);
        let eq_channel7 = EQChannel::new(7).build(state, row, |builder| builder);
        let eq_channel8 = EQChannel::new(8).build(state, row, |builder| builder);

        

        // Button::with_label("Second Button").build(state, second, |builder| builder.class("test"));


        state.style.insert_element(entity, "eqeight");

        entity
    }
}

impl EventHandler for EQ8 {

}


pub struct EQChannel {
    active_switch: Entity,
    frequency_knob: Entity,
    gain_knob: Entity,
    q_knob: Entity,
    response_dropdown: Entity,

    channel_number: u32,
}

impl EQChannel {
    pub fn new(channel_number: u32) -> Self {
        EQChannel {
            active_switch: Entity::null(),
            frequency_knob: Entity::null(),
            gain_knob: Entity::null(),
            q_knob: Entity::null(),
            response_dropdown: Entity::null(),

            channel_number,
        }
    }
}

impl BuildHandler for EQChannel {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        self.active_switch = Checkbox::new(true).with_icon_checked(&self.channel_number.to_string()).with_icon_unchecked(&self.channel_number.to_string()).build(state, entity, |builder| builder);
        self.frequency_knob = ValueKnob::new("Freq", 30.0, 0.0, 2000.0).build(state, entity, |builder| builder);
        self.gain_knob = ValueKnob::new("Gain", 0.5, 0.0, 1.0).build(state, entity, |builder| builder);
        self.q_knob = ValueKnob::new("Q", 0.7, 0.0, 1.0).build(state, entity, |builder| builder);
        self.response_dropdown = Dropdown::new("").build(state, entity, |builder| builder).2;

        state.style.insert_element(entity, "eqchannel");

        entity    
    }
}

impl EventHandler for EQChannel {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) -> bool {
        
        if let Some(checkbox_event) = event.message.downcast::<CheckboxEvent>() {
            if event.target == self.active_switch {
                match checkbox_event {
                    CheckboxEvent::Checked => {
                        entity.set_disabled(state, false);
                    }

                    CheckboxEvent::Unchecked => {
                        entity.set_disabled(state, true);
                    }

                    _=> {}
                }
            }
            
        }

        false
    }
}

pub struct FreqGraph {

}

impl FreqGraph {
    pub fn new() -> Self {
        FreqGraph {

        }
    }
}

impl BuildHandler for FreqGraph {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        let min = 1.477121;
        let max = 3.3013;



        entity
    }
}

impl EventHandler for FreqGraph {
    fn on_draw(&mut self, state: &mut State, entity: Entity, canvas: &mut Canvas<OpenGl>) {
        
        
        
        let mut path = Path::new();
        path.rect()
    }
}

