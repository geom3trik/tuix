#![allow(dead_code)]

use crate::style::*;
use crate::widgets::*;

const ICON_DOWN_OPEN_BIG: &str = "\u{e75c}";
const ICON_RIGHT_OPEN_BIG: &str = "\u{e75e}";

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PanelEvent {
    Open,
    Close,
}

pub struct Panel {
    header: Entity,
    container1: Entity,
    container2: Entity,
    arrow: Entity,
    collapsed: bool,
    title: String,

    container_height: f32,
    container_width: f32,

    expand_height_animation: Animation,
    collapse_height_animation: Animation,
    expand_width_animation: Animation,
    collapse_width_animation: Animation,

    fade_in_animation: Animation,
    fade_out_animation: Animation,

    move_up_animation: Animation,
    move_down_animation: Animation,
    move_left_animation: Animation,
    move_right_animation: Animation,

    arrow_cw_animation: Animation,
    arrow_ccw_animation: Animation,
}

impl Panel {
    pub fn new(title: &str) -> Self {
        Panel {
            header: Entity::default(),
            container1: Entity::default(),
            container2: Entity::default(),
            arrow: Entity::default(),
            title: title.to_string(),
            collapsed: false,

            container_height: 0.0,
            container_width: 0.0,

            expand_height_animation: Animation::default(),
            collapse_height_animation: Animation::default(),
            expand_width_animation: Animation::default(),
            collapse_width_animation: Animation::default(),

            fade_in_animation: Animation::default(),
            fade_out_animation: Animation::default(),

            move_up_animation: Animation::default(),
            move_down_animation: Animation::default(),
            move_left_animation: Animation::default(),
            move_right_animation: Animation::default(),

            arrow_cw_animation: Animation::default(),
            arrow_ccw_animation: Animation::default(),
        }
    }
}

impl Widget for Panel {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
            .set_focusability(state, false)
            //.set_width(state, Auto);
            .set_height(state, Auto);

        self.header = Button::new()
            .on_release(move |_,state,_|
                state.insert_event(
                    Event::new(PanelEvent::Open).target(entity)
                )
            )
            .build(state, entity, |builder| {
                builder
                    .set_layout_type(LayoutType::Row)
                    .set_child_left(Pixels(5.0))
                    //.set_flex_direction(FlexDirection::Row)
                    .class("header")
            });

        self.arrow = Element::new().build(state, self.header, |builder| {
            builder
                .set_text(ICON_DOWN_OPEN_BIG)
                .set_font("icons")
                .set_child_space(Stretch(1.0))
                .set_top(Stretch(1.0))
                .set_bottom(Stretch(1.0))
                .set_width(Pixels(20.0))
                .set_height(Pixels(20.0))
                .set_hoverability(false)
                .set_focusability(false)
                //.set_background_color(Color::rgb(100, 100, 100))
                .class("icon")
        });

        // Label
        Label::new(&self.title).build(state, self.header, |builder| {
            builder
                .set_width(Stretch(1.0))
                .set_height(Stretch(1.0))
                .set_left(Pixels(10.0))
                .set_child_top(Stretch(1.0))
                .set_child_bottom(Stretch(1.0))
                .set_hoverability(false)
                .set_focusability(false)
                .class("label")
        });

        self.container1 = Element::new().build(state, entity, |builder| {
            builder
                .class("container1")
                .set_focusability(false)
                .set_width(Stretch(1.0))
                .set_height(Auto)
                .set_min_height(Pixels(0.0))
        });

        self.container2 = Element::new().build(state, self.container1, |builder| {
            builder
                .class("container2")
                .set_focusability(false)
                .set_clip_widget(self.container1)
                //.set_child_left(Stretch(1.0))
                //.set_child_right(Stretch(1.0))
                .set_width(Stretch(1.0))
                .set_height(Auto)
        });

        entity.set_element(state, "panel");

        // Animations
        let container_expand_animation = AnimationState::new()
            .with_duration(std::time::Duration::from_millis(100))
            .with_keyframe((0.0, Units::Pixels(0.0)))
            .with_keyframe((1.0, Units::Pixels(0.0)));

        self.expand_height_animation = state
            .style
            .height
            .insert_animation(container_expand_animation.clone());

        self.expand_width_animation = state
            .style
            .width
            .insert_animation(container_expand_animation.clone());

        let container_collapse_animation = AnimationState::new()
            .with_duration(std::time::Duration::from_millis(100))
            .with_keyframe((0.0, Units::Pixels(0.0)))
            .with_keyframe((1.0, Units::Pixels(0.0)));

        self.collapse_height_animation = state
            .style
            .height
            .insert_animation(container_collapse_animation.clone());

        self.collapse_width_animation = state
            .style
            .width
            .insert_animation(container_collapse_animation.clone());

        let container_fade_in_animation = AnimationState::new()
            .with_duration(std::time::Duration::from_millis(1))
            .with_keyframe((0.0, Opacity(0.0)))
            .with_keyframe((1.0, Opacity(1.0)));

        let container_hide_animation = AnimationState::new()
            .with_duration(std::time::Duration::from_millis(100))
            .with_keyframe((0.0, Units::Pixels(0.0)))
            .with_keyframe((1.0, Units::Pixels(0.0)));

        self.move_up_animation = state
            .style
            .top
            .insert_animation(container_hide_animation.clone());

        self.move_left_animation = state
            .style
            .left
            .insert_animation(container_hide_animation.clone());

        let container_reveal_animation = AnimationState::new()
            .with_duration(std::time::Duration::from_millis(100))
            .with_keyframe((0.0, Units::Pixels(0.0)))
            .with_keyframe((1.0, Units::Pixels(0.0)));

        self.move_down_animation = state
            .style
            .top
            .insert_animation(container_reveal_animation.clone());

        self.move_right_animation = state
            .style
            .left
            .insert_animation(container_reveal_animation.clone());

        self.fade_in_animation = state
            .style
            .opacity
            .insert_animation(container_fade_in_animation);

        let container_fade_out_animation = AnimationState::new()
            .with_duration(std::time::Duration::from_millis(100))
            .with_delay(std::time::Duration::from_millis(100))
            .with_keyframe((0.0, Opacity(1.0)))
            .with_keyframe((1.0, Opacity(0.0)));

        self.fade_out_animation = state
            .style
            .opacity
            .insert_animation(container_fade_out_animation);

        let arrow_cw_animation = AnimationState::new()
            .with_duration(std::time::Duration::from_millis(100))
            .with_keyframe((0.0, -90.0))
            .with_keyframe((1.0, 0.0));

        self.arrow_cw_animation = state.style.rotate.insert_animation(arrow_cw_animation);

        let arrow_ccw_animation = AnimationState::new()
            .with_duration(std::time::Duration::from_millis(100))
            .with_keyframe((0.0, 0.0))
            .with_keyframe((1.0, -90.0));

        self.arrow_ccw_animation = state.style.rotate.insert_animation(arrow_ccw_animation);

        self.container2
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(panel_event) = event.message.downcast::<PanelEvent>() {
            match panel_event {
                PanelEvent::Open | PanelEvent::Close => {
                    if event.target == entity {
                        if self.collapsed {
                            self.collapsed = false;

                            entity.set_checked(state, true);

                            match entity.get_layout_type(state) {
                                LayoutType::Column => {
                                    state.style.height.play_animation(
                                        self.container1,
                                        self.expand_height_animation,
                                    );

                                    self.container1.set_height(state, Units::Auto);

                                    state
                                        .style
                                        .rotate
                                        .play_animation(self.arrow, self.arrow_cw_animation);
                                    
                                    self.arrow.set_rotate(state, 0.0);
                                }

                                LayoutType::Row => {
                                    state.style.width.play_animation(
                                        self.container1,
                                        self.expand_width_animation,
                                    );

                                    self.container1.set_width(state, Units::Auto);

                                    state
                                        .style
                                        .rotate
                                        .play_animation(self.arrow, self.arrow_ccw_animation);

                                    self.arrow.set_rotate(state, -90.0);
                                }

                                _ => {}
                            }

                            state
                                .style
                                .top
                                .play_animation(self.container2, self.move_down_animation);

                            self.container2.set_opacity(state, 1.0);
                        } else {
                            self.collapsed = true;

                            entity.set_checked(state, false);

                            match entity.get_layout_type(state) {
                                LayoutType::Column => {
                                    if !state.style.height.is_animating(self.container1) {
                                        let container_height =
                                            state.data.get_height(self.container1);
                                        //println!("Container Height: {} {}", self.container1, container_height);

                                        if container_height != self.container_height {
                                            //self.container_height = container_height;

                                            if let Some(animation) = state
                                                .style
                                                .height
                                                .get_animation_mut(self.expand_height_animation)
                                            {
                                                animation.keyframes.last_mut().unwrap().1 =
                                                    Units::Pixels(container_height);
                                            }

                                            if let Some(animation) = state
                                                .style
                                                .height
                                                .get_animation_mut(self.collapse_height_animation)
                                            {
                                                animation.keyframes.first_mut().unwrap().1 =
                                                    Units::Pixels(container_height);
                                            }

                                            if let Some(animation) = state
                                                .style
                                                .top
                                                .get_animation_mut(self.move_down_animation)
                                            {
                                                animation.keyframes.first_mut().unwrap().1 =
                                                    Units::Pixels(-container_height);
                                            }

                                            if let Some(animation) = state
                                                .style
                                                .top
                                                .get_animation_mut(self.move_up_animation)
                                            {
                                                animation.keyframes.last_mut().unwrap().1 =
                                                    Units::Pixels(-container_height);
                                            }

                                            self.container_height = container_height;
                                        }
                                    }

                                    state.style.height.play_animation(
                                        self.container1,
                                        self.collapse_height_animation,
                                    );

                                    self.container1.set_height(state, Units::Pixels(0.0));

                                    state
                                        .style
                                        .rotate
                                        .play_animation(self.arrow, self.arrow_ccw_animation);

                                    self.arrow.set_rotate(state, -90.0);
                                }

                                LayoutType::Row => {
                                    if !state.style.height.is_animating(self.container1) {
                                        let container_width = state.data.get_width(self.container1);

                                        if container_width != self.container_width {
                                            //self.container_height = container_height;

                                            if let Some(animation) = state
                                                .style
                                                .width
                                                .get_animation_mut(self.expand_width_animation)
                                            {
                                                animation.keyframes.last_mut().unwrap().1 =
                                                    Units::Pixels(container_width);
                                            }

                                            if let Some(animation) = state
                                                .style
                                                .width
                                                .get_animation_mut(self.collapse_width_animation)
                                            {
                                                animation.keyframes.first_mut().unwrap().1 =
                                                    Units::Pixels(container_width);
                                            }

                                            if let Some(animation) = state
                                                .style
                                                .left
                                                .get_animation_mut(self.move_left_animation)
                                            {
                                                animation.keyframes.first_mut().unwrap().1 =
                                                    Units::Pixels(-container_width);
                                            }

                                            if let Some(animation) = state
                                                .style
                                                .left
                                                .get_animation_mut(self.move_right_animation)
                                            {
                                                animation.keyframes.last_mut().unwrap().1 =
                                                    Units::Pixels(-container_width);
                                            }

                                            self.container_height = container_width;
                                        }
                                    }

                                    state.style.width.play_animation(
                                        self.container1,
                                        self.collapse_width_animation,
                                    );

                                    self.container1.set_width(state, Units::Pixels(0.0));

                                    state
                                        .style
                                        .rotate
                                        .play_animation(self.arrow, self.arrow_cw_animation);

                                    self.arrow.set_rotate(state, 0.0);
                                }

                                _ => {}
                            }

                            state
                                .style
                                .opacity
                                .play_animation(self.container2, self.fade_out_animation);

                            state
                                .style
                                .top
                                .play_animation(self.container2, self.move_up_animation);

                            self.container2.set_opacity(state, 0.0);
                        }
                    }
                }
            }
        }

        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::GeometryChanged(_) => {
                    if event.target == self.container1 {
                        match entity.get_layout_type(state) {
                            LayoutType::Row => {
                                self.arrow.set_rotate(state, -90.0);
                            }

                            _ => {}
                        }

                        event.consume();
                    }
                }
                _ => {}
            }
        }
    }
}
