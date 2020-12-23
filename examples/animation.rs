extern crate tuix;
use tuix::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TriggerEvent {
    Start,
}

struct Container {
    button1: Entity,
    button2: Entity,

    background_color_animation: usize,
    left_animation: usize,
    height_animation: usize,
    opacity_animation: usize,

    rotation_animation: usize,
}

impl Container {
    pub fn new() -> Self {
        Container {
            button1: Entity::null(),
            button2: Entity::null(),

            background_color_animation: std::usize::MAX,
            left_animation: std::usize::MAX,
            height_animation: std::usize::MAX,
            opacity_animation: std::usize::MAX,

            rotation_animation: std::usize::MAX,
        }
    }
}

impl BuildHandler for Container {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        self.button1 = Button::with_label("Press Me")
            .on_press(Event::new(TriggerEvent::Start))
            .build(state, entity, |builder| builder.class("first"));

        self.button2 =
            Button::with_label("Test").build(state, entity, |builder| builder.class("second"));

        Button::new().build(state, self.button2, |builder| builder.class("third"));

        let background_color_animation = AnimationState::new()
            .with_duration(std::time::Duration::from_secs(2))
            .with_keyframe((0.0, Color::rgb(50, 50, 100)))
            .with_keyframe((1.0, Color::rgb(50, 100, 50)));

        self.background_color_animation = state
            .style
            .background_color
            .insert_animation(background_color_animation);

        let left_animation = AnimationState::new()
            .with_duration(std::time::Duration::from_secs(2))
            .with_keyframe((0.0, Length::Pixels(0.0)))
            .with_keyframe((1.0, Length::Pixels(100.0)));

        self.left_animation = state.style.left.insert_animation(left_animation);

        let height_animation = AnimationState::new()
            .with_duration(std::time::Duration::from_secs(2))
            .with_keyframe((0.0, Length::Pixels(50.0)))
            .with_keyframe((0.0, Length::Pixels(100.0)));

        self.height_animation = state.style.height.insert_animation(height_animation);

        let opacity_animation = AnimationState::new()
            .with_duration(std::time::Duration::from_secs(2))
            .with_keyframe((0.0, Opacity(0.0)))
            .with_keyframe((0.0, Opacity(1.0)));

        self.opacity_animation = state.style.opacity.insert_animation(opacity_animation);

        let rotation_animation = AnimationState::new()
            .with_duration(std::time::Duration::from_secs(2))
            .with_keyframe((0.0, 0.0))
            .with_keyframe((1.0, 90.0));

        self.rotation_animation = state.style.rotate.insert_animation(rotation_animation);

        entity
    }
}

impl EventHandler for Container {
    fn on_event(&mut self, state: &mut State, _entity: Entity, event: &mut Event) -> bool {
        if let Some(trigger_event) = event.message.downcast::<TriggerEvent>() {
            match trigger_event {
                TriggerEvent::Start => {
                    if event.target == self.button1 {
                        println!("Trigger Animation Here");

                        // state.style.background_color.play_animation(self.button2, self.background_color_animation);
                        // state.style.left.play_animation(self.button2, self.left_animation);
                        // state.style.height.play_animation(self.button2, self.height_animation);
                        // state.style.opacity.play_animation(self.button2, self.opacity_animation);
                        state
                            .style
                            .rotate
                            .play_animation(self.button2, self.rotation_animation);
                    }
                }
            }
        }

        false
    }
}

static THEME: &'static str = include_str!("themes/animation_theme.css");

fn main() {
    let mut app = Application::new(|win_desc, state, window| {

        state.style.parse_theme(THEME);


        Container::new().build(state, window, |builder| builder);


        win_desc.with_title("Animation")
    });





    app.run();
}
