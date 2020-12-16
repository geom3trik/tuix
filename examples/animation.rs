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
        self.button1 = Button::new()
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
    let mut app = Application::new(|window| window.with_title("Animation"));

    let state = &mut app.state;

    let window = state.root;

    state.style.parse_theme(THEME);

    // state.style.insert_style_rule(
    //     StyleRule::new()
    //         .selector(Selector::from("button").class("animated"))
    //         .property(Property::BackgroundColor(Color::rgb(
    //             50, 50, 100,
    //         ))), //.property(Property::Animation("example"))
    //              //.property(Property::AnimationDuration(4.0))
    // );

    // state.style.insert_animation(
    //     AnimationRule::new("example").keyframe(0.0, Property::Left(100.0)).keyframe(1.0, Property::Left(300.0))
    // );

    // state.rules.insert_animation_rule(
    //     AnimationRule::new("example")
    //         .with_duration(std::time::Duration::new(4, 0))
    //         .keyframe(Keyframe::new(0.0).property(Property::BackgroundColor(
    //             nanovg::Color::from_rgb(100, 50, 50),
    //         )))
    //         .keyframe(Keyframe::new(100.0).property(Property::BackgroundColor(
    //             nanovg::Color::from_rgb(50, 100, 50),
    //         ))),
    // );

    Container::new().build(state, window, |builder| builder);

    // let my_button = Button::new()
    //     .build(state, window, &mut app.event_manager)
    //     .element("button")
    //     .class("animated")
    //     //.selector(Selector::from("button").class("animated"))
    //     .set_left(100.0)
    //     .set_top(100.0)
    //     .set_width(100.0)
    //     .set_height(50.0)
    //     .set_background_color(nanovg::Color::from_rgb(100, 50, 50))
    //     .entity();

    // let mut test_animation_state = AnimationState::new(my_button);
    // test_animation_state.set_duration(std::time::Duration::new(1, 0));
    // test_animation_state.set_keyframe((0.0, 100.0));
    // test_animation_state.set_keyframe((1.0, 300.0));

    // state.animator.left.push(test_animation_state);

    app.run();
}
