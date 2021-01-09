#![allow(dead_code)]

use crate::{
    AnimationState, BuildHandler, Entity, Event, EventHandler, MouseButton, State, WindowEvent,
};

use crate::widgets::{Button, Element};

use crate::state::style::*;

const ICON_DOWN_OPEN_BIG: &str = "\u{e75c}";
const ICON_RIGHT_OPEN_BIG: &str = "\u{e75e}";

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PanelEvent {
    Open(Entity),
    Close(Entity),
}

pub struct Panel {
    header: Entity,
    container: Entity,
    other_container: Entity,
    checkbox: Entity,
    collapsed: bool,
    title: String,
    //num_items: u32,
    container_height: f32,

    expand_animation: usize,
    fade_in_animation: usize,

    collapse_animation: usize,
    fade_out_animation: usize,

    arrow_expand_animation: usize,
    arrow_collapse_animation: usize,
}

impl Panel {
    pub fn new(title: &str) -> Self {
        Panel {
            header: Entity::null(),
            container: Entity::null(),
            other_container: Entity::null(),
            checkbox: Entity::null(),
            title: title.to_string(),
            collapsed: false,
            //num_items,
            container_height: 0.0,

            expand_animation: std::usize::MAX,
            fade_in_animation: std::usize::MAX,
            collapse_animation: std::usize::MAX,
            fade_out_animation: std::usize::MAX,

            arrow_expand_animation: std::usize::MAX,
            arrow_collapse_animation: std::usize::MAX,
        }
    }
}

impl BuildHandler for Panel {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        self.header = Element::new().build(state, entity, |builder| {
            builder
                .set_flex_direction(FlexDirection::Row)
                //.set_height(Length::Pixels(30.0))
                .class("header")
        });

        self.checkbox = Element::new().build(state, self.header, |builder| {
            builder
                .set_text(ICON_DOWN_OPEN_BIG)
                .set_font("Icons".to_string())
                .set_text_justify(Justify::Center)
                .set_text_align(Align::Center)
                .set_width(Length::Pixels(20.0))
                .set_height(Length::Percentage(1.0))
                .set_hoverability(false)
                .class("arrow")
        });

        // self.checkbox = Checkbox::new(true, ICON_DOWN_OPEN_BIG, ICON_RIGHT_OPEN_BIG).build(state, self.header, |builder| {
        //     builder
        //         .set_width(Length::Pixels(20.0))
        //         .set_height(Length::Percentage(1.0))
        //         .set_hoverability(false)
        // });

        Element::new().build(state, self.header, |builder| {
            builder
                .set_text(&self.title)
                .set_flex_grow(1.0)
                .set_hoverability(false)
        });

        self.container = Element::new().build(state, entity, |builder| {
            builder
                // .set_position(Position::Absolute)
                // .set_top(Length::Percentage(1.0))
                // .set_width(Length::Percentage(1.0))
                //.set_height(Length::Pixels(200.0))
                .class("container")
        });

        self.other_container = Element::new().build(
            state,
            self.container,
            |builder| builder, //.set_flex_grow(1.0).class("test")
        );

        //entity.set_checked(state, true);

        state.style.insert_element(entity, "panel");

        let container_expand_animation = AnimationState::new()
            .with_duration(std::time::Duration::from_millis(100))
            .with_keyframe((0.0, Length::Pixels(0.0)))
            .with_keyframe((1.0, Length::Pixels(0.0)));

        self.expand_animation = state
            .style
            .height
            .insert_animation(container_expand_animation);

        let container_collapse_animation = AnimationState::new()
            .with_duration(std::time::Duration::from_millis(100))
            .with_delay(std::time::Duration::from_millis(150))
            .with_keyframe((0.0, Length::Pixels(0.0)))
            .with_keyframe((1.0, Length::Pixels(0.0)));

        self.collapse_animation = state
            .style
            .height
            .insert_animation(container_collapse_animation);

        let container_fade_in_animation = AnimationState::new()
            .with_duration(std::time::Duration::from_millis(100))
            .with_delay(std::time::Duration::from_millis(100))
            .with_keyframe((0.0, Opacity(0.0)))
            .with_keyframe((1.0, Opacity(1.0)));

        self.fade_in_animation = state
            .style
            .opacity
            .insert_animation(container_fade_in_animation);

        let container_fade_out_animation = AnimationState::new()
            .with_duration(std::time::Duration::from_millis(100))
            .with_keyframe((0.0, Opacity(1.0)))
            .with_keyframe((1.0, Opacity(0.0)));

        self.fade_out_animation = state
            .style
            .opacity
            .insert_animation(container_fade_out_animation);

        let arrow_expand_animation = AnimationState::new()
            .with_duration(std::time::Duration::from_millis(100))
            .with_keyframe((0.0, -90.0))
            .with_keyframe((1.0, 0.0));

        self.arrow_expand_animation = state.style.rotate.insert_animation(arrow_expand_animation);

        let arrow_collapse_animation = AnimationState::new()
            .with_duration(std::time::Duration::from_millis(100))
            .with_delay(std::time::Duration::from_millis(100))
            .with_keyframe((0.0, 0.0))
            .with_keyframe((1.0, -90.0));

        self.arrow_collapse_animation = state
            .style
            .rotate
            .insert_animation(arrow_collapse_animation);

        self.other_container
    }
}

impl EventHandler for Panel {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) -> bool {
        //if event.target == self.header {
        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::Relayout => {
                    // Exclude relayout orginating from animations
                    if event.origin != Entity::new(0, 0) {
                        if !state.style.height.is_animating(self.container) {
                            //let container_height = state.transform.get_height(self.container);
                            let container_height = state.transform.get_child_sum(self.container);
                            if container_height > 0.0 {
                                self.container_height = container_height;

                                if let Some(animation) =
                                    state.style.height.get_animation_mut(self.expand_animation)
                                {
                                    animation.keyframes.last_mut().unwrap().1 =
                                        Length::Pixels(self.container_height);
                                }

                                if let Some(animation) = state
                                    .style
                                    .height
                                    .get_animation_mut(self.collapse_animation)
                                {
                                    animation.keyframes.first_mut().unwrap().1 =
                                        Length::Pixels(self.container_height);
                                }

                                //println!("x: {}  y: {}  w: {}  h: {}", state.transform.get_posx(self.container), state.transform.get_posy(self.container), state.transform.get_width(self.container), state.transform.get_height(self.container));
                                //println!("display: {:?}  visibility: {:?}  opacity: {:?}", state.style.display.get(self.container).cloned().unwrap_or_default(), state.transform.get_visibility(self.container), state.transform.get_opacity(self.container));
                            }
                        }
                    }
                }

                WindowEvent::MouseUp(button) => {
                    if event.target == self.header && state.mouse.left.pressed == self.header {
                        if *button == MouseButton::Left {
                            if self.collapsed {
                                //self.container.set_visibility(state, Visibility::Visible);
                                //self.checkbox.set_text(state, ICON_DOWN_OPEN_BIG);
                                self.collapsed = false;
                                //state.style.checked.set(entity, true);
                                entity.set_checked(state, true);
                                state.insert_event(
                                    Event::new(PanelEvent::Open(entity)).target(entity),
                                );

                                state
                                    .style
                                    .height
                                    .play_animation(self.container, self.expand_animation);
                                state
                                    .style
                                    .opacity
                                    .play_animation(self.other_container, self.fade_in_animation);
                                state
                                    .style
                                    .rotate
                                    .play_animation(self.checkbox, self.arrow_expand_animation);

                                self.checkbox.set_rotate(state, 0.0);
                                self.container
                                    .set_height(state, Length::Pixels(self.container_height));
                                self.other_container.set_opacity(state, 1.0);

                            //self.container.set_display(state, Display::Flexbox);
                            } else {
                                //self.container.set_visibility(state, Visibility::Invisible);
                                //self.checkbox.set_text(state, ICON_RIGHT_OPEN_BIG);
                                self.collapsed = true;
                                //state.style.checked.set(entity, false);
                                entity.set_checked(state, false);
                                state.insert_event(
                                    Event::new(PanelEvent::Close(entity)).target(entity),
                                );

                                state
                                    .style
                                    .height
                                    .play_animation(self.container, self.collapse_animation);
                                state
                                    .style
                                    .opacity
                                    .play_animation(self.other_container, self.fade_out_animation);
                                state
                                    .style
                                    .rotate
                                    .play_animation(self.checkbox, self.arrow_collapse_animation);

                                self.checkbox.set_rotate(state, -90.0);
                                self.container.set_height(state, Length::Pixels(0.0));
                                self.other_container.set_opacity(state, 0.0);
                                //self.container.set_display(state, Display::None);
                            }
                        }

                        return true;
                    }
                }

                _ => {}
            }
        }
        //}

        false
    }
}
