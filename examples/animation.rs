extern crate tuix;
use tuix::*;

static THEME: &'static str = include_str!("themes/animation_theme.css");

fn main() {
    let mut app = Application::new(|win_desc, state, window| {
        state.style.parse_theme(THEME);



        Animations::new().build(state, window, |builder| builder);

        win_desc.with_title("Animation")
    });

    app.run();
}

pub struct Animations {

    element: Entity,

    // Buttons
    background_color_button: Entity,
    position_button: Entity,
    size_button: Entity,
    margins_button: Entity,
    border_button: Entity,
    padding_button: Entity,
    border_radius_button: Entity,

    // Animations
    background_color_animation: usize,
    position_left_animation: usize,
    position_top_animation: usize,
    width_animation: usize,
    height_animation: usize,
    margin_left_animation: usize,
    margin_right_animation: usize,
    margin_top_animation: usize,
    margin_bottom_animation: usize,
    padding_left_animation: usize,
    padding_right_animation: usize,
    padding_top_animation: usize,
    padding_bottom_animation: usize,
    border_animation: usize,
    border_top_left_animation: usize,
    border_bottom_right_animation: usize,

       
}

impl Animations {
    pub fn new() -> Self {
        Animations {
            element: Entity::null(),

            background_color_button: Entity::null(),
            position_button: Entity::null(),
            size_button: Entity::null(),
            margins_button: Entity::null(),
            border_button: Entity::null(),
            padding_button: Entity::null(),
            border_radius_button: Entity::null(),

            background_color_animation: std::usize::MAX,
            position_left_animation: std::usize::MAX,
            position_top_animation: std::usize::MAX,
            width_animation: std::usize::MAX,
            height_animation: std::usize::MAX,
            margin_left_animation: std::usize::MAX,
            margin_right_animation: std::usize::MAX,
            margin_top_animation: std::usize::MAX,
            margin_bottom_animation: std::usize::MAX,
            padding_left_animation: std::usize::MAX,
            padding_right_animation: std::usize::MAX,
            padding_top_animation: std::usize::MAX,
            padding_bottom_animation: std::usize::MAX,
            border_animation: std::usize::MAX,
            border_top_left_animation: std::usize::MAX,
            border_bottom_right_animation: std::usize::MAX,
        }
    }
}

impl BuildHandler for Animations {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        entity.set_flex_grow(state, 1.0);
        
        let hbox = HBox::new().build(state, entity, |builder|
            builder
                .set_flex_grow(1.0)
        );

        let vbox = VBox::new().build(state, hbox, |builder| 
            builder
                //.set_flex_grow(1.0)
                //.set_flex_shrink(0.0)
                .set_background_color(Color::rgb(50,100,50))
        );

        let container = Element::new().build(state, hbox, |builder|
            builder
                .set_flex_grow(4.0)
        );

        self.element = Element::new().build(state, container, |builder|
            builder.class("element")
        );

        Element::new().build(state, self.element, |builder|
            builder.class("subelement")
        );

        // Background Color Animation
        let background_color_animation_state = AnimationState::new()
            .with_duration(std::time::Duration::from_secs(2))
            .with_keyframe((0.0, Color::rgb(100,50,50)))
            .with_keyframe((1.0, Color::rgb(50,50,100)))
            .set_persistent(true);

        self.background_color_animation = state.style.background_color.insert_animation(background_color_animation_state);

        self.background_color_button = Button::with_label("Background Color").build(state, vbox, |builder| builder);

        // Position Animation
        let position_animation_state = AnimationState::new()
            .with_duration(std::time::Duration::from_secs(2))
            .with_keyframe((0.0, Length::Pixels(0.0)))
            .with_keyframe((1.0, Length::Pixels(100.0)))
            .set_persistent(true);

        // Add the same animation to two different properties (need to clone)
        self.position_left_animation = state.style.left.insert_animation(position_animation_state.clone());
        self.position_top_animation = state.style.top.insert_animation(position_animation_state.clone());

        self.position_button = Button::with_label("Position").build(state, vbox, |builder| builder);

        // Size Animation
        let size_animation_state = AnimationState::new()
            .with_duration(std::time::Duration::from_secs(2))
            .with_keyframe((0.0, Length::Pixels(50.0)))
            .with_keyframe((1.0, Length::Pixels(200.0)))
            .set_persistent(true);

        self.width_animation = state.style.width.insert_animation(size_animation_state.clone());
        // Reuse animation state from position animation above
        self.height_animation = state.style.height.insert_animation(position_animation_state.clone());

        self.size_button = Button::with_label("Size").build(state, vbox, |builder| builder);


        // Margins, Padding, Border & Border Radius Animation
        let animation_state = AnimationState::new()
            .with_duration(std::time::Duration::from_secs(1))
            .with_keyframe((0.0, Length::Pixels(0.0)))
            .with_keyframe((1.0, Length::Pixels(10.0)))
            .set_persistent(true);
        
        self.margin_left_animation = state.style.margin_left.insert_animation(animation_state.clone());
        self.margin_right_animation = state.style.margin_right.insert_animation(animation_state.clone());
        self.margin_top_animation = state.style.margin_top.insert_animation(animation_state.clone());
        self.margin_bottom_animation = state.style.margin_bottom.insert_animation(animation_state.clone());

        self.margins_button = Button::with_label("Margins").build(state, vbox, |builder| builder);

        self.border_animation = state.style.border_width.insert_animation(animation_state.clone());

        self.border_button = Button::with_label("Border Width").build(state, vbox, |builder| builder);

        self.padding_left_animation = state.style.padding_left.insert_animation(animation_state.clone());
        self.padding_right_animation = state.style.padding_right.insert_animation(animation_state.clone());
        self.padding_top_animation = state.style.padding_top.insert_animation(animation_state.clone());
        self.padding_bottom_animation = state.style.padding_bottom.insert_animation(animation_state.clone());

        self.padding_button = Button::with_label("Padding").build(state, vbox, |builder| builder);

        self.border_top_left_animation = state.style.border_radius_top_left.insert_animation(animation_state.clone());
        self.border_bottom_right_animation = state.style.border_radius_bottom_right.insert_animation(animation_state.clone());

        self.border_radius_button = Button::with_label("Border Radius").build(state, vbox, |builder| builder);


        entity
    }
}

impl EventHandler for Animations {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) -> bool {

        if let Some(button_event) = event.message.downcast::<ButtonEvent>() {
            match button_event {
                ButtonEvent::ButtonPressed => {
                    if event.target == self.background_color_button {
                        state.style.background_color.play_animation(self.element, self.background_color_animation);
                        self.background_color_button.set_checked(state, true);
                    } else {
                        self.background_color_button.set_checked(state, false);
                    }

                    if event.target == self.position_button {
                        state.style.left.play_animation(self.element, self.position_left_animation);
                        state.style.top.play_animation(self.element, self.position_top_animation);
                        self.position_button.set_checked(state, true);
                    } else {
                        self.position_button.set_checked(state, false);
                    }

                    if event.target == self.size_button {
                        state.style.width.play_animation(self.element, self.width_animation);
                        state.style.height.play_animation(self.element, self.height_animation);
                        self.size_button.set_checked(state, true);
                    } else {
                        self.size_button.set_checked(state, false);
                    }

                    if event.target == self.margins_button {
                        state.style.margin_left.play_animation(self.element, self.margin_left_animation);
                        state.style.margin_right.play_animation(self.element, self.margin_right_animation);
                        state.style.margin_top.play_animation(self.element, self.margin_top_animation);
                        state.style.margin_bottom.play_animation(self.element, self.margin_bottom_animation);
                        
                        self.margins_button.set_checked(state, true);
                    } else {
                        self.margins_button.set_checked(state, false);
                    }

                    if event.target == self.border_button {
                        state.style.border_width.play_animation(self.element, self.border_animation);
                        self.border_button.set_checked(state, true);
                    } else {
                        self.border_button.set_checked(state, false);
                    }

                    if event.target == self.padding_button {
                        state.style.padding_left.play_animation(self.element, self.padding_left_animation);
                        state.style.padding_top.play_animation(self.element, self.padding_top_animation);
                        state.style.padding_right.play_animation(self.element, self.padding_right_animation);
                        state.style.padding_bottom.play_animation(self.element, self.padding_bottom_animation);
                        self.padding_button.set_checked(state, true);
                    } else {
                        self.padding_button.set_checked(state, false);
                    }

                    if event.target == self.border_radius_button {
                        state.style.border_radius_top_left.play_animation(self.element, self.border_top_left_animation);
                        state.style.border_radius_bottom_right.play_animation(self.element, self.border_bottom_right_animation);
                        self.border_radius_button.set_checked(state, true);
                    } else {
                        self.border_radius_button.set_checked(state, false);
                    }
                }

                _=> {}
            }
            
        }

        false
    }
}