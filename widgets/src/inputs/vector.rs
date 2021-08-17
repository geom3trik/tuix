
use crate::common::*;

pub struct Vector {
    // Subwidgets
    textbox_x: Entity,
    textbox_y: Entity,
    textbox_z: Entity,

    indicator_x: Entity,
    indicator_y: Entity,
    indicator_z: Entity,

    // Animations
    grow: Animation,
    shrink: Animation,

    // Data
    pub x: f32,
    pub y: f32,
    pub z: f32,

    // Callbacks
    on_change: Option<Box<dyn Fn(&mut Self, &mut State, Entity)>>,
    shrink_flag: bool,
}

impl Vector {
    pub fn new() -> Self {
        Self {
            textbox_x: Entity::null(),
            textbox_y: Entity::null(),
            textbox_z: Entity::null(),

            indicator_x: Entity::null(),
            indicator_y: Entity::null(),
            indicator_z: Entity::null(),

            grow: Animation::default(),
            shrink: Animation::default(),

            x: 0.0,
            y: 0.0,
            z: 0.0,

            on_change: None,
            shrink_flag: false,
        }
    }

    pub fn with_x(mut self, val: f32) -> Self {
        self.x = val;

        self
    }

    pub fn with_y(mut self, val: f32) -> Self {
        self.y = val;

        self
    }

    pub fn with_z(mut self, val: f32) -> Self {
        self.z = val;

        self
    }

    pub fn on_change<F>(mut self, callback: F) -> Self
    where
        F: 'static + Fn(&mut Self, &mut State, Entity),
    {
        self.on_change = Some(Box::new(callback));

        self
    }
}

impl Widget for Vector {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        entity
            .set_layout_type(state, LayoutType::Row)
            //.set_child_between(state, Pixels(5.0))
            .set_element(state, "vector");

        self.indicator_x = Label::new("x")
            .build(state, entity, |builder|
                builder
                    .set_background_color(Color::rgb(200, 50, 50))
                    .set_color(Color::white())
                    .set_width(Pixels(20.0))
                    .set_height(Stretch(1.0))
                    .set_child_space(Stretch(1.0))
        );

        self.textbox_x = Textbox::new(&self.x.to_string())
            .build(state, entity, |builder| 
                builder
                    .set_right(Pixels(5.0))
        );

        self.indicator_y = Label::new("y")
            .build(state, entity, |builder|
                builder
                    .set_background_color(Color::rgb(50, 200, 50))
                    .set_color(Color::white())
                    .set_width(Pixels(20.0))
                    .set_height(Stretch(1.0))
                    .set_child_space(Stretch(1.0))
        );

        self.textbox_y = Textbox::new(&self.y.to_string())
            .build(state, entity, |builder| {
                builder
                    .set_right(Pixels(5.0))
        });

        self.indicator_z = Label::new("z")
            .build(state, entity, |builder|
                builder
                    .set_background_color(Color::rgb(50, 50, 200))
                    .set_color(Color::white())
                    .set_width(Pixels(20.0))
                    .set_height(Stretch(1.0))
                    .set_child_space(Stretch(1.0))
        );

        self.textbox_z = Textbox::new(&self.z.to_string())
            .build(state, entity, |builder| {
                builder
                    .set_right(Pixels(5.0))
        });

        self.grow = state.style.width.insert_animation(
            AnimationState::new()
                .with_duration(std::time::Duration::from_millis(50))
                .with_keyframe((0.0, Units::Pixels(2.0)))
                .with_keyframe((1.0, Units::Pixels(20.0)))
                .set_persistent(true)
        );

        self.shrink = state.style.width.insert_animation(
            AnimationState::new()
                .with_duration(std::time::Duration::from_millis(50))
                .with_keyframe((0.0, Units::Pixels(20.0)))
                .with_keyframe((1.0, Units::Pixels(2.0)))
                .set_persistent(true)
        );
        
        entity
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::GeometryChanged(_) => {
                    let width = state.data.get_width(entity);
                    if width <= 150.0 {
                        if !self.shrink_flag {
                            self.shrink_flag = true;
                            state.style.width.play_animation(self.indicator_x, self.shrink);
                            state.style.width.play_animation(self.indicator_y, self.shrink);
                            state.style.width.play_animation(self.indicator_z, self.shrink);
                        }
                        
                        
                        

                        self.indicator_x.set_text(state, "");
                        self.indicator_y.set_text(state, "");
                        self.indicator_z.set_text(state, "");
                   
                   
                    } else {

                        if self.shrink_flag {
                            self.shrink_flag = false;
                            state.style.width.play_animation(self.indicator_x, self.grow);
                            state.style.width.play_animation(self.indicator_y, self.grow);
                            state.style.width.play_animation(self.indicator_z, self.grow);
                        }
                        

                        
                        
                        self.indicator_x.set_text(state, "x");
                        self.indicator_y.set_text(state, "y");
                        self.indicator_z.set_text(state, "z");

                        
                    }
                }

                _=> {}
            }
        }
    }
}