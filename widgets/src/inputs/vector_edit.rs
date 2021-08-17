use crate::common::*;
use crate::{Dropdown, DropdownEvent, Textbox, TextboxEvent};
use crate::AnimationState;

const VEC_EDIT_STYLE: &str = r#"
    vector_edit .icon {
        display: none;
    }

    vector_edit .dim {
        flex-grow: 0.0;
    }

    vector_edit .header>label {
        text-justify: center;
    }
"#;

#[derive(Debug, Clone, PartialEq)]
pub enum VectorEditEvent<T> {
    ValueChanged(T, T, T, T),
    Dim1(T),
    Dim2(T, T),
    Dim3(T, T, T),
    Dim4(T, T, T, T),
}

pub struct Dimension {
    text: String,
    pressed: bool,
}

impl Dimension {
    pub fn new(text: &str) -> Self {
        Dimension {
            text: text.to_string(),
            pressed: false,
        }
    }
}

impl Widget for Dimension {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
            .set_text(state, &self.text)
            .set_child_space(state, Stretch(1.0))
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::MouseDown(button) => {
                    if *button == MouseButton::Left {
                        if entity == event.target {
                            self.pressed = true;
                            // println!("Send Change Event");
                            // state.insert_event(
                            //     Event::new(DropdownEvent::SetText(self.text.clone()))
                            //         .target(entity)
                            //         .propagate(Propagation::Up),
                            // );
                        }
                    }
                }

                WindowEvent::MouseUp(button) => {
                    if *button == MouseButton::Left {
                        if self.pressed {
                            self.pressed = false;
                            //self.checkbox.set_checked(state, true);
                            // println!("Send Change Event");
                            state.insert_event(
                                Event::new(DropdownEvent::SetText(self.text.clone()))
                                    .target(entity)
                                    .propagate(Propagation::Up),
                            );
                        }
                    }
                }

                _ => {}
            }
        }
    }
}

pub struct VectorEdit<T> {
    // Subwidgets
    textbox_x: Entity,
    textbox_y: Entity,
    textbox_z: Entity,
    textbox_w: Entity,
    dims: Entity,

    // Animations
    reveal: Animation,
    hide: Animation,
    grow: Animation,
    shrink: Animation,

    // Data
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
    pub num_of_dims: u8,

    // Callbacks
    on_change: Option<Box<dyn Fn(&mut Self, &mut State, Entity)>>,
}

impl<T> VectorEdit<T>
where
    T: 'static
        + Default
        + std::fmt::Debug
        + std::fmt::Display
        + Copy
        + PartialEq
        + std::str::FromStr,
{
    pub fn new() -> Self {
        VectorEdit {
            textbox_x: Entity::null(),
            textbox_y: Entity::null(),
            textbox_z: Entity::null(),
            textbox_w: Entity::null(),
            dims: Entity::null(),

            reveal: Animation::default(),
            hide: Animation::default(),
            grow: Animation::default(),
            shrink: Animation::default(),

            x: T::default(),
            y: T::default(),
            z: T::default(),
            w: T::default(),
            num_of_dims: 4,

            on_change: None,
        }
    }

    pub fn with_x(mut self, val: T) -> Self {
        self.x = val;

        self
    }

    pub fn with_y(mut self, val: T) -> Self {
        self.y = val;

        self
    }

    pub fn with_z(mut self, val: T) -> Self {
        self.z = val;

        self
    }

    pub fn with_w(mut self, val: T) -> Self {
        self.w = val;

        self
    }

    pub fn on_change<F>(mut self, message: F) -> Self
    where
        F: 'static + Fn(&mut Self, &mut State, Entity),
    {
        self.on_change = Some(Box::new(message));

        self
    }
}

impl<T> Widget for VectorEdit<T>
where
    T: 'static
        + Default
        + std::fmt::Debug
        + std::fmt::Display
        + Copy
        + PartialEq
        + std::str::FromStr,
{
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        //state.add_theme(VEC_EDIT_STYLE);

        entity.set_layout_type(state, LayoutType::Row);

        self.textbox_x = Textbox::new(&self.x.to_string())
            .build(state, entity, |builder| 
                builder.set_right(Pixels(5.0))
        );
        self.textbox_y = Textbox::new(&self.y.to_string()).build(state, entity, |builder| {
            builder.set_right(Pixels(5.0))
        });
        self.textbox_z = Textbox::new(&self.z.to_string()).build(state, entity, |builder| {
            builder.set_right(Pixels(5.0))
        });
        self.textbox_w = Textbox::new(&self.w.to_string()).build(state, entity, |builder| {
            builder.set_right(Pixels(5.0))
        });

        self.dims = Dropdown::new("4")
            .build(state, entity, |builder| {
                builder
                    .set_width(Pixels(30.0))
                    //.set_text_justify(Justify::End)
                    .class("dim")
            })
            .2;

        Dimension::new("1").build(state, self.dims, |builder| builder.class("item"));
        Dimension::new("2").build(state, self.dims, |builder| builder.class("item"));
        Dimension::new("3").build(state, self.dims, |builder| builder.class("item"));
        Dimension::new("4").build(state, self.dims, |builder| builder.class("item"));

        self.reveal = state.style.width.insert_animation(
            AnimationState::new()
                .with_duration(std::time::Duration::from_millis(100))
                .with_keyframe((0.0, Stretch(0.0)))
                .with_keyframe((1.0, Stretch(1.0))),
        );

        self.grow = state.style.right.insert_animation(
            AnimationState::new()
                .with_duration(std::time::Duration::from_millis(100))
                .with_keyframe((0.0, Units::Pixels(0.0)))
                .with_keyframe((1.0, Units::Pixels(5.0))),
        );

        self.shrink = state.style.right.insert_animation(
            AnimationState::new()
                .with_duration(std::time::Duration::from_millis(100))
                .with_keyframe((0.0, Units::Pixels(5.0)))
                .with_keyframe((1.0, Units::Pixels(0.0))),
        );

        self.hide = state.style.width.insert_animation(
            AnimationState::new()
                .with_duration(std::time::Duration::from_millis(100))
                .with_keyframe((0.0, Stretch(1.0)))
                .with_keyframe((1.0, Stretch(0.0))),
        );

        if let Some(callback) = self.on_change.take() {
            (callback)(self, state, entity);
            self.on_change = Some(callback);
        }

        entity.set_element(state, "vector_edit");

        entity
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        let target = event.target;

        if let Some(dropdown_event) = event.message.downcast::<DropdownEvent>() {
            match dropdown_event {
                DropdownEvent::SetText(text) => match text.as_ref() {
                    "1" => {
                        if state.data.get_width(self.textbox_x) == 0.0 {
                            state.style.width.play_animation(self.textbox_x, self.reveal);
                            state.style.right.play_animation(self.textbox_x, self.grow);
                        }

                        if state.data.get_width(self.textbox_y) != 0.0 {
                            state.style.width.play_animation(self.textbox_y, self.hide);
                            state.style.right.play_animation(self.textbox_y, self.shrink);
                        }

                        if state.data.get_width(self.textbox_z) != 0.0 {
                            state.style.width.play_animation(self.textbox_z, self.hide);
                            state.style.right.play_animation(self.textbox_z, self.shrink);
                        }

                        if state.data.get_width(self.textbox_w) != 0.0 {
                            state.style.width.play_animation(self.textbox_w, self.hide);
                            state.style.right.play_animation(self.textbox_w, self.shrink);
                        }

                        self.textbox_x.set_width(state, Stretch(1.0));
                        self.textbox_y.set_width(state, Stretch(0.0));
                        self.textbox_z.set_width(state, Stretch(0.0));
                        self.textbox_w.set_width(state, Stretch(0.0));

                        self.textbox_x.set_right(state, Pixels(5.0));
                        self.textbox_y.set_right(state, Pixels(0.0));
                        self.textbox_z.set_right(state, Pixels(0.0));
                        self.textbox_w.set_right(state, Pixels(0.0));

                        self.num_of_dims = 1;

                        state.insert_event(
                            Event::new(VectorEditEvent::Dim1(self.x)).target(entity),
                        );
                    }

                    "2" => {
                        if state.data.get_width(self.textbox_x) == 0.0 {
                            state.style.width.play_animation(self.textbox_x, self.reveal);
                            state.style.right.play_animation(self.textbox_x, self.grow);
                        }

                        if state.data.get_width(self.textbox_y) == 0.0 {
                            state.style.width.play_animation(self.textbox_y, self.reveal);
                            state.style.right.play_animation(self.textbox_y, self.grow);
                        }

                        if state.data.get_width(self.textbox_z) != 0.0 {
                            state.style.width.play_animation(self.textbox_z, self.hide);
                            state.style.right.play_animation(self.textbox_z, self.shrink);
                        }

                        if state.data.get_width(self.textbox_w) != 0.0 {
                            state.style.width.play_animation(self.textbox_w, self.hide);
                            state.style.right.play_animation(self.textbox_w, self.shrink);
                        }

                        self.textbox_x.set_width(state, Stretch(1.0));
                        self.textbox_y.set_width(state, Stretch(1.0));
                        self.textbox_z.set_width(state, Stretch(0.0));
                        self.textbox_w.set_width(state, Stretch(0.0));

                        self.textbox_x.set_right(state, Units::Pixels(5.0));
                        self.textbox_y.set_right(state, Units::Pixels(5.0));
                        self.textbox_z.set_right(state, Units::Pixels(0.0));
                        self.textbox_w.set_right(state, Units::Pixels(0.0));

                        self.num_of_dims = 2;

                        state.insert_event(
                            Event::new(VectorEditEvent::Dim2(self.x, self.y)).target(entity),
                        );
                    }

                    "3" => {
                        if state.data.get_width(self.textbox_x) == 0.0 {
                            state.style.width.play_animation(self.textbox_x, self.reveal);
                            state.style.right.play_animation(self.textbox_x, self.grow);
                        }

                        if state.data.get_width(self.textbox_y) == 0.0 {
                            state.style.width.play_animation(self.textbox_y, self.reveal);
                            state.style.right.play_animation(self.textbox_y, self.grow);
                        }

                        if state.data.get_width(self.textbox_z) == 0.0 {
                            state.style.width.play_animation(self.textbox_z, self.reveal);
                            state.style.right.play_animation(self.textbox_z, self.grow);
                        }

                        if state.data.get_width(self.textbox_w) != 0.0 {
                            state.style.width.play_animation(self.textbox_w, self.hide);
                            state.style.right.play_animation(self.textbox_w, self.shrink);
                        }

                        self.textbox_x.set_width(state, Stretch(1.0));
                        self.textbox_y.set_width(state, Stretch(1.0));
                        self.textbox_z.set_width(state, Stretch(1.0));
                        self.textbox_w.set_width(state, Stretch(0.0));

                        self.textbox_x.set_right(state, Units::Pixels(5.0));
                        self.textbox_y.set_right(state, Units::Pixels(5.0));
                        self.textbox_z.set_right(state, Units::Pixels(5.0));
                        self.textbox_w.set_right(state, Units::Pixels(0.0));

                        self.num_of_dims = 3;

                        state.insert_event(
                            Event::new(VectorEditEvent::Dim3(self.x, self.y, self.z))
                                .target(entity),
                        );
                    }

                    "4" => {
                        if state.data.get_width(self.textbox_x) == 0.0 {
                            state.style.width.play_animation(self.textbox_x, self.reveal);
                            state.style.right.play_animation(self.textbox_x, self.grow);
                        }

                        if state.data.get_width(self.textbox_y) == 0.0 {
                            state.style.width.play_animation(self.textbox_y, self.reveal);
                            state.style.right.play_animation(self.textbox_y, self.grow);
                        }

                        if state.data.get_width(self.textbox_z) == 0.0 {
                            state.style.width.play_animation(self.textbox_z, self.reveal);
                            state.style.right.play_animation(self.textbox_z, self.grow);
                        }

                        if state.data.get_width(self.textbox_w) == 0.0 {
                            state.style.width.play_animation(self.textbox_w, self.reveal);
                            state.style.right.play_animation(self.textbox_w, self.grow);
                        }

                        self.textbox_x.set_width(state, Stretch(1.0));
                        self.textbox_y.set_width(state, Stretch(1.0));
                        self.textbox_z.set_width(state, Stretch(1.0));
                        self.textbox_w.set_width(state, Stretch(1.0));

                        self.textbox_x.set_right(state, Units::Pixels(5.0));
                        self.textbox_y.set_right(state, Units::Pixels(5.0));
                        self.textbox_z.set_right(state, Units::Pixels(5.0));
                        self.textbox_w.set_right(state, Units::Pixels(5.0));

                        self.num_of_dims = 4;

                        state.insert_event(
                            Event::new(VectorEditEvent::Dim4(
                                self.x, self.y, self.z, self.w,
                            ))
                            .target(entity),
                        );
                    }

                    _ => {}
                },
            }
        }

        if let Some(textbox_event) = event.message.downcast::<TextboxEvent>() {
            match textbox_event {
                TextboxEvent::ValueChanged(text) => {
                    if let Ok(val) = text.clone().parse::<T>() {
                        if target == self.textbox_x {
                            self.x = val;
                        }

                        if target == self.textbox_y {
                            self.y = val;
                        }

                        if target == self.textbox_z {
                            self.z = val;
                        }

                        if target == self.textbox_w {
                            self.w = val;
                        }

                        match self.num_of_dims {
                            1 => state.insert_event(
                                Event::new(VectorEditEvent::Dim1(self.x)).target(entity),
                            ),
                            2 => state.insert_event(
                                Event::new(VectorEditEvent::Dim2(self.x, self.y))
                                    .target(entity),
                            ),
                            3 => state.insert_event(
                                Event::new(VectorEditEvent::Dim3(self.x, self.y, self.z))
                                    .target(entity),
                            ),
                            4 => state.insert_event(
                                Event::new(VectorEditEvent::Dim4(
                                    self.x, self.y, self.z, self.w,
                                ))
                                .target(entity),
                            ),
                            _ => {}
                        }

                        if let Some(callback) = self.on_change.take() {
                            (callback)(self, state, entity);
                            self.on_change = Some(callback);
                        }

                        //state.insert_event(Event::new(VectorEditEvent::ValueChanged(self.xval, self.yval, self.zval, self.wval)).target(entity));
                    }
                }

                _ => {}
            }
        }
    }
}
