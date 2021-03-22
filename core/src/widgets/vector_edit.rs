use crate::state::style::*;
use crate::widgets::*;
use crate::widgets::{Dropdown, DropdownEvent, Textbox, TextboxEvent};
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
            .set_text_justify(state, Justify::Center)
            .set_text_align(state, Align::Center);

        entity
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::MouseDown(button) => {
                    if *button == MouseButton::Left {
                        if entity == event.target {
                            self.pressed = true;
                        }
                    }
                }

                WindowEvent::MouseUp(button) => {
                    if *button == MouseButton::Left {
                        if self.pressed {
                            self.pressed = false;
                            //self.checkbox.set_checked(state, true);
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
    x: Entity,
    y: Entity,
    z: Entity,
    w: Entity,
    dims: Entity,

    // Animations
    reveal: usize,
    hide: usize,
    grow: usize,
    shrink: usize,

    xval: T,
    yval: T,
    zval: T,
    wval: T,
    num_of_dims: u8,
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
            x: Entity::null(),
            y: Entity::null(),
            z: Entity::null(),
            w: Entity::null(),
            dims: Entity::null(),

            reveal: std::usize::MAX,
            hide: std::usize::MAX,
            grow: std::usize::MAX,
            shrink: std::usize::MAX,

            xval: T::default(),
            yval: T::default(),
            zval: T::default(),
            wval: T::default(),

            num_of_dims: 4,
        }
    }

    pub fn with_x(mut self, val: T) -> Self {
        self.xval = val;

        self
    }

    pub fn with_y(mut self, val: T) -> Self {
        self.yval = val;

        self
    }

    pub fn with_z(mut self, val: T) -> Self {
        self.zval = val;

        self
    }

    pub fn with_w(mut self, val: T) -> Self {
        self.wval = val;

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
        state.add_theme(VEC_EDIT_STYLE);

        entity.set_flex_direction(state, FlexDirection::Row);

        self.x = Textbox::new(&self.xval.to_string())
            .build(state, entity, |builder| builder.set_flex_grow(1.0));
        self.y = Textbox::new(&self.yval.to_string()).build(state, entity, |builder| {
            builder
                .set_flex_grow(1.0)
                .set_margin_left(Length::Pixels(5.0))
        });
        self.z = Textbox::new(&self.zval.to_string()).build(state, entity, |builder| {
            builder
                .set_flex_grow(1.0)
                .set_margin_left(Length::Pixels(5.0))
        });
        self.w = Textbox::new(&self.wval.to_string()).build(state, entity, |builder| {
            builder
                .set_flex_grow(1.0)
                .set_margin_left(Length::Pixels(5.0))
        });

        self.dims = Dropdown::new("4")
            .build(state, entity, |builder| {
                builder
                    .set_flex_basis(Length::Pixels(30.0))
                    .set_text_justify(Justify::End)
                    .set_margin_left(Length::Pixels(5.0))
                    .class("dim")
            })
            .2;

        Dimension::new("1").build(state, self.dims, |builder| builder.class("item"));
        Dimension::new("2").build(state, self.dims, |builder| builder.class("item"));
        Dimension::new("3").build(state, self.dims, |builder| builder.class("item"));
        Dimension::new("4").build(state, self.dims, |builder| builder.class("item"));

        self.reveal = state.style.flex_grow.insert_animation(
            AnimationState::new()
                .with_duration(std::time::Duration::from_millis(100))
                .with_keyframe((0.0, 0.0))
                .with_keyframe((1.0, 1.0)),
        );

        self.grow = state.style.margin_left.insert_animation(
            AnimationState::new()
                .with_duration(std::time::Duration::from_millis(100))
                .with_keyframe((0.0, Length::Pixels(0.0)))
                .with_keyframe((1.0, Length::Pixels(5.0))),
        );

        self.shrink = state.style.margin_left.insert_animation(
            AnimationState::new()
                .with_duration(std::time::Duration::from_millis(100))
                .with_keyframe((0.0, Length::Pixels(5.0)))
                .with_keyframe((1.0, Length::Pixels(0.0))),
        );

        self.hide = state.style.flex_grow.insert_animation(
            AnimationState::new()
                .with_duration(std::time::Duration::from_millis(100))
                .with_keyframe((0.0, 1.0))
                .with_keyframe((1.0, 0.0)),
        );

        entity.set_element(state, "vector_edit");

        entity
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        let target = event.target;

        if let Some(dropdown_event) = event.message.downcast::<DropdownEvent>() {
            match dropdown_event {
                DropdownEvent::SetText(text) => match text.as_ref() {
                    "1" => {
                        if state.data.get_width(self.x) == 0.0 {
                            state.style.flex_grow.play_animation(self.x, self.reveal);
                        }

                        if state.data.get_width(self.y) != 0.0 {
                            state.style.flex_grow.play_animation(self.y, self.hide);
                            state.style.margin_left.play_animation(self.y, self.shrink);
                        }

                        if state.data.get_width(self.z) != 0.0 {
                            state.style.flex_grow.play_animation(self.z, self.hide);
                            state.style.margin_left.play_animation(self.z, self.shrink);
                        }

                        if state.data.get_width(self.w) != 0.0 {
                            state.style.flex_grow.play_animation(self.w, self.hide);
                            state.style.margin_left.play_animation(self.w, self.shrink);
                        }

                        self.x.set_flex_grow(state, 1.0);
                        self.y.set_flex_grow(state, 0.0);
                        self.z.set_flex_grow(state, 0.0);
                        self.w.set_flex_grow(state, 0.0);

                        self.y.set_margin_left(state, Length::Pixels(0.0));
                        self.z.set_margin_left(state, Length::Pixels(0.0));
                        self.w.set_margin_left(state, Length::Pixels(0.0));

                        self.num_of_dims = 1;

                        state.insert_event(
                            Event::new(VectorEditEvent::Dim1(self.xval)).target(entity),
                        );
                    }

                    "2" => {
                        if state.data.get_width(self.x) == 0.0 {
                            state.style.flex_grow.play_animation(self.x, self.reveal);
                        }

                        if state.data.get_width(self.y) == 0.0 {
                            state.style.flex_grow.play_animation(self.y, self.reveal);
                            state.style.margin_left.play_animation(self.y, self.grow);
                        }

                        if state.data.get_width(self.z) != 0.0 {
                            state.style.flex_grow.play_animation(self.z, self.hide);
                            state.style.margin_left.play_animation(self.z, self.shrink);
                        }

                        if state.data.get_width(self.w) != 0.0 {
                            state.style.flex_grow.play_animation(self.w, self.hide);
                            state.style.margin_left.play_animation(self.w, self.shrink);
                        }

                        self.x.set_flex_grow(state, 1.0);
                        self.y.set_flex_grow(state, 1.0);
                        self.z.set_flex_grow(state, 0.0);
                        self.w.set_flex_grow(state, 0.0);

                        self.y.set_margin_left(state, Length::Pixels(5.0));
                        self.z.set_margin_left(state, Length::Pixels(0.0));
                        self.w.set_margin_left(state, Length::Pixels(0.0));

                        self.num_of_dims = 2;

                        state.insert_event(
                            Event::new(VectorEditEvent::Dim2(self.xval, self.yval)).target(entity),
                        );
                    }

                    "3" => {
                        if state.data.get_width(self.x) == 0.0 {
                            state.style.flex_grow.play_animation(self.x, self.reveal);
                        }

                        if state.data.get_width(self.y) == 0.0 {
                            state.style.flex_grow.play_animation(self.y, self.reveal);
                            state.style.margin_left.play_animation(self.y, self.grow);
                        }

                        if state.data.get_width(self.z) == 0.0 {
                            state.style.flex_grow.play_animation(self.z, self.reveal);
                            state.style.margin_left.play_animation(self.z, self.grow);
                        }

                        if state.data.get_width(self.w) != 0.0 {
                            state.style.flex_grow.play_animation(self.w, self.hide);
                            state.style.margin_left.play_animation(self.w, self.shrink);
                        }

                        self.x.set_flex_grow(state, 1.0);
                        self.y.set_flex_grow(state, 1.0);
                        self.z.set_flex_grow(state, 1.0);
                        self.w.set_flex_grow(state, 0.0);

                        self.y.set_margin_left(state, Length::Pixels(5.0));
                        self.z.set_margin_left(state, Length::Pixels(5.0));
                        self.w.set_margin_left(state, Length::Pixels(0.0));

                        self.num_of_dims = 3;

                        state.insert_event(
                            Event::new(VectorEditEvent::Dim3(self.xval, self.yval, self.zval))
                                .target(entity),
                        );
                    }

                    "4" => {
                        if state.data.get_width(self.x) == 0.0 {
                            state.style.flex_grow.play_animation(self.x, self.reveal);
                        }

                        if state.data.get_width(self.y) == 0.0 {
                            state.style.flex_grow.play_animation(self.y, self.reveal);
                            state.style.margin_left.play_animation(self.y, self.grow);
                        }

                        if state.data.get_width(self.z) == 0.0 {
                            state.style.flex_grow.play_animation(self.z, self.reveal);
                            state.style.margin_left.play_animation(self.z, self.grow);
                        }

                        if state.data.get_width(self.w) == 0.0 {
                            state.style.flex_grow.play_animation(self.w, self.reveal);
                            state.style.margin_left.play_animation(self.w, self.grow);
                        }

                        self.x.set_flex_grow(state, 1.0);
                        self.y.set_flex_grow(state, 1.0);
                        self.z.set_flex_grow(state, 1.0);
                        self.w.set_flex_grow(state, 1.0);

                        self.y.set_margin_left(state, Length::Pixels(5.0));
                        self.z.set_margin_left(state, Length::Pixels(5.0));
                        self.w.set_margin_left(state, Length::Pixels(5.0));

                        self.num_of_dims = 4;

                        state.insert_event(
                            Event::new(VectorEditEvent::Dim4(
                                self.xval, self.yval, self.zval, self.wval,
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
                        if target == self.x {
                            self.xval = val;
                        }

                        if target == self.y {
                            self.yval = val;
                        }

                        if target == self.z {
                            self.zval = val;
                        }

                        if target == self.w {
                            self.wval = val;
                        }

                        match self.num_of_dims {
                            1 => state.insert_event(
                                Event::new(VectorEditEvent::Dim1(self.xval)).target(entity),
                            ),
                            2 => state.insert_event(
                                Event::new(VectorEditEvent::Dim2(self.xval, self.yval))
                                    .target(entity),
                            ),
                            3 => state.insert_event(
                                Event::new(VectorEditEvent::Dim3(self.xval, self.yval, self.zval))
                                    .target(entity),
                            ),
                            4 => state.insert_event(
                                Event::new(VectorEditEvent::Dim4(
                                    self.xval, self.yval, self.zval, self.wval,
                                ))
                                .target(entity),
                            ),
                            _ => {}
                        }

                        //state.insert_event(Event::new(VectorEditEvent::ValueChanged(self.xval, self.yval, self.zval, self.wval)).target(entity));
                    }
                }

                _ => {}
            }
        }
    }
}
