#![allow(dead_code)]

use crate::entity::Entity;
use crate::events::{BuildHandler, Event, EventHandler};
use crate::state::style::*;
use crate::WindowEvent;
use crate::{MouseButton, State};

use crate::widgets::Button;
use crate::AnimationState;

pub struct ScrollContainer {
    container: Entity,
    horizontal_scroll: Entity,
    vertical_scroll: Entity,
    scrolly: f32,

    pressedx: f32,
    pressedy: f32,
    moving: bool,
    position: f32,

    vertical_scroll_animation: usize,
    vertical_container_animation: usize,
}

impl ScrollContainer {
    pub fn new() -> Self {
        ScrollContainer {
            container: Entity::null(),
            horizontal_scroll: Entity::null(),
            vertical_scroll: Entity::null(),
            scrolly: 0.0,

            pressedx: 0.0,
            pressedy: 0.0,
            moving: false,
            position: 0.0,

            vertical_scroll_animation: std::usize::MAX,
            vertical_container_animation: std::usize::MAX,
        }
    }
}

impl BuildHandler for ScrollContainer {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        //entity.set_flex_direction(state, FlexDirection::Row).set_flex_grow(state, 1.0);

        self.container = Button::new().build(state, entity, |builder| {
            builder.set_top(Length::Percentage(0.0)).class("container")
        });

        state.style.clip_widget.insert(self.container, entity);

        //println!("Container: {}", self.container);

        self.vertical_scroll = Button::new().build(state, entity, |builder| {
            builder
                //.set_position(Position::Absolute)
                .set_top(Length::Percentage(0.0))
                //.set_width(Length::Pixels(10.0))
                .set_height(Length::Percentage(1.0))
                .set_right(Length::Pixels(0.0))
                .class("scrollbar")

            //.set_background_color(Color::rgb(70, 70, 70))
        });

        self.vertical_scroll.set_disabled(state, true);

        // self.vertical_scroll =
        //     Scrollbar::new(self.container, Direction::Vertical).build(state, entity, |builder| {
        //         builder
        //             .set_width(Length::Pixels(10.0))
        //             .set_height(Length::Percentage(1.0))
        //             .set_background_color(Color::rgb(50, 50, 100))
        //     });

        state.style.insert_element(entity, "scroll_container");

        let vertical_scroll_animation = AnimationState::new()
            .with_duration(std::time::Duration::from_millis(100))
            .with_keyframe((0.0, Length::Percentage(0.0)))
            .with_keyframe((1.0, Length::Percentage(0.2)));

        self.vertical_scroll_animation =
            state.style.top.insert_animation(vertical_scroll_animation);

        let vertical_container_animation = AnimationState::new()
            .with_duration(std::time::Duration::from_millis(100))
            .with_keyframe((0.0, Length::Percentage(0.0)))
            .with_keyframe((1.0, Length::Percentage(-0.2)));

        self.vertical_container_animation = state
            .style
            .top
            .insert_animation(vertical_container_animation);

        self.container
    }
}

impl EventHandler for ScrollContainer {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) -> bool {
        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::Relayout => {
                    // // To prevent recursive loop when layout event is triggered inside here
                    if event.origin != entity
                        && event.origin != self.container
                        && event.origin != self.vertical_scroll
                    {
                        let mut scrollh = state.transform.get_height(entity)
                            / state.transform.get_height(self.container);

                        if scrollh >= 1.0 {
                            scrollh = 1.0;
                            self.vertical_scroll.set_disabled(state, true);
                        }

                        if scrollh < 1.0 {
                            self.vertical_scroll.set_enabled(state, true);
                        }

                        // BUG: fast scrolling causes smaller scroll because the animation hasn't finished when this function is called again
                        // One way to fix this might be to check whether the value is currently being animated before setting here
                        // Possibly not the best solution but it works
                        if !state.style.top.is_animating(self.vertical_scroll) {
                            let dist = state.transform.get_posy(self.vertical_scroll)
                                - state.transform.get_posy(entity);
                            let space = state.transform.get_height(entity)
                                - (scrollh * state.transform.get_height(entity));
                            self.scrolly = dist / space;
                        }

                        if self.scrolly.is_nan() {
                            self.scrolly = 0.0;
                        }

                        if self.scrolly < 0.0 {
                            self.scrolly = 0.0;
                        }

                        if self.scrolly >= 1.0 {
                            self.scrolly = 1.0;
                        }

                        // self.vertical_scroll
                        //     .set_height(state, Length::Percentage(scrollh));

                        // Setting it this way avoid calling Restyle automatically
                        state
                            .style
                            .height
                            .insert(self.vertical_scroll, Length::Percentage(scrollh));

                        let overflow = 1.0
                            - (state.transform.get_height(self.container)
                                / state.transform.get_height(entity));
                        let overflow2 = 1.0
                            - (state.transform.get_height(entity)
                                / state.transform.get_height(self.container));

                        // self.container
                        //     .set_top(state, Length::Percentage(self.scrolly * overflow));
                        state
                            .style
                            .top
                            .insert(self.container, Length::Percentage(self.scrolly * overflow));

                        // self.vertical_scroll
                        //     .set_top(state, Length::Percentage(self.scrolly * overflow2));
                        state.style.top.insert(
                            self.vertical_scroll,
                            Length::Percentage(self.scrolly * overflow2),
                        );

                        // Relayout and Redraw wont get called automatically so need to manually trigger them
                        state.insert_event(Event::new(WindowEvent::Relayout).origin(entity));
                        //state.insert_event(Event::new(WindowEvent::Redraw));
                        //return true;
                    }
                }

                WindowEvent::MouseScroll(_, y) => {
                    println!("Mouse Scroll Event");
                    // Forward mouse scroll event to the scrollbar
                    // state.insert_event(
                    //     Event::new(WindowEvent::MouseScroll(*x, *y))
                    //         .target(self.vertical_scroll)
                    //         .propagate(Propagation::None),
                    // );

                    //if event.target == entity {

                    let overflow = state.transform.get_height(entity)
                        - state.transform.get_height(self.vertical_scroll);

                    if overflow == 0.0 {
                        return false;
                    }

                    // Need better names for these
                    let overflow = 1.0
                        - (state.transform.get_height(self.container)
                            / state.transform.get_height(entity));
                    let overflow2 = 1.0
                        - (state.transform.get_height(entity)
                            / state.transform.get_height(self.container));

                    self.scrolly += (40.0 * *y) / (state.transform.get_height(entity) * overflow);

                    if self.scrolly < 0.0 {
                        self.scrolly = 0.0;
                    }

                    if self.scrolly > 1.0 {
                        self.scrolly = 1.0;
                    }

                    //println!("Scroll: {}", self.scrolly);

                    // let mut scrollh = state.transform.get_height(entity) / state.transform.get_height(self.container);
                    // if scrollh > 1.0 {
                    //     scrollh = 1.0;
                    // }

                    let current_scroll_top = state
                        .style
                        .top
                        .get(self.vertical_scroll)
                        .cloned()
                        .unwrap_or_default();
                    let current_container_top = state
                        .style
                        .top
                        .get(self.container)
                        .cloned()
                        .unwrap_or_default();

                    self.container
                        .set_top(state, Length::Percentage(self.scrolly * overflow));
                    self.vertical_scroll
                        .set_top(state, Length::Percentage(self.scrolly * overflow2));

                    /*
                    if let Some(animation) = state.style.top.get_animation_mut(self.vertical_scroll_animation) {
                        *animation.keyframes.first_mut().unwrap() = (0.0, current_scroll_top);
                        *animation.keyframes.last_mut().unwrap() = (1.0, Length::Percentage(self.scrolly * overflow2));
                    }

                    state.style.top.play_animation(self.vertical_scroll, self.vertical_scroll_animation);





                    if let Some(animation) = state.style.top.get_animation_mut(self.vertical_container_animation) {
                        *animation.keyframes.first_mut().unwrap() = (0.0, current_container_top);
                        *animation.keyframes.last_mut().unwrap() = (1.0, Length::Percentage(self.scrolly * overflow));
                    }

                    state.style.top.play_animation(self.container, self.vertical_container_animation);
                    */

                    //println!("A: {:?}  B: {:?}", current_container_top, self.scrolly * overflow);

                    //state.insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
                    //state.insert_event(Event::new(WindowEvent::Redraw));
                    //}

                    // Capture the event to stop it triggering twice
                    return true;
                }

                WindowEvent::WindowResize(_, _) => {
                    // let scroll = state
                    //     .style
                    //     .scroll
                    //     .get(self.container)
                    //     .cloned()
                    //     .unwrap_or_default();

                    // event_manager.insert_event(
                    //     Event::new(StyleEvent::Restyle).target(state.root),
                    // );
                }

                WindowEvent::MouseDown(button) => match button {
                    MouseButton::Left => {
                        if state.hovered == self.vertical_scroll {
                            //println!("Clicked on scrollbar");
                            self.pressedx = state.mouse.cursorx;
                            self.pressedy = state.mouse.cursory;
                            self.moving = true;
                            // let scroll = state
                            //     .style
                            //     .scroll
                            //     .get(self.entity)
                            //     .cloned()
                            //     .unwrap_or_default();
                            //self.position = state.transform.get_posy(self.vertical_scroll);
                            self.position = self.scrolly;
                            state.capture(entity);
                        }
                    }
                    _ => {}
                },

                WindowEvent::MouseUp(button) => match button {
                    MouseButton::Left => {
                        self.moving = false;
                        println!("Scroll release");
                        state.release(entity);
                    }

                    _ => {}
                },

                WindowEvent::MouseMove(_, y) => {
                    if self.moving {
                        let dist_y = *y - self.pressedy;
                        let overflow = state.transform.get_height(entity)
                            - state.transform.get_height(self.vertical_scroll);

                        if overflow == 0.0 {
                            return false;
                        }

                        let ratio = dist_y / overflow;
                        let r = self.position + ratio;

                        // let mut scrollh = state.transform.get_height(entity) / state.transform.get_height(self.container);
                        // if scrollh > 1.0 {
                        //     scrollh = 1.0;
                        // }

                        self.scrolly = r;

                        if self.scrolly < 0.0 {
                            self.scrolly = 0.0;
                        }

                        if self.scrolly > 1.0 {
                            self.scrolly = 1.0;
                        }

                        // let scroll = state
                        //     .style
                        //     .scroll
                        //     .get(self.entity)
                        //     .cloned()
                        //     .unwrap_or_default();
                        //self.vertical_scroll
                        //    .set_top(state, Length::Pixels(self.position + dist_y));

                        let overflow = 1.0
                            - (state.transform.get_height(self.container)
                                / state.transform.get_height(entity));
                        let overflow2 = 1.0
                            - (state.transform.get_height(entity)
                                / state.transform.get_height(self.container));

                        self.container
                            .set_top(state, Length::Percentage(self.scrolly * overflow));
                        self.vertical_scroll
                            .set_top(state, Length::Percentage(self.scrolly * overflow2));

                        state.insert_event(Event::new(WindowEvent::Restyle));
                        state
                            .insert_event(Event::new(WindowEvent::Relayout).target(Entity::null()));
                        state.insert_event(Event::new(WindowEvent::Redraw));
                        //println!("overflow: {}, dist: {}, ratio: {}, scrolly: {}", overflow, dist_y, r, self.scrolly);
                    }
                }

                _ => {}
            }
        }
        false
    }
}
